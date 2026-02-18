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
  @apply block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1;
}

.input {
  @apply w-full px-3 py-2 border border-gray-300 rounded-lg;
  @apply focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent;
  @apply disabled:bg-gray-100 disabled:cursor-not-allowed;
  @apply dark:bg-gray-800 dark:border-gray-600 dark:text-white;
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
