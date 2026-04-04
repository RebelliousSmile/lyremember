<template>
  <Teleport to="body">
    <transition name="fade">
      <div
        v-if="show"
        class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
        @click.self="$emit('close')"
        role="dialog"
        aria-modal="true"
        aria-label="Keyboard shortcuts"
      >
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-md w-full mx-4 p-6 space-y-4">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
              {{ $t('shortcuts.title') }}
            </h2>
            <button
              @click="$emit('close')"
              class="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700"
              aria-label="Close"
            >
              <X :size="20" />
            </button>
          </div>

          <div class="space-y-2">
            <div
              v-for="shortcut in shortcuts"
              :key="shortcut.keys"
              class="flex items-center justify-between py-2"
            >
              <span class="text-sm text-gray-600 dark:text-gray-400">
                {{ $t(shortcut.description) }}
              </span>
              <div class="flex gap-1">
                <kbd
                  v-for="key in shortcut.keys.split(' ')"
                  :key="key"
                  class="px-2 py-1 text-xs font-mono bg-gray-100 dark:bg-gray-700 border
                         border-gray-300 dark:border-gray-600 rounded text-gray-700 dark:text-gray-300"
                >
                  {{ key }}
                </kbd>
              </div>
            </div>
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script setup lang="ts">
import { X } from 'lucide-vue-next';
import { shortcuts } from '../../composables/useKeyboardShortcuts';

defineProps<{ show: boolean }>();
defineEmits<{ close: [] }>();
</script>
