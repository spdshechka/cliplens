# ClipLens

A keyboard-first clipboard popup for Hyprland — a polished frontend for [cliphist](https://github.com/sentriz/cliphist).

ClipLens reads your clipboard history from cliphist and presents it in a fast, glass-style launcher popup. It is **not** a clipboard daemon — cliphist handles storage, ClipLens handles display.

## Requirements

- [`cliphist`](https://github.com/sentriz/cliphist) — clipboard history storage
- [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard) — provides `wl-copy`

```sh
# Arch / CachyOS
sudo pacman -S cliphist wl-clipboard
```

You also need cliphist running as a Wayland clipboard watcher. Add to your Hyprland config:

```
exec-once = wl-paste --watch cliphist store
```

## Dev

```sh
cd ~/projects/cliplens
npm install
npm run tauri dev
```

## Build

```sh
npm run tauri build
```

## Hyprland integration

Bind `SUPER+V` to open ClipLens:

```
# ~/.config/hypr/hyprland.conf

bind = SUPER, V, exec, /path/to/cliplens   # or: ~/.cargo/bin/cliplens after build

windowrulev2 = float,         class:(ClipLens)
windowrulev2 = center,        class:(ClipLens)
windowrulev2 = size 820 560,  class:(ClipLens)
windowrulev2 = noanim,        class:(ClipLens)
windowrulev2 = stayfocused,   class:(ClipLens)
```

> **Note:** verify the exact `class` and `title` with `hyprctl clients` after launching.
> The class is set to `ClipLens` by Tauri's `productName` field in `tauri.conf.json`.

### Blur / opacity

ClipLens uses an **opaque window** to avoid the Wayland `Error 71 (Protocol error)` crash that
`transparent = true` triggers on some compositors. The glass look is CSS-only.

For blur behind the window, use Hyprland's compositor blur instead:

```
windowrulev2 = opacity 0.92 0.92, class:(ClipLens)
layerrule = blur, class:(ClipLens)
```

Or simply leave the window fully opaque — the dark glass style still looks clean without wallpaper blur.

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate list |
| `PgUp` / `PgDn` | Jump 6 items |
| `Home` / `End` | First / last item |
| `Enter` | Copy selected |
| `Ctrl+Enter` | Copy and close |
| `Esc` | Close |
| `/` | Focus search |

## Features

- Glass-style dark popup, no window decorations
- Full keyboard navigation, search focused on open
- Type detection: URL, email, shell command, git, code, image/binary
- Pinned items — persisted in `localStorage`, shown at top, no cliphist mutation
- Preview panel with context-aware display (mono for code/commands, URL card, etc.)
- Action buttons: Copy, Copy & Close, Pin/Unpin, Remove from view
- Binary/image entries shown as placeholder — raw bytes never displayed
- Graceful degradation when cliphist or wl-copy is missing

## Stack

- [Tauri v2](https://tauri.app) + Rust backend
- [SvelteKit](https://kit.svelte.dev) + Svelte 5 + TypeScript
- [Tailwind CSS v3](https://tailwindcss.com)
- cliphist + wl-copy (no bundled daemon)
