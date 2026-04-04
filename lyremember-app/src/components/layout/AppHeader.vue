<template>
  <header class="bg-deep-card border-b border-deep-border" role="banner">
    <div class="px-4 py-3 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <button
          @click="uiStore.toggleSidebar()"
          class="p-2 rounded-lg hover:bg-deep-card-hover text-gray-300"
          aria-label="Toggle sidebar"
        >
          <Menu :size="20" />
        </button>

        <div class="flex items-center gap-2">
          <svg width="22" height="22" viewBox="0 0 48 48" fill="none">
            <path d="M18 44C16 38 14 32 13 28C11 22 10 16 12 10C14 6 16 4 14 2" stroke="#F2A93B" stroke-width="1.8" stroke-linecap="round"/>
            <path d="M30 44C32 38 34 32 35 28C37 22 38 16 36 10C34 6 32 4 34 2" stroke="#F2A93B" stroke-width="1.8" stroke-linecap="round"/>
            <line x1="18" y1="44" x2="30" y2="44" stroke="#F2A93B" stroke-width="1.8" stroke-linecap="round"/>
            <path d="M13.5 12H34.5" stroke="#F2A93B" stroke-width="1.5" stroke-linecap="round"/>
            <line x1="20" y1="16" x2="20" y2="20" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="20" y1="23" x2="20" y2="30" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="24" y1="14" x2="24" y2="22" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="24" y1="25" x2="24" y2="32" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="28" y1="16" x2="28" y2="24" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
            <line x1="28" y1="27" x2="28" y2="34" stroke="#F2A93B" stroke-width="1.1" stroke-linecap="round"/>
          </svg>
          <h1 class="text-xl font-bold">
            <span class="text-[#F5F0EB]">Ly</span><span class="text-gold">remember</span>
          </h1>
        </div>
      </div>
      
      <div class="flex items-center gap-2">
        <div class="relative">
          <button
            @click="showUserMenu = !showUserMenu"
            class="flex items-center gap-2 p-2 rounded-lg hover:bg-deep-card-hover text-[#B8B0D0]"
            aria-haspopup="true"
            :aria-expanded="showUserMenu"
          >
            <User :size="20" />
            <span class="text-sm font-medium hidden sm:block">
              {{ authStore.username }}
            </span>
            <ChevronDown :size="16" />
          </button>

          <div
            v-if="showUserMenu"
            class="absolute right-0 mt-2 w-48 bg-deep-card rounded-lg shadow-lg border border-deep-border py-1 z-50"
          >
            <router-link
              to="/profile"
              class="flex items-center gap-2 px-4 py-2 text-[#B8B0D0] hover:bg-deep-card-hover"
              @click="showUserMenu = false"
            >
              <User :size="16" />
              {{ $t('nav.profile') }}
            </router-link>
            <router-link
              to="/settings"
              class="flex items-center gap-2 px-4 py-2 text-[#B8B0D0] hover:bg-deep-card-hover"
              @click="showUserMenu = false"
            >
              <Settings :size="16" />
              {{ $t('nav.settings') }}
            </router-link>
            <button
              @click="handleLogout"
              class="w-full flex items-center gap-2 px-4 py-2 hover:bg-deep-card-hover text-red-400"
            >
              <LogOut :size="16" />
              {{ $t('nav.logout') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { Menu, Moon, Sun, User, ChevronDown, LogOut, Settings } from 'lucide-vue-next';
import { useAuthStore } from '../../stores/auth';
import { useUiStore } from '../../stores/ui';

const authStore = useAuthStore();
const uiStore = useUiStore();
const router = useRouter();
const showUserMenu = ref(false);

async function handleLogout() {
  await authStore.logout();
  router.push('/login');
}
</script>
