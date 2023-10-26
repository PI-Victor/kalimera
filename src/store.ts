import { writable } from "svelte/store";
import { openDB } from "idb";

import type { StorageBucket, StorageObject, StorageProfile } from "./types.ts";

const dbPromise = openDB("kalimera-storage", 1, {
    upgrade(db) {
        db.createObjectStore("buckets", { keyPath: "name" });
        db.createObjectStore("objects", { keyPath: "name" });
        db.createObjectStore("profiles", { keyPath: "name" });
    },
});


export const buckets = writable<StorageBucket[]>([]);
export const objects = writable<StorageObject[]>([]);
export const profiles = writable<StorageProfile[]>([]);
export const currentProfile = writable<StorageProfile | null>(null);


export const loadProfiles = async () => {   
    const db = await dbPromise;
    const transaction = db.transaction(["profiles"], "readonly");
    profiles.set(await transaction.objectStore("profiles").getAll());
}

export const loadBuckets = async () => {
    const db = await dbPromise;
    const transaction = db.transaction(["buckets"], "readonly");
    buckets.set(await transaction.objectStore("buckets").getAll());
}

export const loadObjects = async () => {
    const db = await dbPromise;
    const transaction = db.transaction(["objects"], "readonly");
    objects.set(await transaction.objectStore("objects").getAll());
}

export const addBucket = async (bucket: StorageBucket) => {
    const db = await dbPromise;
    await db.add("buckets", bucket);
    buckets.update(buckets => [...buckets, bucket]);
};

export const addObject = async (object: StorageObject) => {
    const db = await dbPromise;
    await db.add("objects", object);
    objects.update(objects => [...objects, object]);
};

export const addProfile = async (profile: StorageProfile) => {
    const db = await dbPromise;
    await db.add("profiles", profile);
    profiles.update(profiles => [...profiles, profile]);
};

export const deleteProfile = async (profileName: string) => {
    console.log(profileName);
    const db = await dbPromise;
    await db.delete("profiles", profileName);
    profiles.update(profiles => profiles.filter(p => p.name !== profileName));
}

export const setCurrentProfile = (profile: StorageProfile | null) => {
    currentProfile.set(profile);
};
