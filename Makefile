.PHONY: windows
windows:
	cross build -r --target x86_64-pc-windows-gnu

.PHONY: linux
linux:
	cross build -r --target x86_64-unknown-linux-musl

.PHONY: all
all: linux windows
