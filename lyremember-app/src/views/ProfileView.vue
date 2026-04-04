<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        Profile
      </h1>
      
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">User Information</h2>
        </template>
        
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Username
            </label>
            <p class="text-lg text-gray-900 dark:text-white">
              {{ authStore.user?.username }}
            </p>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Email
            </label>
            <p class="text-lg text-gray-900 dark:text-white">
              {{ authStore.user?.email }}
            </p>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Member Since
            </label>
            <p class="text-lg text-gray-900 dark:text-white">
              {{ formatDate(authStore.user?.created_at) }}
            </p>
          </div>
        </div>
      </Card>
      
      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">Statistics</h2>
        </template>
        
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-indigo-600 dark:text-indigo-400">
              {{ songsStore.totalSongs }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Total Songs
            </p>
          </div>

          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-green-600 dark:text-green-400">
              {{ userStats?.total_sessions ?? 0 }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Practice Sessions
            </p>
          </div>

          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-purple-600 dark:text-purple-400">
              {{ userStats && userStats.total_sessions > 0 ? Math.round(userStats.average_score) + '%' : '-' }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Average Score
            </p>
          </div>

          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-orange-600 dark:text-orange-400">
              {{ userStats ? formatDuration(userStats.total_practice_time) : '0m' }}
            </p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
              Practice Time
            </p>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import { useAuthStore } from '../stores/auth';
import { useSongsStore } from '../stores/songs';
import { getUserStats, type UserStats } from '../lib/tauri-api';

const authStore = useAuthStore();
const songsStore = useSongsStore();
const userStats = ref<UserStats | null>(null);

function formatDate(date: string | undefined) {
  if (!date) return 'N/A';
  return new Date(date).toLocaleDateString();
}

function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
  if (authStore.user) {
    try {
      userStats.value = await getUserStats(authStore.user.id);
    } catch (err) {
      console.error('Failed to fetch user stats:', err);
    }
  }
});
</script>
