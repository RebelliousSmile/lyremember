<template>
  <div class="space-y-6">
    <!-- Progress bar -->
    <div class="flex items-center gap-4">
      <div class="flex-1 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
        <div
          class="bg-indigo-600 dark:bg-indigo-400 h-2 rounded-full transition-all duration-500"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <span class="text-sm text-gray-600 dark:text-gray-400 whitespace-nowrap">
        {{ currentIndex + 1 }} / {{ song.lyrics.length }}
      </span>
    </div>

    <!-- Lyrics display -->
    <div class="space-y-1 max-h-[60vh] overflow-y-auto scroll-smooth" ref="lyricsContainer">
      <div
        v-for="(line, index) in song.lyrics"
        :key="index"
        :ref="el => { if (index === currentIndex) activeLine = el as HTMLElement }"
        class="flex gap-4 p-3 rounded-lg transition-all duration-300"
        :class="{
          'bg-indigo-100 dark:bg-indigo-900/40 scale-[1.02]': index === currentIndex,
          'opacity-40': index < currentIndex,
          'opacity-60': index > currentIndex,
        }"
      >
        <div class="flex-1">
          <p class="text-lg" :class="{ 'font-bold text-indigo-700 dark:text-indigo-300': index === currentIndex }">
            {{ line }}
          </p>
          <p
            v-if="song.phonetic_lyrics?.[index]"
            class="text-sm italic mt-1"
            :class="index === currentIndex ? 'text-indigo-500 dark:text-indigo-400' : 'text-gray-400 dark:text-gray-500'"
          >
            {{ song.phonetic_lyrics[index] }}
          </p>
          <p
            v-if="showTranslation && song.translations?.en?.[index]"
            class="text-sm mt-1"
            :class="index === currentIndex ? 'text-gray-600 dark:text-gray-300' : 'text-gray-400 dark:text-gray-500'"
          >
            {{ song.translations.en[index] }}
          </p>
        </div>
      </div>
    </div>

    <!-- Controls -->
    <div class="flex items-center justify-between">
      <div class="flex gap-2">
        <Button variant="ghost" size="sm" :disabled="currentIndex === 0" @click="prev">
          <SkipBack :size="18" />
        </Button>
        <Button variant="primary" @click="togglePlay">
          <component :is="playing ? Pause : Play" :size="20" />
          {{ playing ? 'Pause' : 'Play' }}
        </Button>
        <Button variant="ghost" size="sm" :disabled="currentIndex >= song.lyrics.length - 1" @click="next">
          <SkipForward :size="18" />
        </Button>
      </div>

      <div class="flex items-center gap-4">
        <label class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 cursor-pointer">
          <input type="checkbox" v-model="showTranslation" class="rounded" />
          Translation
        </label>
        <div class="flex items-center gap-2">
          <span class="text-xs text-gray-500">Speed</span>
          <select v-model.number="speed" class="text-sm rounded border-gray-300 dark:border-gray-600 dark:bg-gray-700 px-2 py-1">
            <option :value="4000">Slow</option>
            <option :value="2500">Normal</option>
            <option :value="1500">Fast</option>
          </select>
        </div>
      </div>
    </div>

    <!-- End screen -->
    <div v-if="finished" class="text-center py-6 space-y-4">
      <CheckCircle :size="48" class="mx-auto text-green-500" />
      <p class="text-xl font-semibold text-gray-900 dark:text-white">Song Complete!</p>
      <div class="flex justify-center gap-3">
        <Button variant="primary" @click="restart">
          <RotateCcw :size="18" />
          Restart
        </Button>
        <Button variant="secondary" @click="$emit('finish', sessionData)">
          Done
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onUnmounted } from 'vue';
import { Play, Pause, SkipBack, SkipForward, CheckCircle, RotateCcw } from 'lucide-vue-next';
import Button from '../ui/Button.vue';
import type { Song } from '../../types';

const props = defineProps<{ song: Song }>();
defineEmits<{
  finish: [data: { score: number; linesPracticed: number; linesCorrect: number; durationSeconds: number }];
}>();

const currentIndex = ref(0);
const playing = ref(false);
const showTranslation = ref(false);
const speed = ref(2500);
const finished = ref(false);
const activeLine = ref<HTMLElement | null>(null);
const lyricsContainer = ref<HTMLElement | null>(null);
const startTime = Date.now();

let timer: ReturnType<typeof setInterval> | null = null;

const progress = computed(() =>
  Math.round(((currentIndex.value + (finished.value ? 1 : 0)) / props.song.lyrics.length) * 100)
);

const sessionData = computed(() => ({
  score: 100,
  linesPracticed: props.song.lyrics.length,
  linesCorrect: props.song.lyrics.length,
  durationSeconds: Math.round((Date.now() - startTime) / 1000),
}));

function scrollToActive() {
  nextTick(() => {
    activeLine.value?.scrollIntoView({ behavior: 'smooth', block: 'center' });
  });
}

function next() {
  if (currentIndex.value < props.song.lyrics.length - 1) {
    currentIndex.value++;
    scrollToActive();
  } else {
    stopPlay();
    finished.value = true;
  }
}

function prev() {
  if (currentIndex.value > 0) {
    currentIndex.value--;
    scrollToActive();
  }
}

function startPlay() {
  playing.value = true;
  timer = setInterval(next, speed.value);
}

function stopPlay() {
  playing.value = false;
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
}

function togglePlay() {
  if (finished.value) {
    restart();
    return;
  }
  playing.value ? stopPlay() : startPlay();
}

function restart() {
  stopPlay();
  currentIndex.value = 0;
  finished.value = false;
  scrollToActive();
}

watch(speed, () => {
  if (playing.value) {
    stopPlay();
    startPlay();
  }
});

onUnmounted(() => stopPlay());
</script>
