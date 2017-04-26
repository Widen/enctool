PREFIX := /usr/local
CARGO := cargo
VERSION = $(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)


.PHONY: all
all: release

.PHONY: clean
clean:
	-rm -r dist target

.PHONY: install
install: release
	install -m 0755 target/release/enctool $(PREFIX)/bin

.PHONY: uninstall
uninstall:
	-rm $(PREFIX)/bin/enctool

.PHONY: debug
debug: target/debug/enctool

.PHONY: release
release: target/release/enctool

.PHONY: dist
dist: dist/enctool-$(VERSION).tar.gz dist/enctool-$(VERSION).tar.gz.sha

target/debug/enctool: $(wildcard src/*.rs)
	$(CARGO) build

target/release/enctool: $(wildcard src/*.rs)
	$(CARGO) build --release

dist/enctool-%.tar.gz: target/release/enctool
	mkdir -p dist
	tar -czf $@ -C target/release enctool

dist/enctool-%.tar.gz.sha: dist/enctool-%.tar.gz
	cd dist && shasum -a 256 enctool-$*.tar.gz > ../$@
