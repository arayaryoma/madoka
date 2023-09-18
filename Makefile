.PHONY: build
build:
	@echo "Building..."
	cargo build

.PHONY: install

install: build
	@echo "Installing..."
	cp target/debug/madoka /usr/local/bin/madoka

lint:
	@echo "Linting..."
	cargo clippy -- -D warnings