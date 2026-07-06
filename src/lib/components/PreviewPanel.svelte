<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { MONO_TYPES, TYPE_LABELS, parseBinaryMeta } from '$lib/utils';
  import { previewCache } from '$lib/previewCache';
  import type { ClipItem, ImagePreview } from '$lib/types';

  let {
    item,
    wl_copy_available,
    onCopy,
    onCopyClose,
    onPin,
    onRemove,
  }: {
    item: ClipItem | null;
    wl_copy_available: boolean;
    onCopy: (item: ClipItem) => void;
    onCopyClose: (item: ClipItem) => void;
    onPin: (item: ClipItem) => void;
    onRemove: (item: ClipItem) => void;
  } = $props();

  let isMono = $derived(item ? MONO_TYPES.has(item.type) : false);

  type PreviewState = 'idle' | 'loading' | 'loaded' | 'error';
  let previewState = $state<PreviewState>('idle');
  let previewUrl   = $state<string | null>(null);
  let previewFmt   = $state('');

  $effect(() => {
    const current = item;

    if (!current || current.type !== 'image') {
      previewState = 'idle';
      previewUrl   = null;
      return;
    }

    const cached = previewCache.get(current.raw);
    if (cached !== undefined) {
      previewUrl   = cached;
      previewFmt   = '';
      previewState = cached ? 'loaded' : 'error';
      return;
    }

    previewState = 'loading';
    previewUrl   = null;

    let cancelled = false;
    const raw = current.raw;

    invoke<ImagePreview>('preview_image_item', { entry: raw })
      .then(result => {
        if (cancelled) return;
        previewCache.set(raw, result.data_url);
        previewUrl   = result.data_url;
        previewFmt   = result.format;
        previewState = 'loaded';
      })
      .catch(() => {
        if (cancelled) return;
        previewCache.set(raw, null);
        previewUrl   = null;
        previewState = 'error';
      });

    return () => { cancelled = true; };
  });
</script>

{#if !item}
  <div class="preview-empty">Select an item to preview</div>
{:else}
  <div class="preview-header">
    <span class="preview-type-label">{TYPE_LABELS[item.type]}</span>
    {#if item.type === 'image'}
      {@const meta = parseBinaryMeta(item.preview)}
      {#if previewFmt}
        <span style="font-size:11px;color:var(--text-muted)">
          {previewFmt}{meta?.dims ? ` · ${meta.dims}` : ''}{meta?.size ? ` · ${meta.size}` : ''}
        </span>
      {:else if meta}
        <span style="font-size:11px;color:var(--text-muted)">
          {meta.format}{meta.dims ? ` · ${meta.dims}` : ''} · {meta.size}
        </span>
      {:else}
        <span style="font-size:11px;color:var(--text-muted)">binary / image</span>
      {/if}
    {:else if item.preview.length > 0}
      <span style="font-size:11px;color:var(--text-muted)">{item.preview.length} chars</span>
    {/if}
  </div>

  {#if item.type === 'image'}
    {#if previewState === 'loading'}
      <div class="preview-loading">
        <span class="loading-pulse">Loading image…</span>
      </div>

    {:else if previewState === 'loaded' && previewUrl}
      <div class="preview-image-wrap">
        <img src={previewUrl} alt="" class="preview-img" />
      </div>

    {:else}
      {@const meta = parseBinaryMeta(item.preview)}
      <div style="flex:1;overflow:auto;">
        <div class="preview-binary-card">
          <svg width="38" height="38" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.2" style="color:var(--text-muted);opacity:0.45">
            <rect x="3" y="3" width="18" height="18" rx="2"/>
            <circle cx="8.5" cy="8.5" r="1.5"/>
            <polyline points="21 15 16 10 5 21"/>
          </svg>
          {#if meta}
            <span>{meta.format} image{meta.dims ? ` · ${meta.dims}` : ''}</span>
            <span style="font-size:11px;color:var(--text-muted)">
              {meta.size}{previewState === 'error' ? ' · preview unavailable' : ''}
            </span>
          {:else}
            <span>Image / binary clipboard item</span>
            {#if previewState === 'error'}
              <span style="font-size:11px;color:var(--text-muted)">Preview unavailable</span>
            {/if}
          {/if}
        </div>
      </div>
    {/if}

  {:else if item.type === 'url'}
    <div style="flex:1;overflow:auto;">
      <div class="preview-url-card">
        <div class="preview-url-href">{item.preview}</div>
        <div style="padding:10px 12px;font-size:12px;color:var(--text-muted);">URL — copy to open in browser</div>
      </div>
    </div>

  {:else if item.type === 'email'}
    <div class="preview-content">
      <div style="font-size:11px;color:var(--text-muted);margin-bottom:8px;text-transform:uppercase;letter-spacing:.06em;">Email address</div>
      <div style="color:var(--cyan);font-size:14px;">{item.preview.trim()}</div>
    </div>

  {:else}
    <div class="preview-content" class:mono={isMono}>{item.preview}</div>
  {/if}

  <div class="preview-actions">
    <button
      class="btn btn-primary"
      disabled={!wl_copy_available}
      onclick={() => onCopy(item!)}
    >
      Copy
    </button>
    <button
      class="btn btn-primary"
      disabled={!wl_copy_available}
      onclick={() => onCopyClose(item!)}
    >
      Copy &amp; Close
    </button>
    <button
      class="btn"
      class:btn-cyan={!item.pinned}
      onclick={() => onPin(item!)}
    >
      {item.pinned ? 'Unpin' : 'Pin'}
    </button>
    <button
      class="btn btn-danger"
      onclick={() => onRemove(item!)}
    >
      Remove
    </button>
  </div>
{/if}
