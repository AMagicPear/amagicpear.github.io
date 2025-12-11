import { writable } from 'svelte/store';

export const isAtTopShowCase = writable<boolean>(true);

export const currentPage = writable('home');
