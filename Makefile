.PHONY: format check build build-release test

# ── Formatting ──

# Auto-format all Rust code and apply clippy auto-fixes
format:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty

# ── Checks ──

# Run full check: compile, formatting, and clippy lints
# Equivalent of `make check`:
check:
	@cargo check
	@cargo fmt --all -- --check
	@cargo clippy -- -D warnings

build-release:
	@cargo build --release

# ── Build ──

build:
	@cargo build

# ── Test ──
test:
	@cargo test
