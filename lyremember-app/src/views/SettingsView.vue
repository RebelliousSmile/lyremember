<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <h1 class="text-3xl font-bold text-[#F5F0EB]">
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
              <p class="font-medium text-[#F5F0EB]">{{ $t('settings.darkMode') }}</p>
              <p class="text-sm text-[#8A82A0]">{{ $t('settings.darkModeDesc') }}</p>
            </div>
            <button
              class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors"
              :class="uiStore.darkMode ? 'bg-gold' : 'bg-gray-300'"
              @click="uiStore.toggleDarkMode()"
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
              <p class="font-medium text-[#F5F0EB]">{{ $t('settings.language') }}</p>
              <p class="text-sm text-[#8A82A0]">{{ $t('settings.languageDesc') }}</p>
            </div>
            <select
              :value="locale"
              class="rounded-lg border border-deep-border bg-deep-card-hover px-3 py-2 text-sm text-[#F5F0EB]"
              @change="changeLocale(($event.target as HTMLSelectElement).value)"
            >
              <option v-for="lang in supportedLocales" :key="lang.code" :value="lang.code">
                {{ lang.label }}
              </option>
            </select>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import { useUiStore } from '../stores/ui';
import { supportedLocales, type SupportedLocale } from '../i18n';

const { locale } = useI18n();
const uiStore = useUiStore();

function changeLocale(code: string) {
  if (!supportedLocales.some((l) => l.code === code)) return;
  locale.value = code as SupportedLocale;
  localStorage.setItem('locale', code);
}
</script>
