<template>
  <Card>
    <div class="flex items-center justify-between">
      <div>
        <p class="text-sm text-gray-600 dark:text-gray-400">{{ label }}</p>
        <p class="text-3xl font-bold text-gray-900 dark:text-white">{{ display }}</p>
      </div>
      <slot name="icon" />
    </div>
  </Card>
</template>

<script setup lang="ts">
import Card from './Card.vue';

defineProps<{
  label: string;
  display: string | number;
}>();
</script>
