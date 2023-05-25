import { writable } from 'svelte/store';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const notifications = writable([]);

export function navigate(location, hash) {
    view.update(v => location);
    viewHash.update(h => hash)
}

export function notifications_update(new_notifications) {
    let ordered = new_notifications.sort((a, b) => parseFloat(b.timestamp) - parseFloat(a.timestamp));
    notifications.update(v => ordered)
}