<template>
  <Teleport to="body">
    <TransitionGroup
      tag="div"
      name="toast"
      class="fixed top-4 right-4 z-50 flex flex-col gap-2"
    >
      <div
        v-for="toast in toasts"
        :key="toast.id"
        :class="[
          'px-4 py-3 rounded-lg shadow-lg flex items-center gap-3 min-w-[300px] max-w-[420px]',
          typeClasses[toast.type],
        ]"
      >
        <CheckCircle v-if="toast.type === 'success'" :size="18" />
        <AlertCircle v-else-if="toast.type === 'error'" :size="18" />
        <Info v-else :size="18" />
        <span class="flex-1 text-sm">{{ toast.message }}</span>
        <button @click="remove(toast.id)" class="opacity-60 hover:opacity-100">
          <X :size="16" />
        </button>
      </div>
    </TransitionGroup>
  </Teleport>
</template>

<script setup lang="ts">
import { CheckCircle, AlertCircle, Info, X } from 'lucide-vue-next';
import { useToast, type Toast } from '../../composables/useToast';

const { toasts, remove } = useToast();

const typeClasses: Record<Toast['type'], string> = {
  success: 'bg-green-600 text-white',
  error: 'bg-red-600 text-white',
  info: 'bg-gray-800 text-white dark:bg-gray-200 dark:text-gray-900',
};
</script>

<style scoped>
.toast-enter-active { transition: all 0.3s ease; }
.toast-leave-active { transition: all 0.2s ease; }
.toast-enter-from { opacity: 0; transform: translateX(100%); }
.toast-leave-to { opacity: 0; transform: translateX(100%); }
</style>
