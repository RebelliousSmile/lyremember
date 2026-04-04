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
          <h1 class="text-3xl font-bold text-[#F5F0EB]">
            {{ song?.title }}
          </h1>
          <p class="text-[#8A82A0]">
            {{ song?.artist }}
          </p>
        </div>
        <span class="px-3 py-1 rounded-full bg-gold/10 text-gold">
          {{ song?.language.toUpperCase() }}
        </span>
      </div>

      <div v-if="loading">
        <Spinner :label="$t('songDetail.loadingSong')" />
      </div>

      <div v-else-if="!song" class="text-center py-12">
        <p class="text-red-600 dark:text-red-400">{{ $t('songDetail.songNotFound') }}</p>
      </div>

      <div v-else class="space-y-6">
        <!-- Practice mode active -->
        <div v-if="activeMode">
          <Card>
            <template #header>
              <div class="flex items-center justify-between">
                <h2 class="text-xl font-semibold">{{ modeTitles[activeMode] }}</h2>
                <Button variant="ghost" size="sm" @click="closeMode">
                  <X :size="18" />
                  {{ $t('common.close') }}
                </Button>
              </div>
            </template>

            <KaraokeMode
              v-if="activeMode === 'karaoke'"
              :song="song"
              @finish="onPracticeFinish"
            />
            <FillBlankMode
              v-else-if="activeMode === 'fill-blank'"
              :song="song"
              @finish="onPracticeFinish"
            />
            <McqMode
              v-else-if="activeMode === 'mcq'"
              :song="song"
              @finish="onPracticeFinish"
            />
            <OralMode
              v-else-if="activeMode === 'oral'"
              :song="song"
              @finish="onPracticeFinish"
            />
          </Card>
        </div>

        <!-- Normal view (lyrics + mode selection) -->
        <template v-else>
          <Card>
            <template #header>
              <h2 class="text-xl font-semibold">{{ $t('songDetail.lyrics') }}</h2>
            </template>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
              <!-- Original Lyrics -->
              <div>
                <h3 class="font-semibold text-sm text-[#8A82A0] mb-3">
                  {{ $t('songDetail.original') }} ({{ song.language.toUpperCase() }})
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
                <h3 class="font-semibold text-sm text-[#8A82A0] mb-3">
                  {{ $t('songDetail.phonetic') }}
                </h3>
                <div class="space-y-2">
                  <p
                    v-for="(line, index) in song.phonetic_lyrics"
                    :key="`phonetic-${index}`"
                    class="text-lg leading-relaxed text-[#8A82A0] italic"
                  >
                    {{ line }}
                  </p>
                </div>
              </div>

              <!-- Translation -->
              <div v-if="song.translations && song.translations.en">
                <h3 class="font-semibold text-sm text-[#8A82A0] mb-3">
                  {{ $t('songDetail.englishTranslation') }}
                </h3>
                <div class="space-y-2">
                  <p
                    v-for="(line, index) in song.translations.en"
                    :key="`translation-${index}`"
                    class="text-lg leading-relaxed text-[#8A82A0]"
                  >
                    {{ line }}
                  </p>
                </div>
              </div>
            </div>
          </Card>

          <Card>
            <template #header>
              <h2 class="text-xl font-semibold">{{ $t('songDetail.practiceModes') }}</h2>
            </template>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Button variant="primary" size="lg" className="w-full" @click="startMode('karaoke')">
                <PlayCircle :size="20" />
                {{ $t('songDetail.karaokeMode') }}
              </Button>
              <Button variant="secondary" size="lg" className="w-full" @click="startMode('fill-blank')">
                <PenLine :size="20" />
                {{ $t('songDetail.fillBlank') }}
              </Button>
              <Button variant="secondary" size="lg" className="w-full" @click="startMode('mcq')">
                <List :size="20" />
                {{ $t('songDetail.mcq') }}
              </Button>
              <Button variant="secondary" size="lg" className="w-full" @click="startMode('oral')">
                <Mic :size="20" />
                {{ $t('songDetail.oralPractice') }}
              </Button>
            </div>
          </Card>
        </template>
      </div>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { ArrowLeft, PlayCircle, PenLine, List, Mic, X } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Button from '../components/ui/Button.vue';
import Spinner from '../components/ui/Spinner.vue';
import KaraokeMode from '../components/practice/KaraokeMode.vue';
import FillBlankMode from '../components/practice/FillBlankMode.vue';
import McqMode from '../components/practice/McqMode.vue';
import OralMode from '../components/practice/OralMode.vue';
import { useSongsStore } from '../stores/songs';
import { useAuthStore } from '../stores/auth';
import { createPracticeSession } from '../lib/tauri-api';
import type { Song, PracticeMode } from '../types';

const { t } = useI18n();
const route = useRoute();
const songsStore = useSongsStore();
const authStore = useAuthStore();

const song = ref<Song | null>(null);
const loading = ref(true);
const activeMode = ref<PracticeMode | null>(null);

const modeTitles = computed<Record<PracticeMode, string>>(() => ({
  karaoke: t('songDetail.karaokeMode'),
  'fill-blank': t('songDetail.fillBlank'),
  mcq: t('songDetail.mcq'),
  oral: t('songDetail.oralPractice'),
}));

function startMode(mode: PracticeMode) {
  activeMode.value = mode;
}

function closeMode() {
  activeMode.value = null;
}

async function onPracticeFinish(data: {
  score: number;
  linesPracticed: number;
  linesCorrect: number;
  durationSeconds: number;
}) {
  if (authStore.user && song.value && activeMode.value) {
    try {
      await createPracticeSession(
        authStore.user.id,
        song.value.id,
        activeMode.value,
        data.score,
        data.linesPracticed,
        data.linesCorrect,
        data.durationSeconds,
      );
    } catch (err) {
      console.error('Failed to save practice session:', err);
    }
  }
  activeMode.value = null;
}

onMounted(async () => {
  try {
    const songId = String(route.params.id);
    song.value = await songsStore.fetchSong(songId);

    // Auto-start mode from query param (e.g. from PracticeView)
    const mode = route.query.mode as PracticeMode | undefined;
    if (mode && mode in modeTitles.value) {
      activeMode.value = mode;
    }
  } catch (err) {
    console.error('Failed to fetch song:', err);
  } finally {
    loading.value = false;
  }
});
</script>
