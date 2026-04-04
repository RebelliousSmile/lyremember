import { createRouter, createWebHashHistory } from 'vue-router';
import type { RouteRecordRaw } from 'vue-router';
import { useAuthStore } from '../stores/auth';

// Lazy load components
const LoginView = () => import('../views/LoginView.vue');
const RegisterView = () => import('../views/RegisterView.vue');
const DashboardView = () => import('../views/DashboardView.vue');
const SongsView = () => import('../views/SongsView.vue');
const SongDetailView = () => import('../views/SongDetailView.vue');
const AddSongView = () => import('../views/AddSongView.vue');
const PracticeView = () => import('../views/PracticeView.vue');
const ProfileView = () => import('../views/ProfileView.vue');
const SettingsView = () => import('../views/SettingsView.vue');

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/dashboard',
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: { requiresAuth: false },
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView,
    meta: { requiresAuth: false },
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: DashboardView,
    meta: { requiresAuth: true },
  },
  {
    path: '/songs',
    name: 'songs',
    component: SongsView,
    meta: { requiresAuth: true },
  },
  {
    path: '/songs/add',
    name: 'add-song',
    component: AddSongView,
    meta: { requiresAuth: true },
  },
  {
    path: '/songs/:id',
    name: 'song-detail',
    component: SongDetailView,
    meta: { requiresAuth: true },
  },
  {
    path: '/practice',
    name: 'practice',
    component: PracticeView,
    meta: { requiresAuth: true },
  },
  {
    path: '/profile',
    name: 'profile',
    component: ProfileView,
    meta: { requiresAuth: true },
  },
  {
    path: '/settings',
    name: 'settings',
    component: SettingsView,
    meta: { requiresAuth: true },
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

// Navigation guard
router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore();
  const requiresAuth = to.meta.requiresAuth !== false;

  if (requiresAuth && !authStore.isAuthenticated) {
    // Check if we have a saved token
    const hasAuth = await authStore.checkAuth();
    
    if (!hasAuth) {
      // Redirect to login
      next({ name: 'login', query: { redirect: to.fullPath } });
      return;
    }
  }

  // If already authenticated and trying to access login/register, redirect to dashboard
  if (authStore.isAuthenticated && (to.name === 'login' || to.name === 'register')) {
    next({ name: 'dashboard' });
    return;
  }

  next();
});

export default router;
