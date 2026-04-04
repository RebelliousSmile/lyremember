<template>
  <Card>
    <div class="text-center space-y-6 py-6">
      <div>
        <p class="text-6xl font-bold" :class="scoreColor">{{ score }}%</p>
        <p class="text-gray-500 dark:text-gray-400 mt-2">
          {{ correct }} / {{ total }} lines correct
        </p>
      </div>

      <div class="text-sm text-gray-500 dark:text-gray-400">
        <p>{{ songTitle }} · {{ modeLabel }}</p>
      </div>

      <p class="text-lg font-semibold" :class="scoreColor">{{ message }}</p>

      <div class="flex gap-4 justify-center">
        <Button variant="secondary" size="lg" @click="$emit('back')">
          <ArrowLeft :size="18" /> Back to song
        </Button>
        <Button variant="primary" size="lg" @click="$emit('retry')">
          <RotateCcw :size="18" /> Try again
        </Button>
      </div>
    </div>
  </Card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { ArrowLeft, RotateCcw } from 'lucide-vue-next';
import Card from '../ui/Card.vue';
import Button from '../ui/Button.vue';
import { MODE_LABELS } from '../../stores/practice';
import type { PracticeMode } from '../../stores/practice';

const props = defineProps<{
  score: number;
  correct: number;
  total: number;
  mode: PracticeMode;
  songTitle: string;
}>();

defineEmits<{
  retry: [];
  back: [];
}>();

const modeLabel = computed(() => MODE_LABELS[props.mode] ?? props.mode);

const scoreColor = computed(() => {
  if (props.score >= 80) return 'text-green-600 dark:text-green-400';
  if (props.score >= 50) return 'text-yellow-600 dark:text-yellow-400';
  return 'text-red-600 dark:text-red-400';
});

const message = computed(() => {
  if (props.score >= 90) return 'Excellent!';
  if (props.score >= 70) return 'Great job!';
  if (props.score >= 50) return 'Keep practicing!';
  return 'Don\'t give up!';
});
</script>
