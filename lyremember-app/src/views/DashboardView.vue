<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          Dashboard
        </h1>
      </div>
      
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Total Songs</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">
                {{ songsStore.totalSongs }}
              </p>
            </div>
            <Music :size="40" class="text-indigo-600 dark:text-indigo-400" />
          </div>
        </Card>
        
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Practice Sessions</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">0</p>
            </div>
            <PlayCircle :size="40" class="text-green-600 dark:text-green-400" />
          </div>
        </Card>
        
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 dark:text-gray-400">Average Score</p>
              <p class="text-3xl font-bold text-gray-900 dark:text-white">-</p>
            </div>
            <TrendingUp :size="40" class="text-purple-600 dark:text-purple-400" />
          </div>
        </Card>
      </div>
      
      <Card>
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">Quick Actions</h2>
          </div>
        </template>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Button
            variant="primary"
            size="lg"
            className="w-full"
            @click="$router.push('/songs/add')"
          >
            <Plus :size="20" />
            Add New Song
          </Button>
          
          <Button
            variant="secondary"
            size="lg"
            className="w-full"
            @click="$router.push('/songs')"
          >
            <Music :size="20" />
            Browse Songs
          </Button>
        </div>
      </Card>
      
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">Recent Songs</h2>
        </template>
        
        <div v-if="songsStore.loading" class="text-center py-8">
          <p class="text-gray-500 dark:text-gray-400">Loading...</p>
        </div>
        
        <div v-else-if="songsStore.songs.length === 0" class="text-center py-8">
          <Music :size="48" class="mx-auto text-gray-400 mb-2" />
          <p class="text-gray-600 dark:text-gray-400">No songs yet</p>
          <p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
            Add your first song to get started!
          </p>
        </div>
        
        <div v-else class="space-y-2">
          <router-link
            v-for="song in recentSongs"
            :key="song.id"
            :to="`/songs/${song.id}`"
            class="block p-4 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          >
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-gray-900 dark:text-white">
                  {{ song.title }}
                </h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ song.artist }} • {{ song.language.toUpperCase() }}
                </p>
              </div>
              <ChevronRight :size="20" class="text-gray-400" />
            </div>
          </router-link>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { Music, PlayCircle, TrendingUp, Plus, ChevronRight } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import { useSongsStore } from '../stores/songs';

const songsStore = useSongsStore();

const recentSongs = computed(() => songsStore.songs.slice(0, 5));

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
});
</script>
