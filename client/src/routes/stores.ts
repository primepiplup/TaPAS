import { writable } from "svelte/store";

function initTagList(): string[] {
	return [];
}

export const tagList = writable(initTagList());
