# GUI

![Main Window](../screenshots/neovimgtk-screen.png?raw=true)

## Header bar

- **Open** (left side) — click the label to open the file chooser directly, or the dropdown arrow next to it for recent files and bookmarked project directories.
- **New tab** — opens a new tab in the current window.
- **Paste** — pastes from the system clipboard.
- **Save** — saves the current tab.
- The **⋮** menu on the right:
  - **New Window** — opens another instance of the editor.
  - **Sidebar** — toggles the file browser sidebar.
  - **About** — version and build info.

## Copy/paste

Right-click the editor for a Copy/Paste context menu, or use **Ctrl+Shift+C**/**Ctrl+Shift+V** — both work against the system clipboard (the same one the header bar's Paste button uses). Plain Ctrl+C/Ctrl+V are left alone, since Vim already uses them (Escape-equivalent and Visual Block mode, respectively).

## Sidebar (file browser)

A directory tree with a folder picker at the top, a "show hidden files" toggle, and a right-click context menu for common file operations. Toggle it from the **⋮** menu or with **Alt+B**, or drag its edge to resize — the width and open/closed state are remembered between sessions.

## Tabs

Each tab is a full Neovim tab page. Drag a file onto the window from a file manager to open it in a new tab (or switch to it, if it's already open) rather than replacing what's currently showing.

## Projects

The **Open** dropdown tracks recently-opened files and lets you bookmark project directories for quick access later.
