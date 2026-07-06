import type { ItemType, ClipEntry, ClipItem } from './types';

const URL_RE           = /^(https?|ftp):\/\//i;
const EMAIL_RE         = /^[\w.+%-]+@[\w-]+\.[a-z]{2,}(\s|$)/i;
const GIT_RE           = /^git\s+(clone|commit|push|pull|add|status|log|diff|merge|rebase|stash|fetch|remote|branch|checkout|cherry)/i;
const CMD_RE           = /^(sudo|apt|pacman|yay|paru|pip|pip3|npm|yarn|pnpm|cargo|docker|kubectl|systemctl|journalctl|chmod|chown|ssh|scp|rsync|curl|wget|tar|grep|find|ls|cd|cp|mv|rm|mkdir|echo|cat)\s/;
const CLIPHIST_BIN_RE  = /^\[\[ binary data/i;
const CODE_RE          = /[{}\[\]<>]|^\s*(def |function |const |let |var |import |from |class |return |if |for |while |async |await |export )|\n.*\n/;

export function detectType(preview: string, is_binary: boolean): ItemType {
  if (is_binary) return 'image';
  if (!preview.trim()) return 'unknown';

  const t = preview.trim();

  if (CLIPHIST_BIN_RE.test(t)) return 'image';

  if (URL_RE.test(t))   return 'url';
  if (EMAIL_RE.test(t)) return 'email';
  if (GIT_RE.test(t))   return 'git';
  if (CMD_RE.test(t))   return 'cmd';
  if (CODE_RE.test(t))  return 'code';

  return 'text';
}

export function parseBinaryMeta(
  preview: string
): { format: string; dims: string; size: string } | null {
  const m = preview.match(
    /^\[\[ binary data\s+([\d.]+ \w+)\s+(\w+)(?:\s+(\d+x\d+))?\s*\]\]$/i
  );
  if (!m) return null;
  return {
    size:   m[1],
    format: m[2].toUpperCase(),
    dims:   m[3] ? m[3].replace('x', '×') : '',
  };
}

export const TYPE_LABELS: Record<ItemType, string> = {
  text:    'TXT',
  url:     'URL',
  email:   'MAIL',
  cmd:     'CMD',
  git:     'GIT',
  code:    'CODE',
  image:   'IMG',
  unknown: '???',
};

export const TYPE_BADGE_CLASS: Record<ItemType, string> = {
  text:    'badge-text',
  url:     'badge-url',
  email:   'badge-email',
  cmd:     'badge-cmd',
  git:     'badge-git',
  code:    'badge-code',
  image:   'badge-image',
  unknown: 'badge-unknown',
};

export const MONO_TYPES: Set<ItemType> = new Set(['cmd', 'git', 'code', 'url']);

export function buildItems(
  entries: ClipEntry[],
  pinnedRaws: Set<string>,
  removedIds: Set<string>,
  search: string
): ClipItem[] {
  const q = search.toLowerCase();

  const pinned: ClipItem[] = [];
  const normal: ClipItem[] = [];
  let idx = 0;

  for (const entry of entries) {
    if (removedIds.has(entry.id)) continue;
    const type = detectType(entry.preview, entry.is_binary);
    const isPinned = pinnedRaws.has(entry.raw);
    const item: ClipItem = { ...entry, type, pinned: isPinned, listIndex: -1 };

    if (isPinned) {
      pinned.push(item);
    } else {
      if (!q || entry.preview.toLowerCase().includes(q)) {
        normal.push(item);
      }
    }
  }

  const all = [...pinned, ...normal];
  all.forEach((item, i) => { item.listIndex = i; });
  return all;
}

export function loadPinnedRaws(): Set<string> {
  try {
    const raw = localStorage.getItem('cliplens_pinned');
    if (!raw) return new Set();
    return new Set(JSON.parse(raw));
  } catch {
    return new Set();
  }
}

export function savePinnedRaws(raws: Set<string>): void {
  localStorage.setItem('cliplens_pinned', JSON.stringify([...raws]));
}
