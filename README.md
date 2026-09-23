# neovim-gtk-ui

![Main Window](/screenshots/neovimgtk-screen.png?raw=true)

Neovim GTK UI wraps Neovim in a familiar Gnome window for better integration with the Linux desktop. For me it acts as an in-between from moving out of other Linux editors like Kate and gedit to strictly using Neovim. It takes time to learn all of the ins and outs of Neovim so I was looking for a halfway between the two types of editors. Neovim supports anything this GUI does but this window makes things easier to use daily while still allowing me to master Neovim.

This is being developed on Omarchy meaning that it is focused on Wayland and Hyprland on Arch Linux. That is the only scope of my use of this app so if you use a different distro I can't guarantee that it will work on there or not.

**AI in Use**
- Claude helped me out on this app so if you don't like AI because it stole your milk money when you were little or something I don't know what to say. It works for me just fine.

This began as a fork of [Lyude/geovim-gtk](https://github.com/Lyude/neovim-gtk) which was itself a fork of [daa84/neovim-gtk](https://github.com/daa84/neovim-gtk).


# Configuration
Settings live in `~/.config/neovim-gtk-ui/config.toml` and are auto-created on the first run with every default option commented out and explained. See [wiki/Configuration.md](wiki/Configuration.md) for the full list.

Alternatively you can use `ginit.vim` commands available for anything not covered there.

For window tips see [wiki/GUI.md](wiki/GUI.md).


# Install
## Using the included PKGBUILD (recommended)
You can build and install this app as a real pacman-tracked package.

1. Make sure you have the base Arch build tools (most systems already do):
   ```
   sudo pacman -S --needed base-devel
   ```
2. Clone the repo and `cd` into it:
   ```
   git clone https://github.com/mrjohnnycake/neovim-gtk-ui.git
   cd neovim-gtk-ui
   ```
3. Build and install:
   ```
   makepkg -si
   ```
   `-s` installs any missing dependencies (gtk4, gtksourceview5, neovim, rust) via pacman before
   building; `-i` installs the finished package once it's built. You'll be prompted for your
   password for both.
4. Launch it from your app launcher ("Neovim GTK"), or run `neovim-gtk-ui` from a terminal.

With this method you can uninstall any time with `sudo pacman -R neovim-gtk-ui`.

## Manual install
Skips pacman/makepkg entirely and installs to your user directory instead - no `sudo` needed, but
these files won't be tracked by pacman.

1. Install the build dependencies:
   ```
   sudo pacman -S --needed base-devel gtk4 gtksourceview5 rust
   ```
2. Clone the repo and `cd` into it:
   ```
   git clone https://github.com/mrjohnnycake/neovim-gtk-ui.git
   cd neovim-gtk-ui
   ```
3. Build and install:
   ```
   make PREFIX="$HOME/.local" install
   ```
4. Make sure `~/.local/bin` is on your `PATH`, then launch it from your app launcher ("Neovim
   GTK"), or run `neovim-gtk-ui` from a terminal.

Uninstall any time with:
```
make PREFIX="$HOME/.local" uninstall
```

