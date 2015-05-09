multitooth: build
	cp target/debug/multitooth $@

build:
	cargo build

.PHONY: build
