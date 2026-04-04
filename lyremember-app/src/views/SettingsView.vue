<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        {{ $t('settings.title') }}
      </h1>

      <!-- Appearance -->
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('settings.appearance') }}</h2>
        </template>

        <div class="space-y-6">
          <!-- Dark mode -->
          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium text-gray-900 dark:text-white">{{ $t('settings.darkMode') }}</p>
              <p class="text-sm text-gray-500 dark:text-gray-400">{{ $t('settings.darkModeDesc') }}</p>
            </div>
            <button
              @click="uiStore.toggleDarkMode()"
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
              :class="uiStore.darkMode ? 'bg-indigo-600' : 'bg-gray-300'"
            >
              <span
                class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
                :class="uiStore.darkMode ? 'translate-x-6' : 'translate-x-1'"
              ></span>
            </button>
          </div>

          <!-- Language -->
          <div class="flex items-center justify-between">
            <div>
              <p class="font-medium text-gray-900 dark:text-white">{{ $t('settings.language') }}</p>
              <p class="text-sm text-gray-500 dark:text-gray-400">{{ $t('settings.languageDesc') }}</p>
            </div>
            <select
              :value="locale"
              @change="changeLocale(($event.target as HTMLSelectElement).value)"
              class="rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700
                     px-3 py-2 text-sm text-gray-900 dark:text-white"
            >
              <option v-for="lang in supportedLocales" :key="lang.code" :value="lang.code">
                {{ lang.label }}
              </option>
            </select>
          </div>
        </div>
      </Card>

      <!-- Genius API -->
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('settings.integrations') }}</h2>
        </template>

        <div class="space-y-4">
          <div>
            <h3 class="font-medium text-gray-900 dark:text-white">{{ $t('settings.geniusApi') }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
              {{ $t('settings.geniusDesc') }}
            </p>
          </div>

          <div class="space-y-2">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ $t('settings.geniusToken') }}
            </label>
            <div class="flex gap-2">
              <input
                v-model="geniusToken"
                type="password"
                :placeholder="$t('settings.geniusTokenPlaceholder')"
                class="flex-1 rounded-lg border border-gray-300 dark:border-gray-600 bg-white
                       dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white
                       focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
              <Button variant="primary" @click="saveGeniusToken">
                {{ $t('common.save') }}
              </Button>
            </div>
            <p v-if="tokenSaved" class="text-sm text-green-600 dark:text-green-400">
              {{ $t('settings.tokenSaved') }}
            </p>
            <p class="text-xs text-gray-400 dark:text-gray-500">
              {{ $t('settings.geniusHelp') }}
            </p>
          </div>

          <!-- Genius Search -->
          <div v-if="geniusToken" class="pt-4 border-t border-gray-200 dark:border-gray-700 space-y-3">
            <h3 class="font-medium text-gray-900 dark:text-white">{{ $t('settings.searchSongs') }}</h3>
            <div class="flex gap-2">
              <input
                v-model="searchQuery"
                :placeholder="$t('settings.searchPlaceholder')"
                class="flex-1 rounded-lg border border-gray-300 dark:border-gray-600 bg-white
                       dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white
                       focus:outline-none focus:ring-2 focus:ring-indigo-500"
                @keydown.enter="searchGenius"
              />
              <Button variant="primary" @click="searchGenius" :loading="searching">
                {{ $t('settings.search') }}
              </Button>
            </div>

            <div v-if="searchResults.length > 0" class="space-y-2">
              <div
                v-for="result in searchResults"
                :key="result.id"
                class="flex items-center justify-between p-3 rounded-lg border
                       border-gray-200 dark:border-gray-700 hover:border-indigo-300
                       dark:hover:border-indigo-600 transition-colors"
              >
                <div>
                  <p class="font-medium text-gray-900 dark:text-white">{{ result.title }}</p>
                  <p class="text-sm text-gray-500 dark:text-gray-400">{{ result.artist }}</p>
                </div>
                <Button variant="secondary" size="sm" @click="importSong(result)">
                  {{ $t('settings.import') }}
                </Button>
              </div>
            </div>

            <p v-else-if="searchDone && searchResults.length === 0" class="text-sm text-gray-500 dark:text-gray-400">
              {{ $t('settings.noResults') }}
            </p>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import { useUiStore } from '../stores/ui';
import { supportedLocales, type SupportedLocale } from '../i18n';

const { locale } = useI18n();
const uiStore = useUiStore();
const router = useRouter();

// Genius API
const geniusToken = ref(localStorage.getItem('geniusToken') ?? '');
const tokenSaved = ref(false);
const searchQuery = ref('');
const searching = ref(false);
const searchDone = ref(false);
const searchResults = ref<{ id: string; title: string; artist: string }[]>([]);

function changeLocale(code: string) {
  if (!supportedLocales.some(l => l.code === code)) return;
  locale.value = code as SupportedLocale;
  localStorage.setItem('locale', code);
}

function saveGeniusToken() {
  localStorage.setItem('geniusToken', geniusToken.value);
  tokenSaved.value = true;
  setTimeout(() => { tokenSaved.value = false; }, 2000);
}

async function searchGenius() {
  if (!searchQuery.value.trim()) return;
  searching.value = true;
  searchDone.value = false;
  try {
    // Call Tauri backend when available; for now use a placeholder
    // const results = await searchGeniusSongs(searchQuery.value, geniusToken.value);
    // searchResults.value = results;
    searchResults.value = [];
    searchDone.value = true;
  } catch (err) {
    console.error('Genius search failed:', err);
  } finally {
    searching.value = false;
  }
}

async function importSong(result: { id: string; title: string; artist: string }) {
  // Will be connected when Genius backend commands are available
  router.push({ path: '/songs/add', query: { title: result.title, artist: result.artist } });
}
</script>
