import { ref, onMounted } from 'vue';
import { useAuthStore } from '../stores/auth';
import { getUserStats, getUserStreak, getRecommendations, type UserStats } from '../lib/tauri-api';

export function useUserStats() {
  const authStore = useAuthStore();
  const userStats = ref<UserStats | null>(null);
  const streak = ref<number>(0);
  const recommendedSongIds = ref<string[]>([]);
  const loading = ref(false);

  async function fetchStats() {
    if (!authStore.user) return;
    loading.value = true;
    try {
      const [stats, s, recs] = await Promise.all([
        getUserStats(authStore.user.id),
        getUserStreak(authStore.user.id).catch(() => 0),
        getRecommendations(authStore.user.id, 5).catch(() => [] as string[]),
      ]);
      userStats.value = stats;
      streak.value = s;
      recommendedSongIds.value = recs;
    } catch (err) {
      console.error('Failed to fetch user stats:', err);
    } finally {
      loading.value = false;
    }
  }

  onMounted(fetchStats);

  return { userStats, streak, recommendedSongIds, loading, fetchStats };
}
