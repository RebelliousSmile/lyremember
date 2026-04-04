<template>
  <div class="space-y-6">
    <!-- Progress -->
    <div class="flex items-center gap-4">
      <div class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-green-600 dark:bg-green-400 h-2 rounded-full transition-all duration-500"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <span class="text-sm text-gray-600 dark:text-gray-400">
        {{ currentIndex + 1 }} / {{ song.lyrics.length }}
      </span>
      <span class="text-sm font-semibold" :class="scoreColor">
        {{ correctCount }}/{{ answeredCount }}
      </span>
    </div>

    <!-- Current line -->
    <div v-if="!finished" class="space-y-4">
      <!-- Phonetic hint -->
      <p v-if="song.phonetic_lyrics?.[currentIndex]" class="text-sm italic text-gray-500 dark:text-gray-400">
        {{ song.phonetic_lyrics[currentIndex] }}
      </p>

      <!-- Line with blanks -->
      <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
        <p class="text-xl leading-relaxed">
          <template v-for="(token, i) in currentTokens" :key="i">
            <span v-if="!token.hidden" class="text-gray-900 dark:text-white">{{ token.text }} </span>
            <span
              v-else-if="token.revealed"
              class="font-bold"
              :class="token.correct ? 'text-green-600 dark:text-green-400' : 'text-red-500 dark:text-red-400'"
            >
              {{ token.correct ? token.text : `${userInputs[token.blankIndex]}→${token.text}` }}
              {{ ' ' }}
            </span>
            <span v-else class="inline-block mx-1">
              <input
                :ref="el => { if (token.blankIndex === currentBlankIndex) activeInput = el as HTMLInputElement }"
                type="text"
                v-model="userInputs[token.blankIndex]"
                :placeholder="'_'.repeat(Math.max(3, token.text.length))"
                class="w-32 px-2 py-1 border-b-2 border-indigo-400 bg-transparent text-center
                       text-lg font-medium focus:outline-none focus:border-indigo-600
                       dark:text-white dark:border-indigo-500"
                @keydown.enter="checkBlank(token.blankIndex)"
              />
              {{ ' ' }}
            </span>
          </template>
        </p>
      </div>

      <!-- Translation hint -->
      <p v-if="showHint && song.translations?.en?.[currentIndex]" class="text-sm text-gray-500 dark:text-gray-400">
        💡 {{ song.translations.en[currentIndex] }}
      </p>

      <!-- Actions -->
      <div class="flex items-center justify-between">
        <div class="flex gap-2">
          <Button variant="primary" @click="checkAllBlanks" :disabled="!hasUnrevealedBlanks">
            <Check :size="18" />
            Check
          </Button>
          <Button variant="ghost" size="sm" @click="showHint = !showHint">
            <Lightbulb :size="16" />
            Hint
          </Button>
        </div>
        <Button variant="secondary" @click="nextLine" :disabled="hasUnrevealedBlanks">
          Next
          <ChevronRight :size="18" />
        </Button>
      </div>
    </div>

    <!-- End screen -->
    <div v-else class="text-center py-6 space-y-4">
      <div class="text-6xl mb-2">{{ scoreMedal }}</div>
      <p class="text-xl font-semibold text-gray-900 dark:text-white">
        Score: {{ Math.round(scorePercent) }}%
      </p>
      <p class="text-gray-600 dark:text-gray-400">
        {{ correctCount }} correct out of {{ answeredCount }} blanks
      </p>
      <div class="flex justify-center gap-3">
        <Button variant="primary" @click="restart">
          <RotateCcw :size="18" />
          Retry
        </Button>
        <Button variant="secondary" @click="$emit('finish', sessionData)">
          Done
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue';
import { Check, ChevronRight, Lightbulb, RotateCcw } from 'lucide-vue-next';
import Button from '../ui/Button.vue';
import type { Song } from '../../types';

interface Token {
  text: string;
  hidden: boolean;
  revealed: boolean;
  correct: boolean;
  blankIndex: number;
}

const props = defineProps<{ song: Song }>();
defineEmits<{
  finish: [data: { score: number; linesPracticed: number; linesCorrect: number; durationSeconds: number }];
}>();

const currentIndex = ref(0);
const finished = ref(false);
const showHint = ref(false);
const correctCount = ref(0);
const answeredCount = ref(0);
const userInputs = ref<string[]>([]);
const activeInput = ref<HTMLInputElement | null>(null);
const currentBlankIndex = ref(0);
const startTime = Date.now();

function tokenizeLine(line: string): Token[] {
  const words = line.split(/\s+/).filter(Boolean);
  const blanksCount = Math.max(1, Math.ceil(words.length * 0.3));
  const hiddenIndices = new Set<number>();

  // Randomly select words to hide
  while (hiddenIndices.size < blanksCount && hiddenIndices.size < words.length) {
    hiddenIndices.add(Math.floor(Math.random() * words.length));
  }

  let blankIdx = 0;
  return words.map((word, i) => {
    const isHidden = hiddenIndices.has(i);
    const token: Token = {
      text: word,
      hidden: isHidden,
      revealed: false,
      correct: false,
      blankIndex: isHidden ? blankIdx : -1,
    };
    if (isHidden) blankIdx++;
    return token;
  });
}

const currentTokens = ref<Token[]>(tokenizeLine(props.song.lyrics[0]));

function initLine() {
  currentTokens.value = tokenizeLine(props.song.lyrics[currentIndex.value]);
  const blanksCount = currentTokens.value.filter(t => t.hidden).length;
  userInputs.value = Array(blanksCount).fill('');
  currentBlankIndex.value = 0;
  showHint.value = false;
  nextTick(() => activeInput.value?.focus());
}

const hasUnrevealedBlanks = computed(() =>
  currentTokens.value.some(t => t.hidden && !t.revealed)
);

const progress = computed(() =>
  Math.round(((currentIndex.value + (finished.value ? 1 : 0)) / props.song.lyrics.length) * 100)
);

const scorePercent = computed(() =>
  answeredCount.value === 0 ? 0 : (correctCount.value / answeredCount.value) * 100
);

const scoreColor = computed(() => {
  if (answeredCount.value === 0) return 'text-gray-500';
  const pct = scorePercent.value;
  if (pct >= 80) return 'text-green-600 dark:text-green-400';
  if (pct >= 50) return 'text-yellow-600 dark:text-yellow-400';
  return 'text-red-500 dark:text-red-400';
});

const scoreMedal = computed(() => {
  const pct = scorePercent.value;
  if (pct >= 90) return '🏆';
  if (pct >= 70) return '🎉';
  if (pct >= 50) return '👍';
  return '💪';
});

const sessionData = computed(() => ({
  score: Math.round(scorePercent.value),
  linesPracticed: props.song.lyrics.length,
  linesCorrect: correctCount.value,
  durationSeconds: Math.round((Date.now() - startTime) / 1000),
}));

function normalize(str: string): string {
  return str.toLowerCase().trim().replace(/[^\w\s]/g, '');
}

function checkBlank(blankIndex: number) {
  const token = currentTokens.value.find(t => t.blankIndex === blankIndex);
  if (!token || token.revealed) return;

  const userAnswer = normalize(userInputs.value[blankIndex]);
  const correct = normalize(token.text);
  const isCorrect = userAnswer === correct;

  token.revealed = true;
  token.correct = isCorrect;
  answeredCount.value++;
  if (isCorrect) correctCount.value++;

  // Move focus to next unrevealed blank
  const nextBlank = currentTokens.value.find(t => t.hidden && !t.revealed);
  if (nextBlank) {
    currentBlankIndex.value = nextBlank.blankIndex;
    nextTick(() => activeInput.value?.focus());
  }
}

function checkAllBlanks() {
  currentTokens.value
    .filter(t => t.hidden && !t.revealed)
    .forEach(t => checkBlank(t.blankIndex));
}

function nextLine() {
  if (currentIndex.value < props.song.lyrics.length - 1) {
    currentIndex.value++;
    initLine();
  } else {
    finished.value = true;
  }
}

function restart() {
  currentIndex.value = 0;
  finished.value = false;
  correctCount.value = 0;
  answeredCount.value = 0;
  initLine();
}

watch(currentIndex, () => {}, { immediate: true });

// Initialize first line
nextTick(() => activeInput.value?.focus());
</script>
