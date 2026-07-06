export const previewCache = new Map<string, string | null>();

export function clearPreviewCache(): void {
  previewCache.clear();
}
