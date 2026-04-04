<template>
  <Card>
    <div class="space-y-6">
      <!-- Line with blank -->
      <div class="py-6 text-center">
        <p class="text-xl text-gray-900 dark:text-white leading-relaxed">
          <template v-for="(part, i) in lineParts" :key="i">
            <span v-if="part.type === 'text'">{{ part.value }}</span>
            <span
              v-else
              class="inline-block min-w-[80px] border-b-2 mx-1"
              :class="feedback === null
                ? 'border-indigo-400'
                : feedback ? 'border-green-500 text-green-600' : 'border-red-500 text-red-600'"
            >
              {{ feedback !== null ? hiddenWord : '____' }}
            </span>
          </template>
        </p>
      </div>

      <!-- Single-word line: auto-pass -->
      <div v-if="isSingleWord" class="text-center">
        <p class="text-gray-500 dark:text-gray-400 mb-3">Single-word line — auto-pass</p>
        <Button variant="secondary" @click="emit('answer', true)">Next line</Button>
      </div>

      <!-- Input -->
      <div v-else-if="feedback === null" class="flex gap-3 max-w-md mx-auto">
        <input
          ref="inputRef"
          v-model="userInput"
          type="text"
          class="flex-1 px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-center text-lg"
          placeholder="Type the missing word..."
          @keydown.enter="checkAnswer"
        />
        <Button variant="primary" @click="checkAnswer">Check</Button>
      </div>

      <!-- Feedback -->
      <div v-else class="text-center space-y-3">
        <p v-if="feedback" class="text-green-600 dark:text-green-400 font-semibold text-lg">Correct!</p>
        <div v-else>
          <p class="text-red-600 dark:text-red-400 font-semibold text-lg">
            The answer was: <span class="underline">{{ hiddenWord }}</span>
          </p>
        </div>
        <Button variant="secondary" @click="next">Next line</Button>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue';
import Card from '../ui/Card.vue';
import Button from '../ui/Button.vue';
import type { Song } from '../../types';

const props = defineProps<{
  song: Song;
  currentLine: number;
}>();

const emit = defineEmits<{
  answer: [correct: boolean];
}>();

const userInput = ref('');
const feedback = ref<boolean | null>(null);
const inputRef = ref<HTMLInputElement | null>(null);

const currentLyric = computed(() => props.song.lyrics[props.currentLine - 1] || '');

const hiddenWordIndex = computed(() => {
  const words = currentLyric.value.split(/\s+/);
  if (words.length <= 1) return -1; // Single-word lines: auto-pass
  return ((props.currentLine * 7) + 3) % words.length;
});

const hiddenWord = computed(() => {
  const words = currentLyric.value.split(/\s+/);
  return words[hiddenWordIndex.value] || '';
});

const isSingleWord = computed(() => hiddenWordIndex.value === -1);

const lineParts = computed(() => {
  if (isSingleWord.value) {
    return [{ type: 'text' as const, value: currentLyric.value }];
  }
  const words = currentLyric.value.split(/\s+/);
  const parts: { type: 'text' | 'blank'; value: string }[] = [];
  words.forEach((word, i) => {
    if (i > 0) parts.push({ type: 'text', value: ' ' });
    if (i === hiddenWordIndex.value) {
      parts.push({ type: 'blank', value: word });
    } else {
      parts.push({ type: 'text', value: word });
    }
  });
  return parts;
});

watch(() => props.currentLine, () => {
  userInput.value = '';
  feedback.value = null;
  nextTick(() => inputRef.value?.focus());
});

function checkAnswer() {
  if (!userInput.value.trim()) return;
  const correct = userInput.value.trim().toLowerCase() === hiddenWord.value.toLowerCase();
  feedback.value = correct;
}

function next() {
  emit('answer', feedback.value ?? false);
}
</script>
