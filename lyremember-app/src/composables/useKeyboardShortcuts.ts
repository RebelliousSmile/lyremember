import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';

export function useKeyboardShortcuts() {
  const router = useRouter();
  const showHelp = ref(false);

  function handler(e: KeyboardEvent) {
    // Don't trigger when typing in inputs
    const tag = (e.target as HTMLElement).tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    // Escape — close help
    if (e.key === 'Escape') {
      showHelp.value = false;
      return;
    }

    // ? — toggle shortcuts help
    if (e.key === '?') {
      showHelp.value = !showHelp.value;
      return;
    }

    // g then d — go to dashboard
    // g then s — go to songs
    // g then p — go to practice
    // g then t — go to settings
    if (e.key === 'g') {
      waitForSecondKey();
      return;
    }
  }

  let pendingG = false;
  let gTimeout: ReturnType<typeof setTimeout> | null = null;

  function waitForSecondKey() {
    pendingG = true;
    gTimeout = setTimeout(() => { pendingG = false; }, 1000);
  }

  function gHandler(e: KeyboardEvent) {
    if (!pendingG) return;
    const tag = (e.target as HTMLElement).tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

    pendingG = false;
    if (gTimeout) clearTimeout(gTimeout);

    switch (e.key) {
      case 'd': router.push('/dashboard'); break;
      case 's': router.push('/songs'); break;
      case 'p': router.push('/practice'); break;
      case 't': router.push('/settings'); break;
    }
  }

  onMounted(() => {
    window.addEventListener('keydown', handler);
    window.addEventListener('keydown', gHandler);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handler);
    window.removeEventListener('keydown', gHandler);
    if (gTimeout) clearTimeout(gTimeout);
  });

  return { showHelp };
}

export const shortcuts = [
  { keys: '?', description: 'shortcuts.toggleHelp' },
  { keys: 'g d', description: 'shortcuts.goToDashboard' },
  { keys: 'g s', description: 'shortcuts.goToSongs' },
  { keys: 'g p', description: 'shortcuts.goToPractice' },
  { keys: 'g t', description: 'shortcuts.goToSettings' },
  { keys: 'Esc', description: 'shortcuts.close' },
];
