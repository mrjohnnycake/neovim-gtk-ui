# Maintainer: mrjohnnycake <mrjohnnycake@gmail.com>
pkgname=neovim-gtk-ui
pkgver=0.1.0.r0.0000000
pkgrel=1
pkgdesc="GTK4 UI for Neovim, focused on Wayland/Hyprland on Arch Linux"
arch=('x86_64')
url="https://github.com/mrjohnnycake/neovim-gtk-ui"
license=('GPL3')
depends=('gtk4' 'gtksourceview5' 'neovim')
makedepends=('rust' 'git')
source=("$pkgname::git+$url.git")
sha256sums=('SKIP')

pkgver() {
	cd "$pkgname"
	printf '%s.r%s.%s' \
		"$(grep -m1 '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/')" \
		"$(git rev-list --count HEAD)" \
		"$(git rev-parse --short HEAD)"
}

build() {
	cd "$pkgname"
	cargo build --release --locked
}

check() {
	cd "$pkgname"
	cargo test --release --locked
}

package() {
	cd "$pkgname"
	make PREFIX=/usr DESTDIR="$pkgdir" install
	install -Dm644 desktop/io.github.mrjohnnycake.neovim-gtk-ui.metainfo.xml \
		"$pkgdir/usr/share/metainfo/io.github.mrjohnnycake.neovim-gtk-ui.metainfo.xml"
	# cargo install --root leaves its own bookkeeping files behind, which
	# embed this build's temporary $srcdir path - not needed at runtime.
	rm -f "$pkgdir/usr/.crates.toml" "$pkgdir/usr/.crates2.json"
}
