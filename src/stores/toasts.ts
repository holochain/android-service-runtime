import { writable, get, derived } from 'svelte/store';
import {v7 as uuidv7} from 'uuid';

export interface Toast {
  text: string;
  status: string;
}

export const toasts = writable<{[id:string]: Toast}>({})

export function addToast(text: string, status: string) {
  const id = uuidv7();
  toasts.update((t) => ({...t, [id]: {text, status}}));

  setTimeout(() => {
    toasts.update(($toasts) => { delete $toasts[id]; return $toasts; });
  }, 5000);
}

export const toastsList = derived(toasts, ($toasts) => Object.values($toasts));