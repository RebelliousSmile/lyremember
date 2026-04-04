import { ref, onMounted } from 'vue';
import * as api from '../lib/tauri-api';
import { useAuthStore } from '../stores/auth';
import { useToast } from './useToast';
import type { UserStats } from '../types';

export function useUserStats() {
  const stats = ref<UserStats | null>(null);
  const loading = ref(false);
  const authStore = useAuthStore();
  const toast = useToast();

  async function fetchStats() {
    if (!authStore.user) return;
    loading.value = true;
    try {
      stats.value = await api.getUserStats(authStore.user.id);
    } catch (err) {
      toast.error('Failed to load statistics');
    } finally {
      loading.value = false;
    }
  }

  onMounted(fetchStats);

  return { stats, loading, fetchStats };
}
