<template>
  <div class="min-h-screen flex flex-col bg-deep">
    <!-- Skip link for accessibility -->
    <a
      href="#main-content"
      class="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-50
             focus:px-4 focus:py-2 focus:bg-gold focus:text-deep focus:rounded-lg"
    >
      Skip to content
    </a>

    <AppHeader />

    <div class="flex flex-1 overflow-hidden">
      <AppSidebar />

      <main id="main-content" class="flex-1 overflow-auto pb-20 md:pb-0" role="main">
        <div class="container mx-auto px-4 py-8">
          <slot />
        </div>
      </main>
    </div>

    <!-- Bottom nav for mobile only -->
    <BottomNav class="md:hidden" />

    <KeyboardShortcutsHelp :show="showHelp" @close="showHelp = false" />
  </div>
</template>

<script setup lang="ts">
import AppHeader from './AppHeader.vue';
import AppSidebar from './AppSidebar.vue';
import BottomNav from './BottomNav.vue';
import KeyboardShortcutsHelp from '../ui/KeyboardShortcutsHelp.vue';
import { useKeyboardShortcuts } from '../../composables/useKeyboardShortcuts';

const { showHelp } = useKeyboardShortcuts();
</script>
