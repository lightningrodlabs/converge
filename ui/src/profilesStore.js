import { writable } from 'svelte/store';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const notifications = writable([]);