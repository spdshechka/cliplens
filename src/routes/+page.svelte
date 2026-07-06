<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  import ClipItem    from '$lib/components/ClipItem.svelte';
  import PreviewPanel from '$lib/components/PreviewPanel.svelte';
  import SetupScreen  from '$lib/components/SetupScreen.svelte';
  import Footer       from '$lib/components/Footer.svelte';

  import {
    buildItems,
    loadPinnedRaws,
    savePinnedRaws,
  } from '$lib/utils';
  import { clearPreviewCache } from '$lib/previewCache';
  import type { ClipEntry, ClipItem as ClipItemT, DepStatus } from '$lib/types';

  let deps       = $state<DepStatus | null>(null);
  let entries    = $state<ClipEntry[]>([]);
  let search     = $state('');
  let selectedIndex = $state(0);
  let pinnedRaws = $state<Set<string>>(new Set());
  let removedIds = $state<Set<string>>(new Set());
  let loading    = $state(true);
  let copyError  = $state('');

  let showClearConfirm = $state(false);
  let clearing         = $state(false);
  let clearStatus      = $state<{ ok: boolean; msg: string } | null>(null);

  let searchInput = $state<HTMLInputElement | null>(null);
  let listEl      = $state<HTMLDivElement | null>(null);

  let items = $derived(buildItems(entries, pinnedRaws, removedIds, search));
  let selected = $derived<ClipItemT | null>(items[selectedIndex] ?? null);
  let pinnedCount = $derived(items.filter(i => i.pinned).length);
  let hasPinned   = $derived(pinnedCount > 0);

  $effect(() => {
    void search;
    selectedIndex = 0;
  });

  $effect(() => {
    void selectedIndex;
    void items;
    const el = listEl?.querySelector(`[data-index="${selectedIndex}"]`);
    el?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
  });

  onMount(async () => {
    pinnedRaws = loadPinnedRaws();

    const d = await invoke<DepStatus>('check_dependencies');
    deps = d;

    if (d.cliphist) {
      try {
        entries = await invoke<ClipEntry[]>('list_clipboard_items');
      } catch (e) {
        console.error('Failed to load clipboard items:', e);
      }
    }

    loading = false;
    setTimeout(() => searchInput?.focus(), 50);
  });

  async function closeApp() {
    await getCurrentWindow().close();
  }

  async function copyItem(item: ClipItemT) {
    if (!deps?.wl_copy) return;
    copyError = '';
    try {
      await invoke('copy_clipboard_item', { entry: item.raw });
    } catch (e) {
      copyError = String(e);
    }
  }

  async function copyAndClose(item: ClipItemT) {
    await copyItem(item);
    await closeApp();
  }

  function togglePin(item: ClipItemT) {
    const next = new Set(pinnedRaws);
    if (next.has(item.raw)) {
      next.delete(item.raw);
    } else {
      next.add(item.raw);
    }
    pinnedRaws = next;
    savePinnedRaws(next);
  }

  async function clearHistory() {
    clearing = true;
    clearStatus = null;
    try {
      await invoke('clear_clipboard_history');
      clearPreviewCache();
      entries = [];
      selectedIndex = 0;
      entries = await invoke<ClipEntry[]>('list_clipboard_items');
      clearStatus = { ok: true, msg: 'History cleared' };
      setTimeout(() => { clearStatus = null; }, 2500);
    } catch (e) {
      clearStatus = { ok: false, msg: String(e) };
      setTimeout(() => { clearStatus = null; }, 4000);
    } finally {
      clearing = false;
      showClearConfirm = false;
    }
  }

  function removeItem(item: ClipItemT) {
    removedIds = new Set([...removedIds, item.id]);
    if (selectedIndex >= items.length - 1) {
      selectedIndex = Math.max(0, items.length - 2);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    const inSearch = target === searchInput;

    switch (e.key) {
      case 'Escape':
        e.preventDefault();
        if (showClearConfirm) {
          showClearConfirm = false;
        } else {
          closeApp();
        }
        break;

      case 'ArrowDown':
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, items.length - 1);
        break;

      case 'ArrowUp':
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;

      case 'PageDown':
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 6, items.length - 1);
        break;

      case 'PageUp':
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 6, 0);
        break;

      case 'Home':
        if (inSearch) break;
        e.preventDefault();
        selectedIndex = 0;
        break;

      case 'End':
        if (inSearch) break;
        e.preventDefault();
        selectedIndex = items.length - 1;
        break;

      case 'Enter':
        if (!selected) break;
        e.preventDefault();
        if (e.ctrlKey) {
          copyAndClose(selected);
        } else {
          copyItem(selected);
        }
        break;

      case '/':
        if (!inSearch) {
          e.preventDefault();
          searchInput?.focus();
        }
        break;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="popup-shell">
  <div class="popup">

    <div class="search-header">
      <svg class="search-icon" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2">
        <circle cx="11" cy="11" r="8"/>
        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
      </svg>
      <input
        bind:this={searchInput}
        bind:value={search}
        class="search-input"
        placeholder="Search clipboard…"
        autocomplete="off"
        spellcheck={false}
      />
      {#if search}
        <span class="search-count">{items.length} result{items.length !== 1 ? 's' : ''}</span>
      {/if}
      {#if deps?.cliphist && entries.length > 0}
        <button
          class="clear-btn"
          title="Clear clipboard history"
          onclick={() => { showClearConfirm = true; }}
          disabled={clearing}
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="3 6 5 6 21 6"/>
            <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/>
            <path d="M10 11v6M14 11v6"/>
            <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"/>
          </svg>
        </button>
      {/if}
    </div>

    {#if deps && !deps.wl_copy && deps.cliphist}
      <div class="warning-banner">
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
          <line x1="12" y1="9" x2="12" y2="13"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
        wl-clipboard not found — copy actions are disabled
      </div>
    {/if}

    <div class="main-body">
      <div class="clip-list-panel">
        {#if loading}
          <div class="list-empty">
            <span class="loading-pulse">Loading…</span>
          </div>

        {:else if deps && !deps.cliphist}
          <SetupScreen cliphist_missing={true} wl_copy_missing={false} />

        {:else if items.length === 0 && !search}
          <div class="list-empty">
            <div class="list-empty-icon">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.3" style="color:var(--text-muted)">
                <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
                <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
              </svg>
            </div>
            <div>Clipboard is empty</div>
            <div style="font-size:12px;color:var(--text-muted);">Copy something to get started</div>
          </div>

        {:else if items.length === 0 && search}
          <div class="list-empty">
            <div style="font-size:28px;opacity:.3;">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.4" style="color:var(--text-muted)">
                <circle cx="11" cy="11" r="8"/>
                <line x1="21" y1="21" x2="16.65" y2="16.65"/>
              </svg>
            </div>
            <div>No results for "<em>{search}</em>"</div>
          </div>

        {:else}
          <div class="clip-scroll" bind:this={listEl} role="listbox" aria-label="Clipboard items">
            {#if hasPinned}
              <div class="section-label">Pinned</div>
            {/if}

            {#each items as item (item.id + '_' + item.pinned)}
              {#if !item.pinned && items[item.listIndex - 1]?.pinned && hasPinned}
                <div class="list-divider"></div>
                <div class="section-label">Recent</div>
              {/if}
              <ClipItem
                {item}
                selected={item.listIndex === selectedIndex}
                onclick={() => { selectedIndex = item.listIndex; }}
              />
            {/each}
          </div>
        {/if}
      </div>

      <div class="preview-panel">
        {#if deps && !deps.cliphist}
          <SetupScreen cliphist_missing={true} wl_copy_missing={false} />
        {:else}
          <PreviewPanel
            item={selected}
            wl_copy_available={deps?.wl_copy ?? false}
            onCopy={copyItem}
            onCopyClose={copyAndClose}
            onPin={togglePin}
            onRemove={removeItem}
          />
        {/if}
      </div>
    </div>

    <Footer count={items.length} pinned_count={pinnedCount} />
  </div>
</div>

{#if copyError}
  <div class="toast toast-danger">{copyError}</div>
{/if}

{#if clearStatus}
  <div class="toast" class:toast-danger={!clearStatus.ok} class:toast-ok={clearStatus.ok}>
    {clearStatus.msg}
  </div>
{/if}

{#if showClearConfirm}
  <div class="modal-backdrop" role="dialog" aria-modal="true">
    <div class="modal-box">
      <div class="modal-title">Clear clipboard history?</div>
      <div class="modal-body">
        This will permanently delete all items from cliphist's database.
        Pinned items in ClipLens will also be cleared.
        This cannot be undone.
      </div>
      <div class="modal-actions">
        <button class="btn" onclick={() => { showClearConfirm = false; }}>
          Cancel
        </button>
        <button
          class="btn btn-danger"
          disabled={clearing}
          onclick={clearHistory}
        >
          {clearing ? 'Clearing…' : 'Clear History'}
        </button>
      </div>
    </div>
  </div>
{/if}
