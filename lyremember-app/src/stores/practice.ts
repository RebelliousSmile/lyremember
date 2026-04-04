import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import * as api from '../lib/tauri-api';
import { useAuthStore } from './auth';
import type { Song, PracticeSession } from '../types';

export type PracticeMode = 'karaoke' | 'fill-blank' | 'mcq';

export const MODE_LABELS: Record<PracticeMode, string> = {
  'karaoke': 'Karaoke',
  'fill-blank': 'Fill-in-the-Blank',
  'mcq': 'Multiple Choice',
};

export interface PracticeState {
  currentLine: number;
  totalLines: number;
  correctLines: number;
  startedAt: number;
  answers: boolean[];
}

export const usePracticeStore = defineStore('practice', () => {
  const song = ref<Song | null>(null);
  const mode = ref<PracticeMode | null>(null);
  const state = ref<PracticeState | null>(null);
  const finished = ref(false);
  const loading = ref(false);

  const score = computed(() => {
    if (!state.value || state.value.totalLines === 0) return 0;
    return Math.round((state.value.correctLines / state.value.totalLines) * 100);
  });

  const elapsedSeconds = computed(() => {
    if (!state.value) return 0;
    return Math.floor((Date.now() - state.value.startedAt) / 1000);
  });

  function startSession(s: Song, m: PracticeMode) {
    song.value = s;
    mode.value = m;
    finished.value = false;
    state.value = {
      currentLine: 0,
      totalLines: s.lyrics.length,
      correctLines: 0,
      startedAt: Date.now(),
      answers: [],
    };
  }

  function answerLine(correct: boolean) {
    if (!state.value || finished.value) return;
    state.value.answers.push(correct);
    if (correct) state.value.correctLines++;
    state.value.currentLine++;

    if (state.value.currentLine >= state.value.totalLines) {
      finished.value = true;
    }
  }

  async function saveSession(): Promise<PracticeSession | null> {
    const authStore = useAuthStore();
    if (!authStore.user || !song.value || !mode.value || !state.value) return null;

    loading.value = true;
    try {
      const duration = Math.floor((Date.now() - state.value.startedAt) / 1000);
      return await api.createPracticeSession(
        authStore.user.id,
        song.value.id,
        mode.value,
        score.value,
        state.value.totalLines,
        state.value.correctLines,
        duration,
      );
    } finally {
      loading.value = false;
    }
  }

  function reset() {
    song.value = null;
    mode.value = null;
    state.value = null;
    finished.value = false;
  }

  return {
    song, mode, state, finished, loading,
    score, elapsedSeconds,
    startSession, answerLine, saveSession, reset,
  };
});
