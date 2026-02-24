<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <AppHeader />
    
    <div class="flex flex-1 overflow-hidden">
      <AppSidebar />
      
      <main class="flex-1 overflow-auto">
        <div class="container mx-auto px-4 py-8">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppHeader from './AppHeader.vue';
import AppSidebar from './AppSidebar.vue';
</script>
