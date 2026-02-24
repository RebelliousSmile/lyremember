<template>
  <div :class="['alert', `alert-${type}`]" v-if="modelValue">
    <div class="alert-icon">
      <component :is="icon" :size="20" />
    </div>
    <div class="alert-content">
      <slot />
    </div>
    <button
      v-if="closable"
      @click="$emit('update:modelValue', false)"
      class="alert-close"
    >
      <X :size="16" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Info, CheckCircle, AlertCircle, X } from 'lucide-vue-next';

const props = defineProps<{
  type?: 'info' | 'success' | 'error';
  closable?: boolean;
  modelValue?: boolean;
}>();

defineEmits<{
  'update:modelValue': [value: boolean];
}>();

const icon = computed(() => {
  switch (props.type) {
    case 'success':
      return CheckCircle;
    case 'error':
      return AlertCircle;
    default:
      return Info;
  }
});
</script>

<style scoped>
.alert {
  @apply flex items-start gap-3 p-4 rounded-lg;
}

.alert-info {
  @apply bg-blue-50 text-blue-800 dark:bg-blue-900/20 dark:text-blue-300;
}

.alert-success {
  @apply bg-green-50 text-green-800 dark:bg-green-900/20 dark:text-green-300;
}

.alert-error {
  @apply bg-red-50 text-red-800 dark:bg-red-900/20 dark:text-red-300;
}

.alert-icon {
  @apply flex-shrink-0;
}

.alert-content {
  @apply flex-1;
}

.alert-close {
  @apply flex-shrink-0 text-current hover:opacity-70 transition-opacity;
}
</style>
