import { writable } from 'svelte/store';
import { browser } from '$app/environment';

const storedId = browser ? localStorage.getItem('user_id') : null;

export const userId = writable<string | null>(storedId);

userId.subscribe((value) => {
    if (browser) {
        if (value) {
            localStorage.setItem('user_id', value);
        } else {
            localStorage.removeItem('user_id');
        }
    }
});

export function logout() {
    userId.set(null);
}