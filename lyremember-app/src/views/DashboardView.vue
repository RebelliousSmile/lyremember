<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <h1 class="text-3xl font-bold text-[#F5F0EB]">
          {{ $t('dashboard.title') }}
        </h1>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-[#8A82A0]">{{ $t('dashboard.totalSongs') }}</p>
              <p class="text-3xl font-bold text-[#F5F0EB]">
                {{ songsStore.totalSongs }}
              </p>
            </div>
            <Music :size="40" class="text-gold" />
          </div>
        </Card>

        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-[#8A82A0]">{{ $t('dashboard.practiceSessions') }}</p>
              <p class="text-3xl font-bold text-[#F5F0EB]">{{ totalSessions }}</p>
            </div>
            <PlayCircle :size="40" class="text-green-600 dark:text-green-400" />
          </div>
        </Card>

        <Card>
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-[#8A82A0]">{{ $t('dashboard.averageScore') }}</p>
              <p class="text-3xl font-bold text-[#F5F0EB]">{{ averageScore }}</p>
            </div>
            <TrendingUp :size="40" class="text-purple-600 dark:text-purple-400" />
          </div>
        </Card>
      </div>

      <Card>
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">{{ $t('dashboard.quickActions') }}</h2>
          </div>
        </template>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Button variant="primary" size="lg" className="w-full" @click="$router.push('/songs/add')">
            <Plus :size="20" />
            {{ $t('dashboard.addNewSong') }}
          </Button>

          <Button variant="secondary" size="lg" className="w-full" @click="$router.push('/songs')">
            <Music :size="20" />
            {{ $t('dashboard.browseSongs') }}
          </Button>
        </div>
      </Card>

      <Card>
        <template #header>
          <h2 class="text-xl font-semibold">{{ $t('dashboard.recentSongs') }}</h2>
        </template>

        <div v-if="songsStore.loading">
          <Spinner :label="$t('common.loading')" />
        </div>

        <div v-else-if="songsStore.songs.length === 0" class="text-center py-8">
          <Music :size="48" class="mx-auto text-gray-400 mb-2" />
          <p class="text-[#8A82A0]">{{ $t('dashboard.noSongsYet') }}</p>
          <p class="text-sm text-gray-500 dark:text-gray-500 mt-1">
            {{ $t('dashboard.addFirstSong') }}
          </p>
        </div>

        <div v-else class="space-y-2">
          <router-link
            v-for="song in recentSongs"
            :key="song.id"
            :to="`/songs/${song.id}`"
            class="block p-4 rounded-lg hover:bg-deep-card-hover transition-colors"
          >
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-semibold text-[#F5F0EB]">{{ song.title }}</h3>
                <p class="text-sm text-[#8A82A0]">
                  {{ song.artist }} - {{ song.language.toUpperCase() }}
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
import Spinner from '../components/ui/Spinner.vue';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';

const songsStore = useSongsStore();
const { userStats } = useUserStats();

const recentSongs = computed(() => songsStore.songs.slice(0, 5));
const totalSessions = computed(() => userStats.value?.total_sessions ?? 0);
const averageScore = computed(() => {
  if (!userStats.value || userStats.value.total_sessions === 0) return '-';
  return `${Math.round(userStats.value.average_score)}%`;
});

onMounted(async () => {
  try {
    await songsStore.fetchUserSongs();
  } catch (err) {
    console.error('Failed to fetch songs:', err);
  }
});
</script>
