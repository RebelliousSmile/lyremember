<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <!-- Skip link for accessibility -->
    <a
      href="#main-content"
      class="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50
             focus:px-4 focus:py-2 focus:bg-indigo-600 focus:text-white focus:rounded-lg"
    >
      Skip to content
    </a>

    <AppHeader />

    <div class="flex flex-1 overflow-hidden">
      <AppSidebar />

      <main id="main-content" class="flex-1 overflow-auto" role="main">
        <div class="container mx-auto px-4 py-8">
          <slot />
        </div>
      </main>
    </div>

    <KeyboardShortcutsHelp :show="showHelp" @close="showHelp = false" />
  </div>
</template>

<script setup lang="ts">
import AppHeader from './AppHeader.vue';
import AppSidebar from './AppSidebar.vue';
import KeyboardShortcutsHelp from '../ui/KeyboardShortcutsHelp.vue';
import { useKeyboardShortcuts } from '../../composables/useKeyboardShortcuts';

const { showHelp } = useKeyboardShortcuts();
</script>
