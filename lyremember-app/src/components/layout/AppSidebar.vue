<template>
  <aside
    :class="[
      'sidebar',
      { 'sidebar-open': uiStore.sidebarOpen },
    ]"
  >
    <nav class="sidebar-nav">
      <router-link
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        class="nav-item"
        active-class="nav-item-active"
      >
        <component :is="item.icon" :size="20" />
        <span>{{ item.label }}</span>
      </router-link>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { LayoutDashboard, Music, PlayCircle, User } from 'lucide-vue-next';
import { useUiStore } from '../../stores/ui';

const uiStore = useUiStore();

const navItems = [
  { to: '/dashboard', icon: LayoutDashboard, label: 'Dashboard' },
  { to: '/songs', icon: Music, label: 'Songs' },
  { to: '/practice', icon: PlayCircle, label: 'Practice' },
  { to: '/profile', icon: User, label: 'Profile' },
];
</script>

<style scoped>
.sidebar {
  @apply w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700;
  @apply transition-transform duration-300;
  @apply hidden md:block;
}

.sidebar:not(.sidebar-open) {
  @apply -translate-x-full md:translate-x-0;
}

.sidebar-nav {
  @apply flex flex-col gap-1 p-4;
}

.nav-item {
  @apply flex items-center gap-3 px-4 py-3 rounded-lg;
  @apply text-gray-700 dark:text-gray-300;
  @apply hover:bg-gray-100 dark:hover:bg-gray-700;
  @apply transition-colors duration-200;
}

.nav-item-active {
  @apply bg-indigo-50 dark:bg-indigo-900/20;
  @apply text-indigo-600 dark:text-indigo-400;
  @apply font-medium;
}
</style>
