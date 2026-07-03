set shell := ["bash", "-c"]
default:
    @just --list
fmt:
    cargo fmt --all
fmt-check:
    cargo fmt --all -- --check
check:
    cargo check --all-targets
clippy:
    cargo clippy --all-targets -- -D warnings
test:
    cargo test --all-features
test-proxy:
    SEIA_TEST_PROXY=http://localhost:7890 cargo test
build:
    cargo build --all-features
ci: fmt-check && clippy && test
