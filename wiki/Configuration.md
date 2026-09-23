# Configuration

## config.toml

The first time you run the app, it writes a fully-commented `config.toml` to your config directory (`~/.config/neovim-gtk-ui/config.toml`) — every setting is shown with its default, commented out. Uncomment a line and change its value to override that default; anything left commented keeps working as before. The file is never overwritten once it exists, so your edits are safe across upgrades.

| Setting | Default | What it does |
|---|---|---|
| `prefer_dark_theme` | `true` | Use the dark variant of the GTK theme. |
| `show_header_bar` | `true` | Show the header bar (title, menu, buttons) at the top of the window. |
| `window_decorations` | `true` | Show the window's title bar and border. |
| `show_sidebar` | *(remembered)* | Show the file browser sidebar. Left unset, this remembers whatever you last left it as; set explicitly to always start the same way. |
| `show_hidden_files` | `false` | Show hidden files (dotfiles) in the sidebar by default. |
| `font` | *(GNOME font)* | Editor font, as a Pango font description, e.g. `"Iosevka 14"`. Left unset, this follows the GNOME system monospace font live. |
| `font_features` | *(none)* | OpenType font features, e.g. `"cv17, ss01"` for stylistic sets. |
| `linespace` | `0` | Extra pixels added between lines. Can be negative. |
| `transparency` | `1.0` | Window transparency, from `0.0` (invisible) to `1.0` (opaque). |
| `cursor_blink` | `-1` | How many times the cursor blinks before it stops; `-1` blinks forever. |
| `external_popupmenu` | `true` | Render the completion popup as a native GTK widget instead of Neovim's own terminal-style popup. |
| `external_tabline` | `true` | Render the tab bar as a native GTK widget instead of Neovim's own tabline. |
| `external_cmdline` | `false` | Render the command line as a native GTK widget instead of Neovim's own terminal-style command line. |
| `internal_clipboard` | `false` | Use the GTK clipboard directly for the `+`/`*` registers, instead of Neovim's own clipboard provider (e.g. `wl-clipboard`). |

Each setting can also be overridden per-invocation with an environment variable, which wins over `config.toml` for that one run: `NVIM_GTK_PREFER_DARK_THEME`, `NVIM_GTK_NO_HEADERBAR` (inverted), `NVIM_GTK_NO_WINDOW_DECORATION` (inverted). There's also `NVIM_GTK_RUNTIME_PATH` to override where the bundled Neovim runtime files are loaded from — mainly useful when developing the app itself.

## ginit.vim (advanced)

For anything not covered above, the app also responds to commands sent from Neovim's own config, the same way it always has. Add these to `~/.config/nvim/ginit.vim` (checked for with `if exists('g:GuiLoaded')`):

```vim
if exists('g:GuiLoaded')
  GuiFont Iosevka:h14
  GuiFontFeatures cv17
  GuiLinespace 2
  GuiPopupmenu 0
  GuiTabline 0
  GuiCmdline 1
  NGTransparency 0.9 0.9
  NGPreferDarkTheme on
  NGSetCursorBlink 5
endif
```

Other commands available at any time from Neovim's command line: `NGToggleSidebar`, `NGShowProjectView`. For the GTK clipboard, set `let g:GuiInternalClipboard = 1` before Neovim loads its plugins (`config.toml`'s `internal_clipboard` does this for you).

`config.toml` is read by the GTK app itself, before Neovim even starts — these commands run inside Neovim once it's up. If both set the same thing, whichever runs later wins (in practice, `ginit.vim` runs after `config.toml`'s settings are applied, so it takes priority).
