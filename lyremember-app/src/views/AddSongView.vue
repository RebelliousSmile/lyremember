<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <div class="flex items-center gap-4">
        <Button variant="ghost" @click="$router.back()">
          <ArrowLeft :size="20" />
        </Button>
        <h1 class="text-3xl font-bold text-[#F5F0EB]">
          {{ $t('addSong.title') }}
        </h1>
      </div>

      <Card>
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <Alert v-model="showError" type="error" closable>
            {{ error }}
          </Alert>

          <Input
            v-model="form.title"
            :label="$t('addSong.songTitle')"
            :placeholder="$t('addSong.songTitlePlaceholder')"
            required
          />

          <Input
            v-model="form.artist"
            :label="$t('addSong.artist')"
            :placeholder="$t('addSong.artistPlaceholder')"
            required
          />

          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('addSong.language') }} <span class="text-red-500">*</span>
            </label>
            <select
              v-model="form.language"
              required
              class="w-full px-3 py-2 border border-deep-border rounded-lg bg-deep-card"
            >
              <option value="">{{ $t('addSong.selectLanguage') }}</option>
              <option value="fr">{{ $t('songs.french') }}</option>
              <option value="en">{{ $t('songs.english') }}</option>
              <option value="jp">{{ $t('songs.japanese') }}</option>
              <option value="kr">{{ $t('songs.korean') }}</option>
            </select>
          </div>

          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('addSong.lyrics') }} <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.lyrics"
              rows="10"
              required
              :placeholder="$t('addSong.lyricsPlaceholder')"
              class="w-full px-3 py-2 border border-deep-border rounded-lg bg-deep-card font-mono text-sm"
            />
            <p class="mt-1 text-sm text-[#8A82A0]">
              {{ $t('addSong.lyricsHint') }}
            </p>
          </div>

          <div class="flex items-center gap-2">
            <input v-model="form.autoTranslate" type="checkbox" id="autoTranslate" class="rounded" />
            <label for="autoTranslate" class="text-sm text-[#B8B0D0]">
              {{ $t('addSong.autoTranslate') }}
            </label>
          </div>

          <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <p class="text-sm text-blue-800 dark:text-blue-300">
              <strong>Note:</strong> {{ $t('addSong.autoTranslateNote') }}
            </p>
            <ul class="mt-2 text-sm text-blue-700 dark:text-blue-400 list-disc list-inside space-y-1">
              <li>{{ $t('addSong.autoTranslatePhonetic') }}</li>
              <li>{{ $t('addSong.autoTranslateTranslation') }}</li>
              <li>{{ $t('addSong.autoTranslateOffline') }}</li>
            </ul>
          </div>

          <div class="flex gap-4">
            <Button type="button" variant="ghost" @click="$router.back()" className="flex-1">
              {{ $t('addSong.cancel') }}
            </Button>
            <Button type="submit" variant="primary" :loading="loading" className="flex-1">
              <Plus :size="20" />
              {{ $t('addSong.createSong') }}
            </Button>
          </div>
        </form>
      </Card>
    </div>
  </MainLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { ArrowLeft, Plus } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Input from '../components/ui/Input.vue';
import Button from '../components/ui/Button.vue';
import Alert from '../components/ui/Alert.vue';
import { useSongsStore } from '../stores/songs';
import { useAuthStore } from '../stores/auth';
import type { CreateSongForm } from '../types';

const { t } = useI18n();
const router = useRouter();
const songsStore = useSongsStore();
const authStore = useAuthStore();

const form = ref<CreateSongForm>({
  title: '',
  artist: '',
  language: '',
  lyrics: '',
  autoTranslate: true,
});

const loading = ref(false);
const error = ref('');
const showError = ref(false);

async function handleSubmit() {
  if (!form.value.lyrics.trim()) {
    error.value = t('addSong.enterLyrics');
    showError.value = true;
    return;
  }

  loading.value = true;
  error.value = '';
  showError.value = false;

  try {
    const lyricsArray = form.value.lyrics
      .split('\n')
      .map(line => line.trim())
      .filter(line => line.length > 0);

    if (lyricsArray.length === 0) {
      throw new Error(t('addSong.enterOneLine'));
    }

    const song = await songsStore.createSong(
      form.value.title,
      form.value.artist,
      form.value.language,
      lyricsArray,
    );

    if (authStore.user) {
      await songsStore.addToRepertoire(song.id);
    }

    router.push(`/songs/${song.id}`);
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to create song';
    showError.value = true;
  } finally {
    loading.value = false;
  }
}
</script>
