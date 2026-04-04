import { ref } from 'vue';

export interface Toast {
  id: number;
  message: string;
  type: 'success' | 'error' | 'info';
}

const toasts = ref<Toast[]>([]);
let nextId = 0;

function add(message: string, type: Toast['type'] = 'info', duration = 4000) {
  const id = nextId++;
  toasts.value.push({ id, message, type });
  if (duration > 0) {
    setTimeout(() => remove(id), duration);
  }
}

function remove(id: number) {
  toasts.value = toasts.value.filter(t => t.id !== id);
}

export function useToast() {
  return {
    toasts,
    add,
    remove,
    success: (msg: string) => add(msg, 'success'),
    error: (msg: string) => add(msg, 'error'),
    info: (msg: string) => add(msg, 'info'),
  };
}
