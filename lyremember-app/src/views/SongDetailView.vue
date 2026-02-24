<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center gap-4">
        <Button
          variant="ghost"
          @click="$router.back()"
        >
          <ArrowLeft :size="20" />
        </Button>
        <div class="flex-1">
          <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
            {{ song?.title }}
          </h1>
          <p class="text-gray-600 dark:text-gray-400">
            {{ song?.artist }}
          </p>
        </div>
        <span class="px-3 py-1 rounded-full bg-indigo-100 dark:bg-indigo-900/30 text-indigo-800 dark:text-indigo-300">
          {{ song?.language.toUpperCase() }}
        </span>
      </div>
      
      <div v-if="loading" class="text-center py-12">
        <p class="text-gray-500 dark:text-gray-400">Loading song...</p>
      </div>
      
      <div v-else-if="!song" class="text-center py-12">
        <p class="text-red-600 dark:text-red-400">Song not found</p>
      </div>
      
      <div v-else class="space-y-6">
        <Card>
          <template #header>
            <h2 class="text-xl font-semibold">Lyrics</h2>
          </template>
          
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Original Lyrics -->
            <div>
              <h3 class="font-semibold text-sm text-gray-600 dark:text-gray-400 mb-3">
                Original ({{ song.language.toUpperCase() }})
              </h3>
              <div class="space-y-2">
                <p
                  v-for="(line, index) in song.lyrics"
                  :key="`original-${index}`"
                  class="text-lg leading-relaxed"
                >
                  {{ line }}
                </p>
              </div>
            </div>
            
            <!-- Phonetic -->
            <div v-if="song.phonetic_lyrics">
              <h3 class="font-semibold text-sm text-gray-600 dark:text-gray-400 mb-3">
                Phonetic
              </h3>
              <div class="space-y-2">
                <p
                  v-for="(line, index) in song.phonetic_lyrics"
                  :key="`phonetic-${index}`"
                  class="text-lg leading-relaxed text-gray-600 dark:text-gray-400 italic"
                >
                  {{ line }}
                </p>
              </div>
            </div>
            
            <!-- Translation -->
            <div v-if="song.translations && song.translations.en">
              <h3 class="font-semibold text-sm text-gray-600 dark:text-gray-400 mb-3">
                English Translation
              </h3>
              <div class="space-y-2">
                <p
                  v-for="(line, index) in song.translations.en"
                  :key="`translation-${index}`"
                  class="text-lg leading-relaxed text-gray-600 dark:text-gray-400"
                >
                  {{ line }}
                </p>
              </div>
            </div>
          </div>
        </Card>
        
        <Card>
          <template #header>
            <h2 class="text-xl font-semibold">Practice Modes</h2>
          </template>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <Button variant="primary" size="lg" className="w-full">
              <PlayCircle :size="20" />
              Karaoke Mode
            </Button>
            <Button variant="secondary" size="lg" className="w-full">
              <Music :size="20" />
              Fill-in-the-Blank
            </Button>
            <Button variant="secondary" size="lg" className="w-full">
              <List :size="20" />
              Multiple Choice
            </Button>
            <Button variant="secondary" size="lg" className="w-full">
              <Mic :size="20" />
              Oral Practice
            </Button>
          </div>
        </Card>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { ArrowLeft, PlayCircle, Music, List, Mic } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import { useSongsStore } from '../stores/songs';
import type { Song } from '../types';

const route = useRoute();
const songsStore = useSongsStore();

const song = ref<Song | null>(null);
const loading = ref(true);

onMounted(async () => {
  try {
    const songId = String(route.params.id);
    song.value = await songsStore.fetchSong(songId);
  } catch (err) {
    console.error('Failed to fetch song:', err);
  } finally {
    loading.value = false;
  }
});
</script>
