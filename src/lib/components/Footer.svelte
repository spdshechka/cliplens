<script lang="ts">
  let { count, pinned_count }: { count: number; pinned_count: number } = $props();
</script>

<div class="footer">
  <div class="footer-hints">
    <span class="hint">
      <span class="key">↑↓</span> navigate
    </span>
    <span class="hint">
      <span class="key">Enter</span> copy
    </span>
    <span class="hint">
      <span class="key">Ctrl</span><span class="key">Enter</span> copy &amp; close
    </span>
    <span class="hint">
      <span class="key">Esc</span> close
    </span>
    <span class="hint">
      <span class="key">/</span> search
    </span>
    <span class="hint">
      <span class="key">PgUp</span><span class="key">PgDn</span> jump
    </span>
  </div>
  <div class="footer-status">
    {count} item{count !== 1 ? 's' : ''}{pinned_count > 0 ? ` · ${pinned_count} pinned` : ''}
  </div>
</div>
