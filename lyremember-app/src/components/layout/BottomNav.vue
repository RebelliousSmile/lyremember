<template>
  <nav class="bottom-nav" aria-label="Navigation principale">
    <!-- Ma Lyre -->
    <router-link to="/songs" class="bottom-nav-item" active-class="bottom-nav-active">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" :stroke="isActive('/songs') ? '#F2A93B' : '#8A82A0'" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
        <path d="M2 4C4 3 7 3 9 4C11 5 12 5 12 5V21C12 21 11 20 9 19.5C7 19 4 19 2 20V4Z"/>
        <path d="M22 4C20 3 17 3 15 4C13 5 12 5 12 5V21C12 21 13 20 15 19.5C17 19 20 19 22 20V4Z"/>
        <line x1="5" y1="8" x2="9" y2="8" opacity="0.5"/>
        <line x1="5" y1="11" x2="9" y2="11" opacity="0.5"/>
        <line x1="15" y1="8" x2="19" y2="8" opacity="0.5"/>
        <line x1="15" y1="11" x2="19" y2="11" opacity="0.5"/>
      </svg>
      <span :class="isActive('/songs') ? 'text-gold font-semibold' : 'text-[#8A82A0]'">Ma Lyre</span>
    </router-link>

    <!-- FAB Ajouter -->
    <router-link to="/songs/add" class="bottom-nav-fab" aria-label="Ajouter une chanson">
      <span class="text-2xl text-deep font-light leading-none">+</span>
    </router-link>

    <!-- Mémoriser -->
    <router-link to="/practice" class="bottom-nav-item" active-class="bottom-nav-active">
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" :stroke="isActive('/practice') ? '#F2A93B' : '#8A82A0'" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
        <path d="M4 4H9C9 4 9 2 10.5 2C12 2 12 4 12 4H18V9C18 9 20 9 20 10.5C20 12 18 12 18 12V18H12C12 18 12 20 10.5 20C9 20 9 18 9 18H4V12C4 12 2 12 2 10.5C2 9 4 9 4 9V4Z"/>
      </svg>
      <span :class="isActive('/practice') ? 'text-gold font-semibold' : 'text-[#8A82A0]'">Mémoriser</span>
    </router-link>
  </nav>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();

function isActive(path: string): boolean {
  return route.path.startsWith(path);
}
</script>

<style scoped>
.bottom-nav {
  @apply fixed bottom-0 left-0 right-0 flex items-center justify-around;
  @apply border-t border-deep-border;
  background: #131022;
  height: 68px;
  padding: 0 24px;
  z-index: 40;
}

.bottom-nav-item {
  @apply flex flex-col items-center gap-1 p-2 no-underline;
  font-size: 10px;
  font-family: sans-serif;
}

.bottom-nav-fab {
  @apply flex items-center justify-center rounded-2xl no-underline;
  width: 52px;
  height: 52px;
  background: linear-gradient(135deg, #F2A93B, #E09520);
  box-shadow: 0 4px 20px rgba(242, 169, 59, 0.25);
  margin-top: -14px;
}
</style>
