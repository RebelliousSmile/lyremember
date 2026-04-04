import { ref, onMounted } from 'vue';
import { useAuthStore } from '../stores/auth';
import { getUserStats, type UserStats } from '../lib/tauri-api';

export function useUserStats() {
  const authStore = useAuthStore();
  const userStats = ref<UserStats | null>(null);
  const loading = ref(false);

  async function fetchStats() {
    if (!authStore.user) return;
    loading.value = true;
    try {
      userStats.value = await getUserStats(authStore.user.id);
    } catch (err) {
      console.error('Failed to fetch user stats:', err);
    } finally {
      loading.value = false;
    }
  }

  onMounted(fetchStats);

  return { userStats, loading, fetchStats };
}
