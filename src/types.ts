export type StorageBucket = {
    name: string;
    creation_date: number;
};

export type StorageObject = {
    name: string;
    size: number;
    last_modified: number;
};
