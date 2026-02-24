<template>
  <MainLayout>
    <div class="max-w-2xl mx-auto space-y-6">
      <div class="flex items-center gap-4">
        <Button
          variant="ghost"
          @click="$router.back()"
        >
          <ArrowLeft :size="20" />
        </Button>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
          Add New Song
        </h1>
      </div>
      
      <Card>
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <Alert
            v-model="showError"
            type="error"
            closable
          >
            {{ error }}
          </Alert>
          
          <Input
            v-model="form.title"
            label="Song Title"
            placeholder="Enter song title"
            required
          />
          
          <Input
            v-model="form.artist"
            label="Artist"
            placeholder="Enter artist name"
            required
          />
          
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Language <span class="text-red-500">*</span>
            </label>
            <select
              v-model="form.language"
              required
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800"
            >
              <option value="">Select language</option>
              <option value="fr">French</option>
              <option value="en">English</option>
              <option value="jp">Japanese</option>
              <option value="kr">Korean</option>
            </select>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Lyrics <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.lyrics"
              rows="10"
              required
              placeholder="Enter lyrics (one line per line)"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 font-mono text-sm"
            />
            <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
              Enter each line of lyrics on a new line
            </p>
          </div>
          
          <div class="flex items-center gap-2">
            <input
              v-model="form.autoTranslate"
              type="checkbox"
              id="autoTranslate"
              class="rounded"
            />
            <label for="autoTranslate" class="text-sm text-gray-700 dark:text-gray-300">
              Automatically translate to English (if not already in English)
            </label>
          </div>
          
          <div class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
            <p class="text-sm text-blue-800 dark:text-blue-300">
              <strong>Note:</strong> When you create the song, it will automatically:
            </p>
            <ul class="mt-2 text-sm text-blue-700 dark:text-blue-400 list-disc list-inside space-y-1">
              <li>Generate phonetic romanization for Japanese and Korean</li>
              <li>Translate to English (if auto-translate is enabled)</li>
              <li>Store everything for offline use</li>
            </ul>
          </div>
          
          <div class="flex gap-4">
            <Button
              type="button"
              variant="ghost"
              @click="$router.back()"
              className="flex-1"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              variant="primary"
              :loading="loading"
              className="flex-1"
            >
              <Plus :size="20" />
              Create Song
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
import { ArrowLeft, Plus } from 'lucide-vue-next';
import MainLayout from '../components/layout/MainLayout.vue';
import Card from '../components/ui/Card.vue';
import Input from '../components/ui/Input.vue';
import Button from '../components/ui/Button.vue';
import Alert from '../components/ui/Alert.vue';
import { useSongsStore } from '../stores/songs';
import { useAuthStore } from '../stores/auth';
import type { CreateSongForm } from '../types';

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
    error.value = 'Please enter song lyrics';
    showError.value = true;
    return;
  }

  loading.value = true;
  error.value = '';
  showError.value = false;

  try {
    // Split lyrics into array
    const lyricsArray = form.value.lyrics
      .split('\n')
      .map(line => line.trim())
      .filter(line => line.length > 0);

    if (lyricsArray.length === 0) {
      throw new Error('Please enter at least one line of lyrics');
    }

    // Create the song
    const song = await songsStore.createSong(
      form.value.title,
      form.value.artist,
      form.value.language,
      lyricsArray,
      form.value.autoTranslate
    );

    // Add to user's repertoire
    if (authStore.user) {
      await songsStore.addToRepertoire(song.id);
    }

    // Navigate to the song detail
    router.push(`/songs/${song.id}`);
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to create song';
    showError.value = true;
  } finally {
    loading.value = false;
  }
}
</script>
