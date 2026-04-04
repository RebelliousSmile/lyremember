import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import * as api from '../lib/tauri-api';
import type { Song } from '../types';
import { useAuthStore } from './auth';

export const useSongsStore = defineStore('songs', () => {
  // State
  const songs = ref<Song[]>([]);
  const currentSong = ref<Song | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const searchQuery = ref('');
  const selectedLanguage = ref<string>('all');

  // Getters
  const filteredSongs = computed(() => {
    let filtered = songs.value;

    // Filter by search query
    if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      filtered = filtered.filter(
        (song) =>
          song.title.toLowerCase().includes(query) ||
          song.artist.toLowerCase().includes(query)
      );
    }

    // Filter by language
    if (selectedLanguage.value !== 'all') {
      filtered = filtered.filter((song) => song.language === selectedLanguage.value);
    }

    return filtered;
  });

  const songsByLanguage = computed(() => {
    const grouped: Record<string, Song[]> = {};
    songs.value.forEach((song) => {
      if (!grouped[song.language]) {
        grouped[song.language] = [];
      }
      grouped[song.language].push(song);
    });
    return grouped;
  });

  const totalSongs = computed(() => songs.value.length);

  // Actions
  async function fetchAllSongs() {
    loading.value = true;
    error.value = null;
    try {
      const allSongs = await api.getSongs();
      songs.value = allSongs;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch songs';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function fetchUserSongs() {
    const authStore = useAuthStore();
    if (!authStore.user) {
      throw new Error('User not authenticated');
    }

    loading.value = true;
    error.value = null;
    try {
      const userSongs = await api.getUserSongs(authStore.user.id);
      songs.value = userSongs;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch user songs';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function fetchSong(songId: string) {
    loading.value = true;
    error.value = null;
    try {
      const song = await api.getSong(songId);
      currentSong.value = song;
      return song;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch song';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function createSong(
    title: string,
    artist: string,
    language: string,
    lyrics: string[]
  ) {
    loading.value = true;
    error.value = null;
    try {
      const result = await api.createSong(title, artist, language, lyrics);
      songs.value.push(result.song);
      return result;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to create song';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function addToRepertoire(songId: string) {
    const authStore = useAuthStore();
    if (!authStore.user) {
      throw new Error('User not authenticated');
    }

    loading.value = true;
    try {
      await api.addToRepertoire(authStore.user.id, songId);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to add to repertoire';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function updateSong(
    songId: string,
    title?: string,
    artist?: string,
    lyrics?: string[]
  ) {
    loading.value = true;
    error.value = null;
    try {
      await api.updateSong(songId, title, artist, lyrics);
      // Refresh the song
      await fetchSong(songId);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to update song';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function deleteSong(songId: string) {
    loading.value = true;
    error.value = null;
    try {
      await api.deleteSong(songId);
      songs.value = songs.value.filter((s) => s.id !== songId);
      if (currentSong.value?.id === songId) {
        currentSong.value = null;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to delete song';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query;
  }

  function setSelectedLanguage(language: string) {
    selectedLanguage.value = language;
  }

  function clearError() {
    error.value = null;
  }

  return {
    // State
    songs,
    currentSong,
    loading,
    error,
    searchQuery,
    selectedLanguage,
    // Getters
    filteredSongs,
    songsByLanguage,
    totalSongs,
    // Actions
    fetchAllSongs,
    fetchUserSongs,
    fetchSong,
    createSong,
    addToRepertoire,
    updateSong,
    deleteSong,
    setSearchQuery,
    setSelectedLanguage,
    clearError,
  };
});
