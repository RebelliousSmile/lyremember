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
  @apply bg-gold text-deep hover:bg-gold-light font-semibold;
  @apply focus:ring-gold;
}

.btn-secondary {
  @apply bg-deep-card-hover border border-deep-border hover:bg-deep-card;
  @apply focus:ring-violet-accent;
  color: #B8B0D0;
}

.btn-danger {
  @apply bg-red-600 text-white hover:bg-red-700;
  @apply focus:ring-red-500;
}

.btn-ghost {
  @apply bg-transparent hover:bg-deep-card-hover;
  @apply focus:ring-deep-border;
  color: #B8B0D0;
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
