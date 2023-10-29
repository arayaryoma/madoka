.PHONY: build_release
build_release:
	@echo "Building..."
	cargo build --release

.PHONY: build_debug
build_debug:
	@echo "Building..."
	cargo build

.PHONY: install
install: build_release
	@echo "Installing..."
	cp target/debug/madoka /usr/local/bin/madoka

.PHONY: install_debug
install_debug:
	@echo "Installing with debug build..."
	cargo install --debug --path .


lint:
	@echo "Linting..."
	cargo clippy -- -D warnings
