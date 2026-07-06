<script lang="ts">
  let {
    cliphist_missing,
    wl_copy_missing,
  }: {
    cliphist_missing: boolean;
    wl_copy_missing: boolean;
  } = $props();
</script>

<div class="setup-screen">
  <div class="setup-icon">
    <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" style="color:var(--danger)">
      <circle cx="12" cy="12" r="10"/>
      <line x1="12" y1="8" x2="12" y2="12"/>
      <line x1="12" y1="16" x2="12.01" y2="16"/>
    </svg>
  </div>

  {#if cliphist_missing}
    <div class="setup-title">cliphist not found</div>
    <div class="setup-subtitle">
      ClipLens requires <strong>cliphist</strong> and <strong>wl-clipboard</strong> to be installed.
      Install them and restart the app.
    </div>

    <div class="setup-commands">
      <div class="setup-command">
        <span class="setup-command-label">Arch</span>
        <span>sudo pacman -S cliphist wl-clipboard</span>
      </div>
      <div class="setup-command">
        <span class="setup-command-label">Nix</span>
        <span>nix-env -iA nixpkgs.cliphist nixpkgs.wl-clipboard</span>
      </div>
    </div>

    <div style="font-size:12px;color:var(--text-muted);max-width:320px;line-height:1.6;">
      You also need to start the cliphist daemon in your Hyprland config:<br/>
      <code style="font-family:monospace;color:var(--text-secondary);">exec-once = wl-paste --watch cliphist store</code>
    </div>

  {:else if wl_copy_missing}
    <div class="setup-title">wl-clipboard not found</div>
    <div class="setup-subtitle">
      Copying is disabled. Install <strong>wl-clipboard</strong> to enable copy actions.
    </div>
    <div class="setup-commands">
      <div class="setup-command">
        <span class="setup-command-label">Arch</span>
        <span>sudo pacman -S wl-clipboard</span>
      </div>
    </div>
  {/if}
</div>
