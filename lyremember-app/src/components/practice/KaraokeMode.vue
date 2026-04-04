<template>
  <Card>
    <div class="space-y-6">
      <!-- Previous lines (faded) -->
      <div class="space-y-2 opacity-40">
        <p v-for="(line, i) in previousLines" :key="i" class="text-lg">{{ line }}</p>
      </div>

      <!-- Current line to memorize -->
      <div class="py-6 border-y border-gray-200 dark:border-gray-700">
        <p class="text-2xl font-bold text-center text-gray-900 dark:text-white">
          {{ showAnswer ? song.lyrics[currentLine - 1] : '...' }}
        </p>
        <p
          v-if="song.phonetic_lyrics && showPhonetic"
          class="text-center text-gray-500 dark:text-gray-400 italic mt-2"
        >
          {{ song.phonetic_lyrics[currentLine - 1] }}
        </p>
      </div>

      <!-- Controls -->
      <div class="flex flex-col items-center gap-4">
        <div v-if="!showAnswer" class="text-center">
          <p class="text-gray-500 dark:text-gray-400 mb-4">Can you remember this line?</p>
          <Button variant="primary" size="lg" @click="showAnswer = true">
            <Eye :size="20" /> Reveal
          </Button>
        </div>

        <div v-else class="flex gap-4">
          <Button variant="danger" size="lg" @click="answer(false)">
            <X :size="20" /> Didn't know
          </Button>
          <Button variant="primary" size="lg" @click="answer(true)">
            <Check :size="20" /> Got it!
          </Button>
        </div>

        <button
          class="text-sm text-gray-400 hover:text-gray-600"
          @click="showPhonetic = !showPhonetic"
        >
          {{ showPhonetic ? 'Hide' : 'Show' }} phonetic
        </button>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Eye, Check, X } from 'lucide-vue-next';
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

const showAnswer = ref(false);
const showPhonetic = ref(false);

const previousLines = computed(() =>
  props.song.lyrics.slice(Math.max(0, props.currentLine - 3), props.currentLine - 1)
);

watch(() => props.currentLine, () => {
  showAnswer.value = false;
});

function answer(correct: boolean) {
  emit('answer', correct);
}
</script>
