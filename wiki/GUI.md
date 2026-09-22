# GUI

![Main Window](../screenshots/neovimgtk-screen.png?raw=true)

## Header bar

- **Open** (left side) — recent files and bookmarked project directories.
- **New tab** — opens a new tab in the current window.
- **Paste** — pastes from the system clipboard.
- **Save All** — saves every modified buffer.
- The **⋮** menu on the right:
  - **New Window** — opens another instance of the editor.
  - **Sidebar** — toggles the file browser sidebar.
  - **Plugins** — opens the plugin manager.
  - **About** — version and build info.

## Sidebar (file browser)

A directory tree with a folder picker at the top, a "show hidden files" toggle, and a right-click context menu for common file operations. Toggle it from the **⋮** menu or with **Alt+B**, or drag its edge to resize — the width and open/closed state are remembered between sessions.

## Tabs

Each tab is a full Neovim tab page. Drag a file onto the window from a file manager to open it in a new tab (or switch to it, if it's already open) rather than replacing what's currently showing.

## Projects

The **Open** menu tracks recently-opened files and lets you bookmark project directories for quick access later.

## Plugin manager

Manages `vim-plug`-based plugins without hand-editing your Neovim config: browse, install, and enable/disable plugins from a dialog (**⋮ → Plugins**).
