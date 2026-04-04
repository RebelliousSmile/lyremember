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
        <div class="bg-deep-card rounded-xl shadow-2xl max-w-md w-full mx-4 p-6 space-y-4">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-[#F5F0EB]">
              {{ $t('shortcuts.title') }}
            </h2>
            <button
              @click="$emit('close')"
              class="p-1 rounded hover:bg-deep-card-hover"
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
              <span class="text-sm text-[#8A82A0]">
                {{ $t(shortcut.description) }}
              </span>
              <div class="flex gap-1">
                <kbd
                  v-for="key in shortcut.keys.split(' ')"
                  :key="key"
                  class="px-2 py-1 text-xs font-mono bg-deep-card-hover border
                         border-deep-border rounded text-[#B8B0D0]"
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
