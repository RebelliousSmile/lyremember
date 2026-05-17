<template>
  <div class="space-y-6">
    <!-- Progress -->
    <div class="flex items-center gap-4">
      <div class="flex-1 bg-deep-card-hover rounded-full h-2">
        <div
          class="bg-orange-500 dark:bg-orange-400 h-2 rounded-full transition-all duration-500"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <span class="text-sm text-[#8A82A0]">
        {{ currentIndex + 1 }} / {{ song.lyrics.length }}
      </span>
    </div>

    <div v-if="!finished" class="space-y-6">
      <!-- Translation prompt -->
      <div class="p-6 bg-deep-card rounded-xl text-center space-y-2">
        <p class="text-sm text-[#8A82A0] uppercase tracking-wide">
          Try to say this in {{ song.language.toUpperCase() }}:
        </p>
        <p
          v-if="song.translations?.en?.[currentIndex]"
          class="text-xl font-semibold text-[#F5F0EB]"
        >
          {{ song.translations.en[currentIndex] }}
        </p>
        <p v-else class="text-lg text-[#B8B0D0] italic">Read line {{ currentIndex + 1 }} aloud</p>
      </div>

      <!-- Reveal stages -->
      <div class="p-6 bg-deep rounded-xl border border-deep-border space-y-4">
        <!-- Stage 0: Hidden -->
        <div v-if="revealStage === 0" class="text-center py-4">
          <p class="text-[#8A82A0] text-lg">Try saying the line first, then reveal hints below</p>
        </div>

        <!-- Stage 1: Phonetic -->
        <div v-if="revealStage >= 1" class="space-y-1">
          <p class="text-xs uppercase tracking-wide text-[#8A82A0]">Phonetic</p>
          <p class="text-xl italic text-gold">
            {{ song.phonetic_lyrics?.[currentIndex] ?? 'N/A' }}
          </p>
        </div>

        <!-- Stage 2: First characters hint -->
        <div v-if="revealStage >= 2" class="space-y-1">
          <p class="text-xs uppercase tracking-wide text-[#8A82A0]">First characters</p>
          <p class="text-xl text-[#B8B0D0] font-mono">
            {{ firstCharsHint }}
          </p>
        </div>

        <!-- Stage 3: Full original -->
        <div v-if="revealStage >= 3" class="space-y-1">
          <p class="text-xs uppercase tracking-wide text-[#8A82A0]">Original</p>
          <p class="text-xl font-semibold text-[#F5F0EB]">
            {{ song.lyrics[currentIndex] }}
          </p>
        </div>
      </div>

      <!-- Live speech (experimental, only if browser supports it) -->
      <div
        v-if="liveSupported && !selfAssessed"
        class="p-4 rounded-xl border border-deep-border bg-deep-card/50 space-y-2"
      >
        <div class="flex items-center justify-between gap-3">
          <p class="text-xs uppercase tracking-wide text-[#8A82A0]">Live (experimental)</p>
          <Button
            :variant="listening ? 'danger' : 'secondary'"
            size="sm"
            @click="listening ? stopListening() : startListening()"
          >
            <Mic :size="16" />
            {{ listening ? 'Stop' : 'Speak' }}
          </Button>
        </div>
        <p v-if="lastTranscript" class="text-sm text-[#B8B0D0]">
          You said: <span class="italic">"{{ lastTranscript }}"</span>
        </p>
        <p
          v-if="lastScore !== null"
          class="text-sm font-semibold"
          :class="lastScore >= 0.7 ? 'text-green-500' : 'text-orange-400'"
        >
          Match: {{ Math.round(lastScore * 100) }}%
        </p>
      </div>

      <!-- Self assessment -->
      <div v-if="revealStage >= 3 && !selfAssessed" class="space-y-2">
        <p class="text-sm text-[#8A82A0] text-center">How did you do?</p>
        <div class="flex justify-center gap-3">
          <Button variant="danger" size="sm" @click="assess(false)">
            <X :size="16" />
            Didn't know
          </Button>
          <Button variant="secondary" size="sm" @click="assess(true)">
            <Minus :size="16" />
            Partially
          </Button>
          <Button variant="primary" size="sm" @click="assess(true)">
            <Check :size="16" />
            Got it!
          </Button>
        </div>
      </div>

      <!-- Controls -->
      <div class="flex items-center justify-between">
        <Button v-if="revealStage < 3" variant="secondary" @click="revealStage++">
          <Eye :size="18" />
          Reveal {{ ['phonetic', 'hint', 'answer'][revealStage] }}
        </Button>
        <div v-else></div>
        <Button variant="primary" :disabled="revealStage < 3 || !selfAssessed" @click="nextLine">
          Next
          <ChevronRight :size="18" />
        </Button>
      </div>
    </div>

    <!-- End screen -->
    <div v-else class="text-center py-6 space-y-4">
      <Mic :size="48" class="mx-auto text-orange-500" />
      <p class="text-xl font-semibold text-[#F5F0EB]">Practice Complete!</p>
      <p class="text-[#8A82A0]">
        {{ correctCount }} / {{ song.lyrics.length }} lines self-assessed as correct
      </p>
      <div class="flex justify-center gap-3">
        <Button variant="primary" @click="restart">
          <RotateCcw :size="18" />
          Retry
        </Button>
        <Button variant="secondary" @click="$emit('finish', sessionData)"> Done </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import { Eye, ChevronRight, Check, X, Minus, Mic, RotateCcw } from 'lucide-vue-next';
import Button from '../ui/Button.vue';
import type { Song } from '../../types';
import { hasSpeechRecognition, scoreSpoken } from '../../lib/oral-scoring';

const props = defineProps<{ song: Song }>();
defineEmits<{
  finish: [
    data: { score: number; linesPracticed: number; linesCorrect: number; durationSeconds: number },
  ];
}>();

const currentIndex = ref(0);
const revealStage = ref(0);
const selfAssessed = ref(false);
const correctCount = ref(0);
const finished = ref(false);
let startTime = Date.now();

// ---- Live speech recognition (experimental, opt-in) ----
const liveSupported = hasSpeechRecognition();
const listening = ref(false);
const lastTranscript = ref('');
const lastScore = ref<number | null>(null);
const SCORE_PASS_THRESHOLD = 0.7;

interface MinimalSpeechRecognition {
  lang: string;
  interimResults: boolean;
  continuous: boolean;
  start: () => void;
  stop: () => void;
  onresult: ((e: { results: ArrayLike<ArrayLike<{ transcript: string }>> }) => void) | null;
  onend: (() => void) | null;
  onerror: ((e: unknown) => void) | null;
}

let recognition: MinimalSpeechRecognition | null = null;

function langToLocale(lang: string): string {
  return { fr: 'fr-FR', en: 'en-US', jp: 'ja-JP', kr: 'ko-KR' }[lang] ?? 'en-US';
}

function startListening() {
  if (!liveSupported) return;
  const w = window as unknown as {
    SpeechRecognition?: new () => MinimalSpeechRecognition;
    webkitSpeechRecognition?: new () => MinimalSpeechRecognition;
  };
  const Ctor = w.SpeechRecognition ?? w.webkitSpeechRecognition;
  if (!Ctor) return;
  recognition = new Ctor();
  recognition.lang = langToLocale(props.song.language);
  recognition.interimResults = false;
  recognition.continuous = false;
  recognition.onresult = (e) => {
    const transcript = e.results[0]?.[0]?.transcript ?? '';
    lastTranscript.value = transcript;
    const expected = props.song.lyrics[currentIndex.value] ?? '';
    const score = scoreSpoken(expected, transcript);
    lastScore.value = score;
    if (score >= SCORE_PASS_THRESHOLD && !selfAssessed.value) {
      assess(true);
    }
  };
  recognition.onend = () => {
    listening.value = false;
  };
  recognition.onerror = () => {
    listening.value = false;
  };
  listening.value = true;
  lastTranscript.value = '';
  lastScore.value = null;
  recognition.start();
}

function stopListening() {
  recognition?.stop();
  listening.value = false;
}

onUnmounted(() => {
  stopListening();
});

const progress = computed(() =>
  Math.round(((currentIndex.value + (finished.value ? 1 : 0)) / props.song.lyrics.length) * 100),
);

const firstCharsHint = computed(() => {
  const line = props.song.lyrics[currentIndex.value];
  return line
    .split(/\s+/)
    .map((word) => word.charAt(0) + '_'.repeat(Math.max(1, word.length - 1)))
    .join(' ');
});

const sessionData = computed(() => ({
  score: Math.round((correctCount.value / props.song.lyrics.length) * 100),
  linesPracticed: props.song.lyrics.length,
  linesCorrect: correctCount.value,
  durationSeconds: Math.round((Date.now() - startTime) / 1000),
}));

function assess(correct: boolean) {
  selfAssessed.value = true;
  if (correct) correctCount.value++;
}

function nextLine() {
  if (currentIndex.value < props.song.lyrics.length - 1) {
    currentIndex.value++;
    revealStage.value = 0;
    selfAssessed.value = false;
    lastTranscript.value = '';
    lastScore.value = null;
  } else {
    finished.value = true;
  }
}

function restart() {
  currentIndex.value = 0;
  revealStage.value = 0;
  selfAssessed.value = false;
  correctCount.value = 0;
  finished.value = false;
  startTime = Date.now();
}
</script>
