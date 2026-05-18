# cspell:ignore gsub
.SUFFIXES:

help:
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: sync-version
sync-version:  ## Sync Cargo.toml version from VERSION (source of truth)
	@v=$$(awk '/^[[:space:]]*#/ {next} /^[[:space:]]*$$/ {next} {gsub(/[[:space:]]/,""); print; exit}' VERSION); \
	if [ -z "$$v" ]; then echo "VERSION has no version line" >&2; exit 1; fi; \
	awk -v v="$$v" ' \
	  /^version = ".*"[[:space:]]*#[[:space:]]*source:[[:space:]]*\/VERSION/ { \
	    print "version = \"" v "\"  # source: /VERSION (synced by `make sync-version`; do not edit by hand)"; next \
	  } \
	  { print } \
	' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

.PHONY: check
check:  ## Fast type-check across all targets and features
	cargo check --all-targets --all-features

.PHONY: clippy
clippy:  ## Clippy with warnings denied (CI bar)
	cargo clippy --all-targets --all-features -- --deny warnings

.PHONY: test
test:  ## Run the test suite with all features enabled
	cargo test --all-features

.PHONY: format
format:  ## Format the workspace with rustfmt
	cargo fmt --all

.PHONY: fmt-check
fmt-check:  ## Verify formatting without modifying files
	cargo fmt --all -- --check

.PHONY: cover
cover:  ## Coverage profile + HTML file (cover.out, cover.html)
	@if ! cargo llvm-cov --version >/dev/null 2>&1; then \
		echo "error: cargo-llvm-cov not installed."; \
		echo "  install: cargo install cargo-llvm-cov && rustup component add llvm-tools-preview"; \
		exit 1; \
	fi
	cargo llvm-cov --all-features --lcov --output-path cover.out
	cargo llvm-cov report --html --output-dir target/llvm-cov
	cargo llvm-cov report

.PHONY: cover-open
cover-open: cover  ## Run coverage and open the HTML report in a browser
	cargo llvm-cov --all-features --html --open

.PHONY: list-crate
list-crate:  ## List files that would be packaged into the crates.io tarball
	cargo package --list --allow-dirty

.PHONY: demo
demo:  ## Run the demo example (cargo run --example demo)
	cargo run --example demo

.PHONY: clean
clean:  ## Remove the target/ directory
	cargo clean

.PHONY: verify
verify: fmt-check clippy test  ## Run the full pre-commit gate (fmt, clippy, test)
	@echo "All checks passed."
