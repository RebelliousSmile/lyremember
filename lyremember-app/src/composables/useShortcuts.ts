import { onMounted, onUnmounted } from 'vue';

/**
 * Bind keyboard shortcuts active while the composable's owner is mounted.
 * Keys = `KeyboardEvent.key` ; ' ' = space, 'Tab', 'Escape', '1' to '9', 'a' to 'z', etc.
 * Ignored when focus is in an INPUT / TEXTAREA / SELECT.
 */
export function useShortcuts(bindings: Record<string, () => void>) {
  function handler(e: KeyboardEvent) {
    const tag = (e.target as HTMLElement).tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;
    const fn = bindings[e.key];
    if (fn) {
      e.preventDefault();
      fn();
    }
  }

  onMounted(() => window.addEventListener('keydown', handler));
  onUnmounted(() => window.removeEventListener('keydown', handler));
}
