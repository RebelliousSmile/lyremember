<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          Practice
        </h1>
      </div>

      <!-- Stats overview -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Total Sessions</p>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">
                {{ userStats?.total_sessions ?? 0 }}
              </p>
            </div>
            <PlayCircle :size="32" class="text-green-600 dark:text-green-400" />
          </div>
        </Card>
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Songs Practiced</p>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">
                {{ userStats?.songs_practiced ?? 0 }}
              </p>
            </div>
            <Music :size="32" class="text-indigo-600 dark:text-indigo-400" />
          </div>
        </Card>
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Average Score</p>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">
                {{ userStats && userStats.total_sessions > 0 ? Math.round(userStats.average_score) + '%' : '-' }}
              </p>
            </div>
            <TrendingUp :size="32" class="text-purple-600 dark:text-purple-400" />
          </div>
        </Card>
      </div>

      <!-- Song selection -->
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">Choose a Song to Practice</h2>
        </template>

        <div v-if="songsStore.loading" class="text-center py-8">
          <p class="text-gray-500 dark:text-gray-400">Loading songs...</p>
        </div>

        <div v-else-if="songsStore.songs.length === 0" class="text-center py-8">
          <Music :size="48" class="mx-auto text-gray-400 mb-2" />
          <p class="text-gray-600 dark:text-gray-400">No songs in your repertoire</p>
          <p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
            Add songs first to start practicing!
          </p>
          <Button
            variant="primary"
            className="mt-4"
            @click="$router.push('/songs/add')"
          >
            <Plus :size="18" />
            Add a Song
          </Button>
        </div>

        <div v-else class="space-y-3">
          <div
            v-for="song in songsStore.songs"
            :key="song.id"
            class="p-4 rounded-lg border border-gray-200 dark:border-gray-700 hover:border-indigo-300 dark:hover:border-indigo-600 transition-colors"
          >
            <div class="flex items-center justify-between mb-3">
              <div>
                <h3 class="font-semibold text-gray-900 dark:text-white">
                  {{ song.title }}
                </h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ song.artist }}
                </p>
              </div>
              <span class="px-2 py-1 text-xs rounded-full bg-indigo-100 dark:bg-indigo-900/30 text-indigo-800 dark:text-indigo-300">
                {{ song.language.toUpperCase() }}
              </span>
            </div>
            <div class="flex flex-wrap gap-2">
              <Button
                variant="primary"
                size="sm"
                @click="startPractice(song.id, 'karaoke')"
              >
                <PlayCircle :size="16" />
                Karaoke
              </Button>
              <Button
                variant="secondary"
                size="sm"
                @click="startPractice(song.id, 'fill-blank')"
              >
                <PenLine :size="16" />
                Fill-in-Blank
              </Button>
              <Button
                variant="secondary"
                size="sm"
                @click="startPractice(song.id, 'mcq')"
              >
                <List :size="16" />
                MCQ
              </Button>
              <Button
                variant="secondary"
                size="sm"
                @click="startPractice(song.id, 'oral')"
              >
                <Mic :size="16" />
                Oral
              </Button>
            </div>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { PlayCircle, Music, TrendingUp, PenLine, List, Mic, Plus } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';

const router = useRouter();
const songsStore = useSongsStore();
const { userStats } = useUserStats();

function startPractice(songId: string, mode: string) {
  router.push({ path: `/songs/${songId}`, query: { mode } });
}

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
});
</script>
