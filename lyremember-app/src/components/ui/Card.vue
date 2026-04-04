<template>
  <div :class="['card', className]">
    <div v-if="$slots.header" class="card-header">
      <slot name="header" />
    </div>
    <div class="card-body">
      <slot />
    </div>
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  className?: string;
}>();
</script>

<style scoped>
.card {
  @apply bg-deep-card rounded-lg shadow-md overflow-hidden border border-deep-border;
}

.card-header {
  @apply px-6 py-4 border-b border-deep-border;
}

.card-body {
  @apply px-6 py-4;
}

.card-footer {
  @apply px-6 py-4 border-t border-deep-border;
}
</style>
