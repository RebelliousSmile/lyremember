<template>
  <div class="input-wrapper">
    <label v-if="label" :for="id" class="input-label">
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>
    <input
      :id="id"
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      :required="required"
      :disabled="disabled"
      :class="['input', { 'input-error': error }]"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    />
    <p v-if="error" class="input-error-text">{{ error }}</p>
    <p v-else-if="hint" class="input-hint">{{ hint }}</p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  id?: string;
  label?: string;
  type?: string;
  modelValue?: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  error?: string;
  hint?: string;
}>();

defineEmits<{
  'update:modelValue': [value: string];
}>();
</script>

<style scoped>
.input-wrapper {
  @apply w-full;
}

.input-label {
  @apply block text-sm font-medium mb-1;
  color: #B8B0D0;
}

.input {
  @apply w-full px-3 py-2 border border-deep-border rounded-lg;
  @apply focus:outline-none focus:ring-2 focus:ring-gold focus:border-transparent;
  @apply disabled:opacity-50 disabled:cursor-not-allowed;
  @apply bg-deep-surface text-[#F5F0EB];
  @apply transition-colors duration-200;
}

.input-error {
  @apply border-red-500 focus:ring-red-500;
}

.input-error-text {
  @apply mt-1 text-sm text-red-600 dark:text-red-400;
}

.input-hint {
  @apply mt-1 text-sm text-gray-500 dark:text-gray-400;
}
</style>
