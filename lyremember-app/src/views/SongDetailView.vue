<template>
  <MainLayout>
    <div class="space-y-5">
      <!-- Loading state -->
      <div v-if="loading">
        <Spinner :label="$t('songDetail.loadingSong')" />
      </div>

      <!-- Not found -->
      <div v-else-if="!song" class="text-center py-12">
        <p class="text-red-400">{{ $t('songDetail.songNotFound') }}</p>
      </div>

      <template v-else>
        <!-- Song Header Bar -->
        <div class="flex items-center gap-3 py-2">
          <button
            class="w-10 h-10 flex items-center justify-center rounded-xl bg-deep-card border border-deep-border text-[#B8B0D0] hover:text-[#F5F0EB] transition-colors"
            @click="$router.back()"
          >
            <ArrowLeft :size="20" />
          </button>

          <!-- Cover with gradient bg and emoji -->
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-violet-accent to-[#5E5480] flex items-center justify-center text-lg shrink-0">
            {{ languageEmoji(song.language) }}
          </div>

          <div class="flex-1 min-w-0">
            <h1 class="text-base font-bold text-[#F5F0EB] truncate leading-tight">
              {{ song.title }}
            </h1>
            <p class="text-sm text-[#B8B0D0] truncate leading-tight">
              {{ song.artist }}
            </p>
          </div>

          <button class="w-10 h-10 flex items-center justify-center rounded-xl text-[#8A82A0] hover:text-[#F5F0EB] transition-colors">
            <MoreHorizontal :size="20" />
          </button>
        </div>

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

        <!-- Normal view -->
        <template v-else>
          <!-- Language Selector Bar -->
          <div
            v-if="song.translations && Object.keys(song.translations).length > 0"
            class="flex items-center gap-2 flex-wrap"
          >
            <span class="text-xs text-[#8A82A0] font-medium mr-1">Traduire en :</span>
            <button
              v-for="lang in availableTranslationLangs"
              :key="lang"
              class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all"
              :class="selectedTranslation === lang
                ? 'border border-gold text-gold bg-gold/[0.08]'
                : 'border border-deep-border text-[#B8B0D0] bg-deep-card hover:border-[#3A3460]'"
              @click="selectedTranslation = selectedTranslation === lang ? null : lang"
            >
              {{ langFlag(lang) }} {{ lang.toUpperCase() }}
            </button>
          </div>

          <!-- Lyrics Bilingual View -->
          <div class="bg-deep-card rounded-2xl border border-deep-border p-5 space-y-5">
            <div v-for="(section, sIdx) in lyricSections" :key="sIdx">
              <!-- Section label -->
              <p class="text-[10px] font-bold uppercase tracking-[0.15em] text-gold mb-3">
                {{ section.label }}
              </p>

              <!-- Lyric lines -->
              <div class="space-y-3">
                <div
                  v-for="(line, lIdx) in section.lines"
                  :key="lIdx"
                  class="border-l-2 border-violet-accent/40 pl-3"
                >
                  <p class="text-[15px] leading-relaxed text-[#F5F0EB] font-serif">
                    {{ line.original }}
                  </p>
                  <p
                    v-if="line.translation"
                    class="text-[13px] leading-relaxed text-gold italic opacity-85 mt-0.5"
                  >
                    {{ line.translation }}
                  </p>
                </div>
              </div>
            </div>
          </div>

          <!-- Practice Modes 2x2 Grid -->
          <div>
            <h2 class="text-sm font-bold text-[#F5F0EB] mb-3">{{ $t('songDetail.practiceModes') }}</h2>
            <div class="grid grid-cols-2 gap-3">
              <!-- Fill in the blanks -->
              <button
                class="bg-deep-card rounded-2xl border border-deep-border p-5 text-left hover:bg-deep-card-hover transition-colors"
                @click="startMode('fill-blank')"
              >
                <div class="w-10 h-10 rounded-full bg-[#F59E0B]/20 flex items-center justify-center mb-3">
                  <PenLine :size="18" class="text-[#F59E0B]" />
                </div>
                <p class="text-sm font-bold text-[#F5F0EB] mb-0.5">{{ $t('songDetail.fillBlank') }}</p>
                <p class="text-[11px] text-[#8A82A0] leading-snug">{{ $t('songDetail.fillBlankDesc', 'Complétez les paroles manquantes') }}</p>
              </button>

              <!-- First letter -->
              <button
                class="bg-deep-card rounded-2xl border border-deep-border p-5 text-left hover:bg-deep-card-hover transition-colors"
                @click="startMode('mcq')"
              >
                <div class="w-10 h-10 rounded-full bg-[#10B981]/20 flex items-center justify-center mb-3">
                  <List :size="18" class="text-[#10B981]" />
                </div>
                <p class="text-sm font-bold text-[#F5F0EB] mb-0.5">{{ $t('songDetail.mcq') }}</p>
                <p class="text-[11px] text-[#8A82A0] leading-snug">{{ $t('songDetail.mcqDesc', 'Choisissez la bonne réponse') }}</p>
              </button>

              <!-- Karaoke -->
              <button
                class="bg-deep-card rounded-2xl border border-deep-border p-5 text-left hover:bg-deep-card-hover transition-colors"
                @click="startMode('karaoke')"
              >
                <div class="w-10 h-10 rounded-full bg-[#EC4899]/20 flex items-center justify-center mb-3">
                  <PlayCircle :size="18" class="text-[#EC4899]" />
                </div>
                <p class="text-sm font-bold text-[#F5F0EB] mb-0.5">{{ $t('songDetail.karaokeMode') }}</p>
                <p class="text-[11px] text-[#8A82A0] leading-snug">{{ $t('songDetail.karaokeDesc', 'Chantez en suivant les paroles') }}</p>
              </button>

              <!-- Oral / Flashcards -->
              <button
                class="bg-deep-card rounded-2xl border border-deep-border p-5 text-left hover:bg-deep-card-hover transition-colors"
                @click="startMode('oral')"
              >
                <div class="w-10 h-10 rounded-full bg-[#6366F1]/20 flex items-center justify-center mb-3">
                  <Mic :size="18" class="text-[#6366F1]" />
                </div>
                <p class="text-sm font-bold text-[#F5F0EB] mb-0.5">{{ $t('songDetail.oralPractice') }}</p>
                <p class="text-[11px] text-[#8A82A0] leading-snug">{{ $t('songDetail.oralDesc', 'Pratiquez à l\'oral') }}</p>
              </button>
            </div>
          </div>
        </template>
      </template>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { ArrowLeft, PlayCircle, PenLine, List, Mic, X, MoreHorizontal } from 'lucide-vue-next';
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
const selectedTranslation = ref<string | null>(null);

const modeTitles = computed<Record<PracticeMode, string>>(() => ({
  karaoke: t('songDetail.karaokeMode'),
  'fill-blank': t('songDetail.fillBlank'),
  mcq: t('songDetail.mcq'),
  oral: t('songDetail.oralPractice'),
}));

const availableTranslationLangs = computed(() => {
  if (!song.value?.translations) return [];
  return Object.keys(song.value.translations);
});

const langFlagMap: Record<string, string> = {
  fr: '\u{1F1EB}\u{1F1F7}',
  en: '\u{1F1EC}\u{1F1E7}',
  es: '\u{1F1EA}\u{1F1F8}',
  de: '\u{1F1E9}\u{1F1EA}',
  it: '\u{1F1EE}\u{1F1F9}',
  pt: '\u{1F1F5}\u{1F1F9}',
  ja: '\u{1F1EF}\u{1F1F5}',
  ko: '\u{1F1F0}\u{1F1F7}',
  zh: '\u{1F1E8}\u{1F1F3}',
  ar: '\u{1F1F8}\u{1F1E6}',
  ru: '\u{1F1F7}\u{1F1FA}',
  hi: '\u{1F1EE}\u{1F1F3}',
};

function langFlag(lang: string): string {
  return langFlagMap[lang.toLowerCase()] || '\u{1F310}';
}

function languageEmoji(lang: string): string {
  return langFlagMap[lang.toLowerCase()] || '\u{1F3B5}';
}

interface LyricLine {
  original: string;
  translation: string | null;
}

interface LyricSection {
  label: string;
  lines: LyricLine[];
}

const lyricSections = computed<LyricSection[]>(() => {
  if (!song.value) return [];
  const lyrics = song.value.lyrics;
  const translations = selectedTranslation.value && song.value.translations
    ? song.value.translations[selectedTranslation.value] || null
    : null;

  const sections: LyricSection[] = [];
  let currentLines: LyricLine[] = [];
  let sectionCount = 0;

  for (let i = 0; i < lyrics.length; i++) {
    const line = lyrics[i];
    if (line.trim() === '') {
      // Empty line = section break
      if (currentLines.length > 0) {
        sectionCount++;
        sections.push({
          label: `Verse ${sectionCount}`,
          lines: currentLines,
        });
        currentLines = [];
      }
    } else {
      currentLines.push({
        original: line,
        translation: translations && translations[i] ? translations[i] : null,
      });
    }
  }

  // Push remaining lines
  if (currentLines.length > 0) {
    sectionCount++;
    sections.push({
      label: `Verse ${sectionCount}`,
      lines: currentLines,
    });
  }

  // If no sections were created (no empty lines), just put everything in one section
  if (sections.length === 0 && lyrics.length > 0) {
    sections.push({
      label: 'Verse 1',
      lines: lyrics.map((line: string, i: number) => ({
        original: line,
        translation: translations && translations[i] ? translations[i] : null,
      })),
    });
  }

  return sections;
});

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

    // Auto-select first translation language if available
    if (song.value?.translations) {
      const langs = Object.keys(song.value.translations);
      if (langs.length > 0) {
        selectedTranslation.value = langs[0];
      }
    }
  } catch (err) {
    console.error('Failed to fetch song:', err);
  } finally {
    loading.value = false;
  }
});
</script>
