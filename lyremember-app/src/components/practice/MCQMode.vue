<template>
  <Card>
    <div class="space-y-6">
      <div class="text-center py-4">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">What comes next after:</p>
        <p class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ previousLine }}
        </p>
      </div>

      <div class="grid grid-cols-1 gap-3 max-w-lg mx-auto">
        <button
          v-for="(option, i) in options"
          :key="i"
          :disabled="selectedIndex !== null"
          class="p-4 rounded-lg border-2 text-left transition-all"
          :class="optionClass(i)"
          @click="selectOption(i)"
        >
          <span class="font-mono text-sm text-gray-400 mr-2">{{ String.fromCharCode(65 + i) }}.</span>
          {{ option }}
        </button>
      </div>

      <div v-if="selectedIndex !== null" class="text-center">
        <Button variant="secondary" @click="next">Next line</Button>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
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

const selectedIndex = ref<number | null>(null);

const correctLine = computed(() => props.song.lyrics[props.currentLine - 1] || '');

const previousLine = computed(() => {
  if (props.currentLine <= 1) return '(First line of the song)';
  return props.song.lyrics[props.currentLine - 2];
});

const options = computed(() => {
  const lyrics = props.song.lyrics;
  const correct = correctLine.value;
  const distractors: string[] = [];

  // Pick distractors from other lines
  for (let i = 0; i < lyrics.length && distractors.length < 3; i++) {
    if (i !== props.currentLine - 1 && lyrics[i] !== correct) {
      distractors.push(lyrics[i]);
    }
  }

  // If not enough distractors, pad with modified versions
  while (distractors.length < 3) {
    distractors.push(correct.split('').reverse().join(''));
  }

  // Shuffle: insert correct answer at deterministic position
  const position = (props.currentLine * 3 + 1) % 4;
  const result = [...distractors.slice(0, 3)];
  result.splice(position, 0, correct);
  return result.slice(0, 4);
});

const correctIndex = computed(() => options.value.indexOf(correctLine.value));

watch(() => props.currentLine, () => {
  selectedIndex.value = null;
});

function optionClass(index: number): string {
  if (selectedIndex.value === null) {
    return 'border-gray-200 dark:border-gray-600 hover:border-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/20 cursor-pointer';
  }
  if (index === correctIndex.value) {
    return 'border-green-500 bg-green-50 dark:bg-green-900/20';
  }
  if (index === selectedIndex.value) {
    return 'border-red-500 bg-red-50 dark:bg-red-900/20';
  }
  return 'border-gray-200 dark:border-gray-600 opacity-50';
}

function selectOption(index: number) {
  if (selectedIndex.value !== null) return;
  selectedIndex.value = index;
}

function next() {
  emit('answer', selectedIndex.value === correctIndex.value);
}
</script>
