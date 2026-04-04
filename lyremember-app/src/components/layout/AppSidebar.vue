<template>
  <aside
    :class="[
      'sidebar',
      { 'sidebar-open': uiStore.sidebarOpen },
    ]"
  >
    <nav class="sidebar-nav" aria-label="Main navigation">
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
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { LayoutDashboard, Music, PlayCircle, User, Settings } from 'lucide-vue-next';
import { useUiStore } from '../../stores/ui';

const { t } = useI18n();
const uiStore = useUiStore();

const navItems = computed(() => [
  { to: '/dashboard', icon: LayoutDashboard, label: t('nav.dashboard') },
  { to: '/songs', icon: Music, label: t('nav.songs') },
  { to: '/practice', icon: PlayCircle, label: t('nav.practice') },
  { to: '/profile', icon: User, label: t('nav.profile') },
  { to: '/settings', icon: Settings, label: t('nav.settings') },
]);
</script>

<style scoped>
.sidebar {
  @apply w-64 bg-deep-card border-r border-deep-border;
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
  color: #B8B0D0;
  @apply hover:bg-deep-card-hover;
  @apply transition-colors duration-200;
}

.nav-item-active {
  background: rgba(242, 169, 59, 0.1);
  color: #F2A93B;
  @apply font-medium;
}
</style>
