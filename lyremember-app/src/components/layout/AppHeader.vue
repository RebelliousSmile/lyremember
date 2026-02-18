<template>
  <header class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
    <div class="px-4 py-3 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <button
          @click="uiStore.toggleSidebar()"
          class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          <Menu :size="20" />
        </button>
        
        <h1 class="text-xl font-bold text-indigo-600 dark:text-indigo-400">
          LyRemember
        </h1>
      </div>
      
      <div class="flex items-center gap-2">
        <button
          @click="uiStore.toggleDarkMode()"
          class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700"
          :title="uiStore.darkMode ? 'Light mode' : 'Dark mode'"
        >
          <Moon v-if="uiStore.darkMode" :size="20" />
          <Sun v-else :size="20" />
        </button>
        
        <div class="relative">
          <button
            @click="showUserMenu = !showUserMenu"
            class="flex items-center gap-2 p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            <User :size="20" />
            <span class="text-sm font-medium hidden sm:block">
              {{ authStore.username }}
            </span>
            <ChevronDown :size="16" />
          </button>
          
          <div
            v-if="showUserMenu"
            class="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-1"
          >
            <router-link
              to="/profile"
              class="flex items-center gap-2 px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700"
              @click="showUserMenu = false"
            >
              <User :size="16" />
              Profile
            </router-link>
            <button
              @click="handleLogout"
              class="w-full flex items-center gap-2 px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-red-600"
            >
              <LogOut :size="16" />
              Logout
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
import { Menu, Moon, Sun, User, ChevronDown, LogOut } from 'lucide-vue-next';
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
