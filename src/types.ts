export type StorageBucket = {
  name: string;
  creation_date: number;
};

export type StorageObject = {
  name: string;
  size: number;
  last_modified: number;
};

export type StorageProfile = {
  name: string;
  region: string;
  endpoint: string;
  forcePathStyle: boolean;
  accessKeyId: string;
  secretAccessKey: string;
};
