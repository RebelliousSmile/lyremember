<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">Dashboard</h1>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatsCard label="Total Songs" :display="songsStore.totalSongs">
          <template #icon><Music :size="40" class="text-indigo-600 dark:text-indigo-400" /></template>
        </StatsCard>
        <StatsCard label="Practice Sessions" :display="stats?.total_sessions ?? 0">
          <template #icon><PlayCircle :size="40" class="text-green-600 dark:text-green-400" /></template>
        </StatsCard>
        <StatsCard label="Average Score" :display="averageScoreDisplay">
          <template #icon><TrendingUp :size="40" class="text-purple-600 dark:text-purple-400" /></template>
        </StatsCard>
      </div>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">Quick Actions</h2>
        </template>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Button variant="primary" size="lg" className="w-full" @click="$router.push('/songs/add')">
            <Plus :size="20" /> Add New Song
          </Button>
          <Button variant="secondary" size="lg" className="w-full" @click="$router.push('/songs')">
            <Music :size="20" /> Browse Songs
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
          <p class="text-sm text-gray-500 mt-1">Add your first song to get started!</p>
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
                <h3 class="font-semibold text-gray-900 dark:text-white">{{ song.title }}</h3>
                <p class="text-sm text-gray-600 dark:text-gray-400">
                  {{ song.artist }} · {{ song.language.toUpperCase() }}
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
import StatsCard from '../components/ui/StatsCard.vue';
import Button from '../components/ui/Button.vue';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';
import { useToast } from '../composables/useToast';

const songsStore = useSongsStore();
const { stats } = useUserStats();
const toast = useToast();

const RECENT_SONGS_LIMIT = 5;
const recentSongs = computed(() => songsStore.songs.slice(0, RECENT_SONGS_LIMIT));
const averageScoreDisplay = computed(() =>
  stats.value && stats.value.total_sessions > 0
    ? `${Math.round(stats.value.average_score)}%`
    : '-'
);

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch {
    toast.error('Failed to load songs');
  }
});
</script>
