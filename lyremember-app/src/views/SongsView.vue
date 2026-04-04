<template>
  <MainLayout>
    <div class="space-y-5">
      <!-- Header: "Ma Lyre" title + subtitle + search toggle -->
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold">
            <span class="text-[#F5F0EB]">Ma</span>
            <span class="text-gold">Lyre</span>
          </h1>
          <p class="text-sm text-[#B8B0D0] mt-1">
            {{ songsStore.totalSongs }} {{ $t('songs.title').toLowerCase() }} · {{ uniqueLanguages.length }} {{ uniqueLanguages.length === 1 ? 'langue' : 'langues' }}
          </p>
        </div>
        <button
          @click="showSearch = !showSearch"
          class="w-10 h-10 flex items-center justify-center rounded-full bg-deep-card border border-deep-border text-[#B8B0D0] hover:text-[#F5F0EB] transition-colors"
        >
          <Search :size="20" />
        </button>
      </div>

      <!-- Search bar (toggle) -->
      <div v-if="showSearch" class="relative">
        <input
          ref="searchInput"
          v-model="songsStore.searchQuery"
          :placeholder="$t('songs.searchPlaceholder')"
          @input="songsStore.setSearchQuery(($event.target as HTMLInputElement).value)"
          class="w-full px-4 py-2.5 pl-10 rounded-xl bg-deep-card border border-deep-border text-[#F5F0EB] placeholder-[#8A82A0] focus:outline-none focus:border-gold/50 transition-colors"
        />
        <Search :size="18" class="absolute left-3 top-1/2 -translate-y-1/2 text-[#8A82A0]" />
      </div>

      <!-- Language filter pills -->
      <div class="flex gap-2 overflow-x-auto pb-1 scrollbar-hide">
        <button
          v-for="lang in languageFilters"
          :key="lang.value"
          @click="songsStore.setSelectedLanguage(lang.value)"
          :class="[
            'px-4 py-1.5 rounded-full text-sm font-medium whitespace-nowrap transition-colors flex-shrink-0',
            songsStore.selectedLanguage === lang.value
              ? 'bg-gold text-deep'
              : 'bg-deep-card border border-deep-border text-[#B8B0D0] hover:border-gold/30'
          ]"
        >
          {{ lang.label }}
        </button>
      </div>

      <!-- Loading state -->
      <div v-if="songsStore.loading">
        <Spinner :label="$t('common.loading')" />
      </div>

      <!-- Empty state -->
      <div v-else-if="songsStore.filteredSongs.length === 0" class="text-center py-12">
        <Music :size="64" class="mx-auto text-[#8A82A0] mb-4" />
        <p class="text-xl text-[#8A82A0] mb-2">
          {{ songsStore.searchQuery ? $t('songs.noSongs') : $t('songs.noSongsYet') }}
        </p>
        <p class="text-[#8A82A0] mb-4">
          {{ songsStore.searchQuery ? $t('songs.tryDifferentSearch') : $t('songs.addFirstSongHint') }}
        </p>
        <button
          v-if="!songsStore.searchQuery"
          @click="$router.push('/songs/add')"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gold text-deep font-medium hover:bg-gold-light transition-colors"
        >
          <Plus :size="20" />
          {{ $t('songs.addFirstSong') }}
        </button>
      </div>

      <!-- Song list (vertical) -->
      <div v-else class="space-y-2">
        <div
          v-for="song in songsStore.filteredSongs"
          :key="song.id"
          @click="$router.push(`/songs/${song.id}`)"
          class="flex items-center gap-3 p-3 rounded-xl bg-deep-card hover:bg-deep-card-hover cursor-pointer transition-colors"
        >
          <!-- Cover: gradient violet bg with first letter -->
          <div class="w-[50px] h-[50px] flex-shrink-0 rounded-xl bg-gradient-to-br from-violet-accent to-[#5E5480] flex items-center justify-center">
            <span class="text-xl font-bold text-[#F5F0EB]">
              {{ langEmoji(song.language) || song.title.charAt(0).toUpperCase() }}
            </span>
          </div>

          <!-- Center: title, badge, artist, progress bar -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <span class="font-semibold text-[#F5F0EB] truncate">{{ song.title }}</span>
              <span class="flex-shrink-0 px-1.5 py-0.5 text-[10px] font-semibold rounded bg-gold/15 text-gold uppercase leading-none">
                {{ langFlag(song.language) }} {{ song.language.toUpperCase() }}
              </span>
            </div>
            <p class="text-sm text-[#8A82A0] truncate mt-0.5">{{ song.artist }}</p>
            <!-- Mini progress bar -->
            <div class="mt-1.5 h-[3px] w-full rounded-full bg-deep-border overflow-hidden">
              <div
                class="h-full rounded-full bg-gold transition-all"
                :style="{ width: songProgress(song) + '%' }"
              />
            </div>
          </div>

          <!-- Progress percentage -->
          <span class="text-sm font-medium text-[#B8B0D0] flex-shrink-0 ml-2">
            {{ songProgress(song) }}%
          </span>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue';
import { Music, Plus, Search } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Spinner from '../components/ui/Spinner.vue';
import { useSongsStore } from '../stores/songs';
import type { Song } from '../types';

const songsStore = useSongsStore();
const showSearch = ref(false);
const searchInput = ref<HTMLInputElement | null>(null);

// Auto-focus search input when toggled open
watch(showSearch, async (val) => {
  if (val) {
    await nextTick();
    searchInput.value?.focus();
  }
});

const languageFilters = [
  { value: 'all', label: 'Tout' },
  { value: 'en', label: '\uD83C\uDDEC\uD83C\uDDE7 EN' },
  { value: 'fr', label: '\uD83C\uDDEB\uD83C\uDDF7 FR' },
  { value: 'jp', label: '\uD83C\uDDEF\uD83C\uDDF5 JP' },
  { value: 'kr', label: '\uD83C\uDDF0\uD83C\uDDF7 KR' },
];

const uniqueLanguages = computed(() => {
  const langs = new Set(songsStore.songs.map((s: Song) => s.language));
  return Array.from(langs);
});

function langFlag(language: string): string {
  const flags: Record<string, string> = {
    en: '\uD83C\uDDEC\uD83C\uDDE7',
    fr: '\uD83C\uDDEB\uD83C\uDDF7',
    jp: '\uD83C\uDDEF\uD83C\uDDF5',
    kr: '\uD83C\uDDF0\uD83C\uDDF7',
  };
  return flags[language] || '';
}

function langEmoji(language: string): string {
  return langFlag(language);
}

function songProgress(song: Song): number {
  // Estimate progress based on available enrichments
  let progress = 0;
  if (song.phonetic_lyrics) progress += 50;
  if (song.translations) progress += 50;
  return progress;
}

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
});
</script>

<style scoped>
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>
