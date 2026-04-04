<template>
  <div class="flex flex-col items-center justify-center py-8 gap-3" role="status">
    <div
      class="animate-spin rounded-full border-4 border-gray-200 dark:border-gray-700"
      :class="[sizeClass, colorClass]"
    ></div>
    <p v-if="label" class="text-sm text-gray-500 dark:text-gray-400">{{ label }}</p>
    <span class="sr-only">Loading</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  size?: 'sm' | 'md' | 'lg';
  label?: string;
}>(), {
  size: 'md',
});

const sizeClass = computed(() => ({
  sm: 'w-5 h-5 border-2',
  md: 'w-8 h-8 border-4',
  lg: 'w-12 h-12 border-4',
}[props.size]));

const colorClass = 'border-t-indigo-600 dark:border-t-indigo-400';
</script>
