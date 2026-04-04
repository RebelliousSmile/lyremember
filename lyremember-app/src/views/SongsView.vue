<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h1 class="text-3xl font-bold text-[#F5F0EB]">
          {{ $t('songs.title') }}
        </h1>
        <Button variant="primary" @click="$router.push('/songs/add')">
          <Plus :size="20" />
          {{ $t('songs.addSong') }}
        </Button>
      </div>

      <Card>
        <div class="flex flex-col md:flex-row gap-4">
          <div class="flex-1">
            <Input
              v-model="songsStore.searchQuery"
              :placeholder="$t('songs.searchPlaceholder')"
              @input="songsStore.setSearchQuery($event.target.value)"
            >
              <template #prefix>
                <Search :size="20" />
              </template>
            </Input>
          </div>

          <select
            v-model="songsStore.selectedLanguage"
            @change="(e) => songsStore.setSelectedLanguage((e.target as HTMLSelectElement).value)"
            class="px-4 py-2 border border-deep-border rounded-lg bg-deep-card"
          >
            <option value="all">{{ $t('songs.allLanguages') }}</option>
            <option value="fr">{{ $t('songs.french') }}</option>
            <option value="en">{{ $t('songs.english') }}</option>
            <option value="jp">{{ $t('songs.japanese') }}</option>
            <option value="kr">{{ $t('songs.korean') }}</option>
          </select>
        </div>
      </Card>

      <div v-if="songsStore.loading">
        <Spinner :label="$t('common.loading')" />
      </div>

      <div v-else-if="songsStore.filteredSongs.length === 0" class="text-center py-12">
        <Music :size="64" class="mx-auto text-gray-400 mb-4" />
        <p class="text-xl text-[#8A82A0] mb-2">
          {{ songsStore.searchQuery ? $t('songs.noSongs') : $t('songs.noSongsYet') }}
        </p>
        <p class="text-[#8A82A0] mb-4">
          {{ songsStore.searchQuery ? $t('songs.tryDifferentSearch') : $t('songs.addFirstSongHint') }}
        </p>
        <Button v-if="!songsStore.searchQuery" variant="primary" @click="$router.push('/songs/add')">
          <Plus :size="20" />
          {{ $t('songs.addFirstSong') }}
        </Button>
      </div>

      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <Card
          v-for="song in songsStore.filteredSongs"
          :key="song.id"
          className="hover:shadow-lg transition-shadow cursor-pointer"
          @click="$router.push(`/songs/${song.id}`)"
        >
          <div class="space-y-3">
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h3 class="font-semibold text-lg text-[#F5F0EB] line-clamp-2">
                  {{ song.title }}
                </h3>
                <p class="text-sm text-[#8A82A0]">
                  {{ song.artist }}
                </p>
              </div>
              <span class="px-2 py-1 text-xs font-medium rounded-full bg-gold/10 text-gold">
                {{ song.language.toUpperCase() }}
              </span>
            </div>

            <div class="flex items-center gap-2 text-sm text-[#8A82A0]">
              <Hash :size="16" />
              {{ $t('songs.lines', { count: song.lyrics.length }) }}
            </div>

            <div class="flex gap-2 pt-2 border-t border-deep-border">
              <span
                v-if="song.phonetic_lyrics"
                class="inline-flex items-center gap-1 text-xs px-2 py-1 rounded bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-300"
              >
                <Check :size="12" />
                {{ $t('songs.phonetic') }}
              </span>
              <span
                v-if="song.translations"
                class="inline-flex items-center gap-1 text-xs px-2 py-1 rounded bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-300"
              >
                <Check :size="12" />
                {{ $t('songs.translation') }}
              </span>
            </div>
          </div>
        </Card>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { Music, Plus, Search, Hash, Check } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import Input from '../components/ui/Input.vue';
import Spinner from '../components/ui/Spinner.vue';
import { useSongsStore } from '../stores/songs';

const songsStore = useSongsStore();

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
});
</script>
