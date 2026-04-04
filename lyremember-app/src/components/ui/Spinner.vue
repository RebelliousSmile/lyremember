<template>
  <div class="flex flex-col items-center justify-center py-8 gap-3" role="status">
    <div
      class="animate-spin rounded-full border-4 border-deep-border"
      :class="[sizeClass, colorClass]"
    ></div>
    <p v-if="label" class="text-sm text-[#8A82A0]">{{ label }}</p>
    <span class="sr-only">{{ $t('common.loading') }}</span>
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

const colorClass = 'border-t-gold';
</script>
