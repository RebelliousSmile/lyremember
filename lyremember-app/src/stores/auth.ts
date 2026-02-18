import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import * as api from '../lib/tauri-api';
import type { User } from '../types';

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null);
  const token = ref<string | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Getters
  const isAuthenticated = computed(() => !!token.value && !!user.value);
  const username = computed(() => user.value?.username || '');

  // Actions
  async function register(username: string, email: string, password: string) {
    loading.value = true;
    error.value = null;
    try {
      const newUser = await api.register(username, email, password);
      user.value = newUser;
      return newUser;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Registration failed';
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function login(username: string, password: string) {
    loading.value = true;
    error.value = null;
    try {
      const authToken = await api.login(username, password);
      token.value = authToken;
      
      // Verify token and get user
      const authenticatedUser = await api.verifyToken(authToken);
      user.value = authenticatedUser;
      
      // Save token to localStorage
      localStorage.setItem('auth_token', authToken);
      
      return authenticatedUser;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Login failed';
      token.value = null;
      user.value = null;
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function logout() {
    user.value = null;
    token.value = null;
    localStorage.removeItem('auth_token');
  }

  async function checkAuth() {
    const savedToken = localStorage.getItem('auth_token');
    if (!savedToken) {
      return false;
    }

    loading.value = true;
    try {
      const authenticatedUser = await api.verifyToken(savedToken);
      user.value = authenticatedUser;
      token.value = savedToken;
      return true;
    } catch (err) {
      // Token invalid, clear it
      localStorage.removeItem('auth_token');
      token.value = null;
      user.value = null;
      return false;
    } finally {
      loading.value = false;
    }
  }

  function clearError() {
    error.value = null;
  }

  return {
    // State
    user,
    token,
    loading,
    error,
    // Getters
    isAuthenticated,
    username,
    // Actions
    register,
    login,
    logout,
    checkAuth,
    clearError,
  };
});
