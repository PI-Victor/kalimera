import { writable } from "svelte/store";

import type { StorageBucket, StorageObject, StorageProfile } from "./types.ts";

export const buckets = writable<StorageBucket[]>([]);

export const objects = writable<StorageObject[]>([]);

export const profiles = writable<StorageProfile[]>([]);

export const currentProfile = writable<StorageProfile | null>(null);
