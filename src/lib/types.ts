export type ItemType =
  | 'text'
  | 'url'
  | 'email'
  | 'cmd'
  | 'git'
  | 'code'
  | 'image'
  | 'unknown';

export interface ClipEntry {
  raw: string;
  id: string;
  preview: string;
  is_binary: boolean;
}

export interface ClipItem extends ClipEntry {
  type: ItemType;
  pinned: boolean;
  listIndex: number;
}

export interface DepStatus {
  cliphist: boolean;
  wl_copy: boolean;
}

export interface ImagePreview {
  data_url: string;
  format: string;
  size_bytes: number;
}
