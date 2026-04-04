<template>
  <div class="space-y-6">
    <!-- Progress -->
    <div class="flex items-center gap-4">
      <div class="flex-1 bg-deep-card-hover rounded-full h-2">
        <div
          class="bg-purple-600 dark:bg-purple-400 h-2 rounded-full transition-all duration-500"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <span class="text-sm text-[#8A82A0]">
        {{ currentIndex + 1 }} / {{ song.lyrics.length }}
      </span>
      <span class="text-sm font-semibold" :class="scoreColor">
        {{ correctCount }}/{{ currentIndex + (answered ? 1 : 0) }}
      </span>
    </div>

    <!-- Question -->
    <div v-if="!finished" class="space-y-6">
      <!-- Prompt: show translation or phonetic, ask for original line -->
      <div class="p-6 bg-deep-card rounded-xl text-center space-y-2">
        <p class="text-sm text-[#8A82A0] uppercase tracking-wide">
          {{ questionType === 'translation' ? 'What is the original lyric for:' : 'Which lyric matches this phonetic:' }}
        </p>
        <p class="text-xl font-semibold text-[#F5F0EB]">
          {{ questionText }}
        </p>
      </div>

      <!-- Choices -->
      <div class="grid grid-cols-1 gap-3">
        <button
          v-for="(choice, i) in choices"
          :key="`q${currentIndex}-${i}`"
          @click="selectAnswer(i)"
          :disabled="answered"
          class="p-4 rounded-lg border-2 text-left transition-all duration-200"
          :class="choiceClass(i)"
        >
          <span class="inline-flex items-center gap-3">
            <span class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold"
              :class="choiceBadgeClass(i)"
            >
              {{ ['A', 'B', 'C', 'D'][i] }}
            </span>
            <span class="text-lg">{{ choice }}</span>
          </span>
        </button>
      </div>

      <!-- Feedback + Next -->
      <div v-if="answered" class="flex items-center justify-between">
        <p class="font-medium" :class="selectedIndex === correctChoiceIndex ? 'text-green-600' : 'text-red-500'">
          {{ selectedIndex === correctChoiceIndex ? '✓ Correct!' : `✗ The answer was: ${choices[correctChoiceIndex]}` }}
        </p>
        <Button variant="primary" @click="nextQuestion">
          Next
          <ChevronRight :size="18" />
        </Button>
      </div>
    </div>

    <!-- End screen -->
    <div v-else class="text-center py-6 space-y-4">
      <div class="text-6xl mb-2">{{ scoreMedal }}</div>
      <p class="text-xl font-semibold text-[#F5F0EB]">
        Score: {{ Math.round(scorePercent) }}%
      </p>
      <p class="text-[#8A82A0]">
        {{ correctCount }} correct out of {{ song.lyrics.length }}
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
import { ref, computed } from 'vue';
import { ChevronRight, RotateCcw } from 'lucide-vue-next';
import Button from '../ui/Button.vue';
import type { Song } from '../../types';

const props = defineProps<{ song: Song }>();
defineEmits<{
  finish: [data: { score: number; linesPracticed: number; linesCorrect: number; durationSeconds: number }];
}>();

const currentIndex = ref(0);
const correctCount = ref(0);
const answered = ref(false);
const selectedIndex = ref(-1);
const finished = ref(false);
let startTime = Date.now();

// Determine question type based on available data
const questionType = computed<'translation' | 'phonetic'>(() => {
  if (props.song.translations?.en?.[currentIndex.value]) return 'translation';
  return 'phonetic';
});

const questionText = computed(() => {
  if (questionType.value === 'translation') {
    return props.song.translations?.en?.[currentIndex.value] ?? '';
  }
  return props.song.phonetic_lyrics?.[currentIndex.value] ?? props.song.lyrics[currentIndex.value];
});

function generateChoices(): { choices: string[]; correctIndex: number } {
  const correctLine = props.song.lyrics[currentIndex.value];
  // Deduplicate other lines to avoid identical choices
  const otherLines = [...new Set(props.song.lyrics.filter((_, i) => i !== currentIndex.value))]
    .filter(line => line !== correctLine);

  // Pick up to 3 random distractors
  const shuffled = [...otherLines].sort(() => Math.random() - 0.5);
  const distractors = shuffled.slice(0, 3);

  // If not enough unique lines, generate word-scrambled variants
  const words = correctLine.split(/\s+/);
  let variantIdx = 0;
  while (distractors.length < 3) {
    const scrambled = [...words].sort(() => Math.random() - 0.5).join(' ');
    const variant = scrambled !== correctLine ? scrambled : `${correctLine} (${++variantIdx})`;
    if (!distractors.includes(variant)) {
      distractors.push(variant);
    }
    variantIdx++;
    if (variantIdx > 10) break; // Safety valve
  }

  // Place correct answer at a random position using explicit index tracking
  const correctIdx = Math.floor(Math.random() * (distractors.length + 1));
  const all = [...distractors];
  all.splice(correctIdx, 0, correctLine);

  return { choices: all, correctIndex: correctIdx };
}

let generated = generateChoices();
const choices = ref(generated.choices);
const correctChoiceIndex = ref(generated.correctIndex);

const progress = computed(() =>
  Math.round(((currentIndex.value + (finished.value ? 1 : 0)) / props.song.lyrics.length) * 100)
);

const scorePercent = computed(() => {
  const total = currentIndex.value + (answered.value ? 1 : 0);
  return total === 0 ? 0 : (correctCount.value / total) * 100;
});

const scoreColor = computed(() => {
  if (currentIndex.value === 0 && !answered.value) return 'text-gray-500';
  const pct = scorePercent.value;
  if (pct >= 80) return 'text-green-600 dark:text-green-400';
  if (pct >= 50) return 'text-yellow-600 dark:text-yellow-400';
  return 'text-red-500 dark:text-red-400';
});

const scoreMedal = computed(() => {
  const pct = (correctCount.value / props.song.lyrics.length) * 100;
  if (pct >= 90) return '🏆';
  if (pct >= 70) return '🎉';
  if (pct >= 50) return '👍';
  return '💪';
});

const sessionData = computed(() => ({
  score: Math.round((correctCount.value / props.song.lyrics.length) * 100),
  linesPracticed: props.song.lyrics.length,
  linesCorrect: correctCount.value,
  durationSeconds: Math.round((Date.now() - startTime) / 1000),
}));

function selectAnswer(index: number) {
  if (answered.value) return;
  selectedIndex.value = index;
  answered.value = true;
  if (index === correctChoiceIndex.value) {
    correctCount.value++;
  }
}

function choiceClass(i: number) {
  if (!answered.value) {
    return 'border-deep-border hover:border-gold cursor-pointer';
  }
  if (i === correctChoiceIndex.value) {
    return 'border-green-500 bg-green-50 dark:bg-green-900/20';
  }
  if (i === selectedIndex.value) {
    return 'border-red-500 bg-red-50 dark:bg-red-900/20';
  }
  return 'border-deep-border opacity-50';
}

function choiceBadgeClass(i: number) {
  if (!answered.value) {
    return 'bg-deep-card-hover text-[#B8B0D0]';
  }
  if (i === correctChoiceIndex.value) {
    return 'bg-green-500 text-white';
  }
  if (i === selectedIndex.value) {
    return 'bg-red-500 text-white';
  }
  return 'bg-deep-card-hover text-[#8A82A0]';
}

function nextQuestion() {
  if (currentIndex.value < props.song.lyrics.length - 1) {
    currentIndex.value++;
    answered.value = false;
    selectedIndex.value = -1;
    const gen = generateChoices();
    choices.value = gen.choices;
    correctChoiceIndex.value = gen.correctIndex;
  } else {
    finished.value = true;
  }
}

function restart() {
  currentIndex.value = 0;
  correctCount.value = 0;
  answered.value = false;
  selectedIndex.value = -1;
  finished.value = false;
  startTime = Date.now();
  const gen = generateChoices();
  choices.value = gen.choices;
  correctChoiceIndex.value = gen.correctIndex;
}
</script>
