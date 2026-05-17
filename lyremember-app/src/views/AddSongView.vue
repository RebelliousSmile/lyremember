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
        <form class="space-y-6" @submit.prevent="handleSubmit">
          <Alert v-model="showError" type="error" closable>
            {{ error }}
          </Alert>

          <div>
            <label class="block text-sm font-medium text-[#B8B0D0] mb-1">
              {{ $t('addSong.importFile') }}
            </label>
            <input
              type="file"
              accept=".txt,.json,.lrc"
              class="block w-full text-sm text-[#B8B0D0] file:mr-3 file:px-3 file:py-1.5 file:rounded-lg file:border-0 file:bg-gold file:text-black file:font-medium file:cursor-pointer hover:file:bg-gold/90"
              @change="handleFileImport"
            />
            <p class="mt-1 text-xs text-[#8A82A0]">{{ $t('addSong.importFileHint') }}</p>
          </div>

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

          <Input
            v-model="form.geniusUrl"
            :label="$t('addSong.geniusUrl')"
            :placeholder="$t('addSong.geniusUrlPlaceholder')"
            type="url"
          />

          <div class="flex items-center gap-2">
            <input
              id="autoTranslate"
              v-model="form.autoTranslate"
              type="checkbox"
              class="rounded"
            />
            <label for="autoTranslate" class="text-sm text-[#B8B0D0]">
              {{ $t('addSong.autoTranslate') }}
            </label>
          </div>

          <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <p class="text-sm text-blue-800 dark:text-blue-300">
              <strong>Note:</strong> {{ $t('addSong.autoTranslateNote') }}
            </p>
            <ul
              class="mt-2 text-sm text-blue-700 dark:text-blue-400 list-disc list-inside space-y-1"
            >
              <li>{{ $t('addSong.autoTranslatePhonetic') }}</li>
              <li>{{ $t('addSong.autoTranslateTranslation') }}</li>
              <li>{{ $t('addSong.autoTranslateOffline') }}</li>
            </ul>
          </div>

          <div class="flex gap-4">
            <Button type="button" variant="ghost" class-name="flex-1" @click="$router.back()">
              {{ $t('addSong.cancel') }}
            </Button>
            <Button type="submit" variant="primary" :loading="loading" class-name="flex-1">
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
import { parseByExtension, FileImportError } from '../lib/file-parsers';

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
  geniusUrl: '',
});

const loading = ref(false);
const error = ref('');
const showError = ref(false);

async function handleFileImport(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;
  try {
    const content = await file.text();
    const parsed = parseByExtension(file.name, content);
    if (parsed.title) form.value.title = parsed.title;
    if (parsed.artist) form.value.artist = parsed.artist;
    if (parsed.language) form.value.language = parsed.language;
    form.value.lyrics = parsed.lyrics.join('\n');
  } catch (e) {
    error.value =
      e instanceof FileImportError ? e.message : `Failed to import file: ${(e as Error).message}`;
    showError.value = true;
  } finally {
    input.value = '';
  }
}

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
      .map((line) => line.trim())
      .filter((line) => line.length > 0);

    if (lyricsArray.length === 0) {
      throw new Error(t('addSong.enterOneLine'));
    }

    const song = await songsStore.createSong(
      form.value.title,
      form.value.artist,
      form.value.language,
      lyricsArray,
      form.value.geniusUrl.trim() || null,
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
