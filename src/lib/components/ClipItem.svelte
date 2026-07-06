<script lang="ts">
  import { TYPE_LABELS, TYPE_BADGE_CLASS, parseBinaryMeta } from '$lib/utils';
  import type { ClipItem } from '$lib/types';

  let {
    item,
    selected,
    onclick,
  }: {
    item: ClipItem;
    selected: boolean;
    onclick: () => void;
  } = $props();
</script>

<div
  class="clip-item"
  class:selected
  class:pinned-item={item.pinned}
  data-index={item.listIndex}
  role="option"
  aria-selected={selected}
  onclick={onclick}
  onkeydown={(e) => { if (e.key === 'Enter') onclick(); }}
  tabindex="-1"
>
  <span class="type-badge {TYPE_BADGE_CLASS[item.type]}">{TYPE_LABELS[item.type]}</span>

  {#if item.type === 'image'}
    {@const meta = parseBinaryMeta(item.preview)}
    <span class="item-preview binary-label">
      {#if meta}
        {meta.format} image{meta.dims ? ` · ${meta.dims}` : ''} · {meta.size}
      {:else}
        image / binary clipboard item
      {/if}
    </span>
  {:else}
    <span class="item-preview">{item.preview || '(empty)'}</span>
  {/if}

  {#if item.pinned}
    <span class="item-pin-dot" title="Pinned"></span>
  {/if}
</div>
