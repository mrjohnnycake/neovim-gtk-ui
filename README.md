# neovim-gtk-ui

![Main Window](/screenshots/neovimgtk-screen.png?raw=true)

Neovim GTK UI wraps Neovim in a familiar Gnome window for better integration with the Linux desktop. For me it acts as an in-between from moving out of other Linux editors like Kate and gedit to strictly using Neovim. It takes time to learn all of the ins and outs of Neovim so I was looking for a halfway between the two types of editors. Neovim supports anything this GUI does but this window makes things easier to use daily while still allowing me to master Neovim.

This is being developed on Omarchy meaning that it is focused on Wayland and Hyprland on Arch Linux. That is the only scope of my use of this app so if you use a different distro I can't guarantee that it will work on there or not.

**AI in Use**
- This is being developed with support of Claude so if you don't like AI because it stole your milk money when you were little or something I don't know what to say. It works for me just fine.

This began as a fork of [Lyude/geovim-gtk](https://github.com/Lyude/neovim-gtk) which was itself a fork of [daa84/neovim-gtk](https://github.com/daa84/neovim-gtk).

---

For more screenshots and a description of basic usage see [wiki/GUI.md](wiki/GUI.md).

# Configuration
Settings live in `~/.config/neovim-gtk-ui/config.toml`, auto-created on first run with every option
commented out and explained. See [wiki/Configuration.md](wiki/Configuration.md) for the full list,
plus the `ginit.vim` commands available for anything not covered there.

# Install
## From sources
First check [build prerequisites](#build-prerequisites)

By default to `/usr/local`:
```
make install
```
Or to some custom path:
```
make PREFIX=/some/custom/path install
```


# Build prerequisites
## Linux
First install the GTK development packages. On Debian/Ubuntu derivatives
this can be done as follows:
``` shell
apt install libgtk-4-dev
```

On Fedora:
```bash
dnf install atk-devel glib2-devel pango-devel gtk4-devel
```

Then install the latest rust compiler, best with the
[rustup tool](https://rustup.rs/). The build command:
```
cargo build --release
```

As of writing this (Dec 16, 2022) the packaged rust tools in Fedora also work for building.

