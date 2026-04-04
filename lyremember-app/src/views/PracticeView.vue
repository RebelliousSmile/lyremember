<template>
  <MainLayout>
    <div class="space-y-6 pb-8">
      <!-- Header -->
      <div>
        <h1 class="text-[28px] font-bold text-gold" style="font-family: Georgia, 'Times New Roman', serif;">
          {{ $t('practice.title') }}
        </h1>
        <p class="text-[#8A82A0] mt-1">{{ $t('practice.chooseSong') }}</p>
      </div>

      <!-- Progress overview banner -->
      <div
        class="rounded-2xl p-5 border border-gold/20"
        style="background: linear-gradient(135deg, rgba(242, 169, 59, 0.10) 0%, rgba(123, 111, 160, 0.08) 100%);"
      >
        <div class="grid grid-cols-3 gap-4 text-center">
          <div>
            <p class="text-2xl font-bold text-gold" style="font-family: Georgia, 'Times New Roman', serif;">
              {{ userStats?.songs_practiced ?? 0 }}
            </p>
            <p class="text-xs text-[#8A82A0] mt-1">{{ $t('practice.songsPracticed') }}</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-gold" style="font-family: Georgia, 'Times New Roman', serif;">
              {{ userStats && userStats.total_sessions > 0 ? Math.round(userStats.average_score) + '%' : '-' }}
            </p>
            <p class="text-xs text-[#8A82A0] mt-1">{{ $t('practice.averageScore') }}</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-gold" style="font-family: Georgia, 'Times New Roman', serif;">
              {{ userStats?.total_sessions ?? 0 }}
            </p>
            <p class="text-xs text-[#8A82A0] mt-1">{{ $t('practice.totalSessions') }}</p>
          </div>
        </div>
      </div>

      <!-- Loading state -->
      <div v-if="songsStore.loading">
        <Spinner :label="$t('common.loading')" />
      </div>

      <!-- Empty state -->
      <div v-else-if="songsStore.songs.length === 0" class="text-center py-12">
        <Music :size="48" class="mx-auto text-[#8A82A0] mb-3" />
        <p class="text-[#8A82A0]">{{ $t('practice.noSongsInRepertoire') }}</p>
        <p class="text-sm text-[#8A82A0] mt-1">
          {{ $t('practice.addSongsFirst') }}
        </p>
        <Button variant="primary" className="mt-4" @click="$router.push('/songs/add')">
          <Plus :size="18" />
          {{ $t('practice.addASong') }}
        </Button>
      </div>

      <!-- Song list & mode selection -->
      <div v-else class="space-y-3">
        <div
          v-for="song in songsStore.songs"
          :key="song.id"
          class="space-y-3"
        >
          <!-- Song card -->
          <div
            class="bg-deep-card rounded-2xl p-4 border border-deep-border hover:border-gold/40 transition-colors cursor-pointer"
            @click="toggleSong(song.id)"
          >
            <div class="flex items-center gap-3">
              <!-- Cover placeholder -->
              <div
                class="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
                style="background: linear-gradient(135deg, #7B6FA0, #5E5480);"
              >
                <Music :size="22" class="text-white/70" />
              </div>

              <!-- Song info -->
              <div class="flex-1 min-w-0">
                <h3 class="font-bold text-[#F5F0EB] truncate">{{ song.title }}</h3>
                <p class="text-sm text-[#B8B0D0] truncate">{{ song.artist }}</p>
              </div>

              <!-- Mastery percentage -->
              <div class="text-right flex-shrink-0">
                <p class="text-lg font-bold text-gold">
                  {{ getMasteryPercent(song.id) }}%
                </p>
                <p class="text-xs text-[#8A82A0]">mémorisé</p>
              </div>
            </div>
          </div>

          <!-- Practice mode cards (2x2 grid) -->
          <div
            v-if="selectedSongId === song.id"
            class="grid grid-cols-2 gap-3 pl-2 pr-2"
          >
            <!-- Texte à trous -->
            <div
              class="bg-deep-card rounded-2xl p-5 border border-deep-border hover:border-[#F59E0B]/40 transition-colors cursor-pointer"
              @click="startPractice(song.id, 'fill-blank')"
            >
              <div class="w-10 h-10 rounded-full bg-[#F59E0B]/15 flex items-center justify-center mb-3">
                <PenLine :size="20" class="text-[#F59E0B]" />
              </div>
              <p class="font-bold text-[#F5F0EB] text-sm">{{ $t('practice.fillBlank') }}</p>
              <p class="text-xs text-[#8A82A0] mt-1">Complétez les paroles manquantes</p>
            </div>

            <!-- Première lettre (oral) -->
            <div
              class="bg-deep-card rounded-2xl p-5 border border-deep-border hover:border-[#10B981]/40 transition-colors cursor-pointer"
              @click="startPractice(song.id, 'oral')"
            >
              <div class="w-10 h-10 rounded-full bg-[#10B981]/15 flex items-center justify-center mb-3">
                <Mic :size="20" class="text-[#10B981]" />
              </div>
              <p class="font-bold text-[#F5F0EB] text-sm">{{ $t('practice.oral') }}</p>
              <p class="text-xs text-[#8A82A0] mt-1">Récitez les paroles à voix haute</p>
            </div>

            <!-- Karaoké -->
            <div
              class="bg-deep-card rounded-2xl p-5 border border-deep-border hover:border-[#EC4899]/40 transition-colors cursor-pointer"
              @click="startPractice(song.id, 'karaoke')"
            >
              <div class="w-10 h-10 rounded-full bg-[#EC4899]/15 flex items-center justify-center mb-3">
                <PlayCircle :size="20" class="text-[#EC4899]" />
              </div>
              <p class="font-bold text-[#F5F0EB] text-sm">{{ $t('practice.karaoke') }}</p>
              <p class="text-xs text-[#8A82A0] mt-1">Suivez les paroles en rythme</p>
            </div>

            <!-- QCM -->
            <div
              class="bg-deep-card rounded-2xl p-5 border border-deep-border hover:border-[#6366F1]/40 transition-colors cursor-pointer"
              @click="startPractice(song.id, 'mcq')"
            >
              <div class="w-10 h-10 rounded-full bg-[#6366F1]/15 flex items-center justify-center mb-3">
                <List :size="20" class="text-[#6366F1]" />
              </div>
              <p class="font-bold text-[#F5F0EB] text-sm">{{ $t('practice.mcq') }}</p>
              <p class="text-xs text-[#8A82A0] mt-1">Choisissez la bonne réponse</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { PlayCircle, Music, TrendingUp, PenLine, List, Mic, Plus } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import Spinner from '../components/ui/Spinner.vue';
import { useSongsStore } from '../stores/songs';
import { useUserStats } from '../composables/useUserStats';

const router = useRouter();
const songsStore = useSongsStore();
const { userStats } = useUserStats();

const selectedSongId = ref<string | null>(null);

function toggleSong(songId: string) {
  selectedSongId.value = selectedSongId.value === songId ? null : songId;
}

function getMasteryPercent(songId: string): number {
  // Placeholder: derive from userStats or return 0
  return 0;
}

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
