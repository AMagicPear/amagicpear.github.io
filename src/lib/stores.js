import { writable } from 'svelte/store';

export const isAtTopShowCase = writable(true);

export const currentPage = writable('home');
