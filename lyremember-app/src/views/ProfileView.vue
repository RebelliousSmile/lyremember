<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">Profile</h1>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">User Information</h2>
        </template>
        <div v-if="authStore.user" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Username</label>
            <p class="text-lg text-gray-900 dark:text-white">{{ authStore.user.username }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
            <p class="text-lg text-gray-900 dark:text-white">{{ authStore.user.email }}</p>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Member Since</label>
            <p class="text-lg text-gray-900 dark:text-white">{{ formatDate(authStore.user.created_at) }}</p>
          </div>
        </div>
      </Card>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">Statistics</h2>
        </template>
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-indigo-600 dark:text-indigo-400">{{ songsStore.totalSongs }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Total Songs</p>
          </div>
          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-green-600 dark:text-green-400">{{ stats?.total_sessions ?? 0 }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Sessions</p>
          </div>
          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-purple-600 dark:text-purple-400">{{ averageScoreDisplay }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Avg Score</p>
          </div>
          <div class="text-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
            <p class="text-3xl font-bold text-orange-600 dark:text-orange-400">{{ practiceTimeDisplay }}</p>
            <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">Practice Time</p>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import { useAuthStore } from '../stores/auth';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';
import { useToast } from '../composables/useToast';

const authStore = useAuthStore();
const songsStore = useSongsStore();
const { stats } = useUserStats();
const toast = useToast();

const averageScoreDisplay = computed(() =>
  stats.value && stats.value.total_sessions > 0
    ? `${Math.round(stats.value.average_score)}%`
    : '-'
);

const practiceTimeDisplay = computed(() => {
  if (!stats.value) return '0m';
  const seconds = stats.value.total_practice_time;
  if (seconds < 60) return `${seconds}s`;
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  return `${hours}h ${minutes % 60}m`;
});

function formatDate(date: string) {
  return new Date(date).toLocaleDateString();
}

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch {
    toast.error('Failed to load songs');
  }
});
</script>
