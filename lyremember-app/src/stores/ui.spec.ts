import { describe, it, expect, beforeEach, vi } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useUiStore } from './ui';

describe('ui store — dark mode', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    document.documentElement.classList.remove('dark');
  });

  it('toggleDarkMode flips state, updates <html class>, persists to localStorage', () => {
    const store = useUiStore();
    expect(store.darkMode).toBe(false);

    store.toggleDarkMode();
    expect(store.darkMode).toBe(true);
    expect(document.documentElement.classList.contains('dark')).toBe(true);
    expect(localStorage.getItem('darkMode')).toBe('true');

    store.toggleDarkMode();
    expect(store.darkMode).toBe(false);
    expect(document.documentElement.classList.contains('dark')).toBe(false);
    expect(localStorage.getItem('darkMode')).toBe('false');
  });

  it('initializeDarkMode prefers an explicit saved value over system preference', () => {
    localStorage.setItem('darkMode', 'true');
    // Pretend system preference is light to make sure the saved value wins.
    vi.spyOn(window, 'matchMedia').mockReturnValue({
      matches: false,
      media: '(prefers-color-scheme: dark)',
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => false,
    } as unknown as MediaQueryList);

    const store = useUiStore();
    store.initializeDarkMode();
    expect(store.darkMode).toBe(true);
    expect(document.documentElement.classList.contains('dark')).toBe(true);
  });

  it('initializeDarkMode falls back to prefers-color-scheme when no saved value', () => {
    vi.spyOn(window, 'matchMedia').mockReturnValue({
      matches: true,
      media: '(prefers-color-scheme: dark)',
      onchange: null,
      addListener: () => {},
      removeListener: () => {},
      addEventListener: () => {},
      removeEventListener: () => {},
      dispatchEvent: () => false,
    } as unknown as MediaQueryList);

    const store = useUiStore();
    store.initializeDarkMode();
    expect(store.darkMode).toBe(true);
    expect(document.documentElement.classList.contains('dark')).toBe(true);
  });
});
