import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useUiStore = defineStore('ui', () => {
  // State
  const sidebarOpen = ref(true);
  const darkMode = ref(false);
  const mobileMenuOpen = ref(false);

  // Actions
  function toggleSidebar() {
    sidebarOpen.value = !sidebarOpen.value;
  }

  function closeSidebar() {
    sidebarOpen.value = false;
  }

  function openSidebar() {
    sidebarOpen.value = true;
  }

  function toggleDarkMode() {
    darkMode.value = !darkMode.value;
    document.documentElement.classList.toggle('dark', darkMode.value);
    localStorage.setItem('darkMode', darkMode.value.toString());
  }

  function toggleMobileMenu() {
    mobileMenuOpen.value = !mobileMenuOpen.value;
  }

  function closeMobileMenu() {
    mobileMenuOpen.value = false;
  }

  function initializeDarkMode() {
    const saved = localStorage.getItem('darkMode');
    if (saved !== null) {
      darkMode.value = saved === 'true';
    } else {
      // Check system preference
      darkMode.value = window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    document.documentElement.classList.toggle('dark', darkMode.value);
  }

  return {
    // State
    sidebarOpen,
    darkMode,
    mobileMenuOpen,
    // Actions
    toggleSidebar,
    closeSidebar,
    openSidebar,
    toggleDarkMode,
    toggleMobileMenu,
    closeMobileMenu,
    initializeDarkMode,
  };
});
