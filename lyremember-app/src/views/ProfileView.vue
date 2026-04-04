<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <h1 class="text-3xl font-bold text-[#F5F0EB]">
        {{ $t('profile.title') }}
      </h1>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('profile.userInfo') }}</h2>
        </template>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('profile.username') }}
            </label>
            <p class="text-lg text-[#F5F0EB]">
              {{ authStore.user?.username }}
            </p>
          </div>

          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('profile.email') }}
            </label>
            <p class="text-lg text-[#F5F0EB]">
              {{ authStore.user?.email }}
            </p>
          </div>

          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('profile.memberSince') }}
            </label>
            <p class="text-lg text-[#F5F0EB]">
              {{ formatDate(authStore.user?.created_at) }}
            </p>
          </div>
        </div>
      </Card>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('profile.statistics') }}</h2>
        </template>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center p-4 bg-deep-card-hover rounded-lg">
            <p class="text-3xl font-bold text-gold">
              {{ songsStore.totalSongs }}
            </p>
            <p class="text-sm text-[#8A82A0] mt-1">
              {{ $t('profile.totalSongs') }}
            </p>
          </div>

          <div class="text-center p-4 bg-deep-card-hover rounded-lg">
            <p class="text-3xl font-bold text-green-600 dark:text-green-400">
              {{ userStats?.total_sessions ?? 0 }}
            </p>
            <p class="text-sm text-[#8A82A0] mt-1">
              {{ $t('profile.practiceSessions') }}
            </p>
          </div>

          <div class="text-center p-4 bg-deep-card-hover rounded-lg">
            <p class="text-3xl font-bold text-purple-600 dark:text-purple-400">
              {{ userStats && userStats.total_sessions > 0 ? Math.round(userStats.average_score) + '%' : '-' }}
            </p>
            <p class="text-sm text-[#8A82A0] mt-1">
              {{ $t('profile.averageScore') }}
            </p>
          </div>

          <div class="text-center p-4 bg-deep-card-hover rounded-lg">
            <p class="text-3xl font-bold text-orange-600 dark:text-orange-400">
              {{ userStats ? formatDuration(userStats.total_practice_time) : '0m' }}
            </p>
            <p class="text-sm text-[#8A82A0] mt-1">
              {{ $t('profile.practiceTime') }}
            </p>
          </div>
        </div>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import { useAuthStore } from '../stores/auth';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';

const authStore = useAuthStore();
const songsStore = useSongsStore();
const { userStats } = useUserStats();

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
});
</script>
