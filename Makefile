PREFIX?=/usr/local

test:
	RUST_BACKTRACE=1 cargo test

run:
	RUST_LOG=warn RUST_BACKTRACE=1 cargo run $(CARGO_ARGS) -- --no-fork

install: install-resources
	cargo install --locked $(CARGO_ARGS) --path . --force --root $(DESTDIR)$(PREFIX)

install-flatpak: install
	mkdir -p /app/share/metainfo/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui.metainfo.xml /app/share/metainfo/

install-debug: install-resources
	cargo install --locked $(CARGO_ARGS) --debug --path . --force --root $(DESTDIR)$(PREFIX)

install-resources:
	mkdir -p $(DESTDIR)$(PREFIX)/share/neovim-gtk-ui/
	cp -r runtime $(DESTDIR)$(PREFIX)/share/neovim-gtk-ui/
	mkdir -p $(DESTDIR)$(PREFIX)/share/applications/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui.desktop \
		$(DESTDIR)$(PREFIX)/share/applications/io.github.mrjohnnycake.neovim-gtk-ui.desktop
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui_128.png $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/io.github.mrjohnnycake.neovim-gtk-ui.png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/48x48/apps/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui_48.png $(DESTDIR)$(PREFIX)/share/icons/hicolor/48x48/apps/io.github.mrjohnnycake.neovim-gtk-ui.png
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/
	mkdir -p $(DESTDIR)$(PREFIX)/share/icons/hicolor/symbolic/apps/
	cp desktop/io.github.mrjohnnycake.neovim-gtk-ui-symbolic.svg $(DESTDIR)$(PREFIX)/share/icons/hicolor/symbolic/apps/

uninstall:
	rm $(DESTDIR)$(PREFIX)/bin/neovim-gtk-ui
	rm -r $(DESTDIR)$(PREFIX)/share/neovim-gtk-ui/
	rm $(DESTDIR)$(PREFIX)/share/applications/io.github.mrjohnnycake.neovim-gtk-ui.desktop
	rm $(DESTDIR)$(PREFIX)/share/icons/hicolor/128x128/apps/io.github.mrjohnnycake.neovim-gtk-ui.png
	rm $(DESTDIR)$(PREFIX)/share/icons/hicolor/48x48/apps/io.github.mrjohnnycake.neovim-gtk-ui.png
	rm $(DESTDIR)$(PREFIX)/share/icons/hicolor/scalable/apps/io.github.mrjohnnycake.neovim-gtk-ui.svg
	rm $(DESTDIR)$(PREFIX)/share/icons/hicolor/symbolic/apps/io.github.mrjohnnycake.neovim-gtk-ui-symbolic.svg

.PHONY: all clean test uninstall
