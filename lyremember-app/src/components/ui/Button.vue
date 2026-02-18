<template>
  <button
    :type="type"
    :disabled="disabled || loading"
    :class="[
      'btn',
      `btn-${variant}`,
      `btn-${size}`,
      { 'btn-loading': loading },
      className,
    ]"
    @click="$emit('click', $event)"
  >
    <span v-if="loading" class="btn-spinner"></span>
    <slot />
  </button>
</template>

<script setup lang="ts">
defineProps<{
  type?: 'button' | 'submit' | 'reset';
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  className?: string;
}>();

defineEmits<{
  click: [event: MouseEvent];
}>();
</script>

<style scoped>
.btn {
  @apply inline-flex items-center justify-center gap-2 rounded-lg font-medium transition-all duration-200;
  @apply focus:outline-none focus:ring-2 focus:ring-offset-2;
  @apply disabled:opacity-50 disabled:cursor-not-allowed;
}

.btn-primary {
  @apply bg-indigo-600 text-white hover:bg-indigo-700;
  @apply focus:ring-indigo-500;
  @apply dark:bg-indigo-500 dark:hover:bg-indigo-600;
}

.btn-secondary {
  @apply bg-gray-200 text-gray-900 hover:bg-gray-300;
  @apply focus:ring-gray-400;
  @apply dark:bg-gray-700 dark:text-gray-100 dark:hover:bg-gray-600;
}

.btn-danger {
  @apply bg-red-600 text-white hover:bg-red-700;
  @apply focus:ring-red-500;
}

.btn-ghost {
  @apply bg-transparent text-gray-700 hover:bg-gray-100;
  @apply focus:ring-gray-300;
  @apply dark:text-gray-300 dark:hover:bg-gray-800;
}

.btn-sm {
  @apply px-3 py-1.5 text-sm;
}

.btn-md {
  @apply px-4 py-2 text-base;
}

.btn-lg {
  @apply px-6 py-3 text-lg;
}

.btn-loading {
  @apply opacity-75;
}

.btn-spinner {
  @apply inline-block w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin;
}
</style>
