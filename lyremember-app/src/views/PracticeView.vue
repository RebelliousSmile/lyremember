<template>
  <MainLayout>
    <div class="space-y-6">
      <div class="flex items-center gap-4">
        <Button variant="ghost" @click="handleBack">
          <ArrowLeft :size="20" />
        </Button>
        <div class="flex-1">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ practiceStore.song?.title }}
          </h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            {{ modeLabel }} · Line {{ currentLine }} / {{ totalLines }}
          </p>
        </div>
        <div class="text-right">
          <p class="text-2xl font-bold text-indigo-600 dark:text-indigo-400">{{ practiceStore.score }}%</p>
          <p class="text-xs text-gray-500">Score</p>
        </div>
      </div>

      <!-- Progress bar -->
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-indigo-600 h-2 rounded-full transition-all duration-300"
          :style="{ width: `${progress}%` }"
        />
      </div>

      <template v-if="practiceStore.song && practiceStore.state && practiceStore.mode">
        <!-- Practice mode content -->
        <div v-if="!practiceStore.finished">
          <KaraokeMode
            v-if="practiceStore.mode === 'karaoke'"
            :song="practiceStore.song"
            :current-line="currentLine"
            @answer="practiceStore.answerLine"
          />
          <FillBlankMode
            v-else-if="practiceStore.mode === 'fill-blank'"
            :song="practiceStore.song"
            :current-line="currentLine"
            @answer="practiceStore.answerLine"
          />
          <MCQMode
            v-else-if="practiceStore.mode === 'mcq'"
            :song="practiceStore.song"
            :current-line="currentLine"
            @answer="practiceStore.answerLine"
          />
        </div>

        <!-- Result screen -->
        <PracticeResult
          v-else
          :score="practiceStore.score"
          :correct="practiceStore.state.correctLines"
          :total="practiceStore.state.totalLines"
          :mode="practiceStore.mode"
          :song-title="practiceStore.song.title"
          @retry="retry"
          @back="goToSong"
        />
      </template>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ArrowLeft } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Button from '../components/ui/Button.vue';
import KaraokeMode from '../components/practice/KaraokeMode.vue';
import FillBlankMode from '../components/practice/FillBlankMode.vue';
import MCQMode from '../components/practice/MCQMode.vue';
import PracticeResult from '../components/practice/PracticeResult.vue';
import { usePracticeStore, MODE_LABELS } from '../stores/practice';
import { useSongsStore } from '../stores/songs';
import { useToast } from '../composables/useToast';
import type { PracticeMode } from '../stores/practice';

const route = useRoute();
const router = useRouter();
const practiceStore = usePracticeStore();
const songsStore = useSongsStore();
const toast = useToast();

const currentLine = computed(() => (practiceStore.state?.currentLine ?? 0) + 1);
const totalLines = computed(() => practiceStore.state?.totalLines ?? 0);
const progress = computed(() => totalLines.value > 0
  ? (currentLine.value - 1) / totalLines.value * 100
  : 0
);
const modeLabel = computed(() => practiceStore.mode ? MODE_LABELS[practiceStore.mode] : '');

onMounted(async () => {
  const songId = String(route.params.songId);
  const mode = String(route.params.mode) as PracticeMode;

  if (!['karaoke', 'fill-blank', 'mcq'].includes(mode)) {
    router.push(`/songs/${songId}`);
    return;
  }

  try {
    const song = await songsStore.fetchSong(songId);
    if (!song) throw new Error('Song not found');
    practiceStore.startSession(song, mode);
  } catch {
    toast.error('Failed to load song');
    router.push('/songs');
  }
});

onUnmounted(() => {
  if (practiceStore.finished) {
    practiceStore.saveSession().catch(() => {
      toast.error('Could not save practice session');
    });
  }
});

function handleBack() {
  if (practiceStore.finished || confirm('Quit practice? Progress will be lost.')) {
    practiceStore.reset();
    router.back();
  }
}

function retry() {
  if (practiceStore.song && practiceStore.mode) {
    practiceStore.startSession(practiceStore.song, practiceStore.mode);
  }
}

function goToSong() {
  const songId = practiceStore.song?.id;
  practiceStore.reset();
  router.push(`/songs/${songId}`);
}
</script>
