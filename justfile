set shell := ["bash", "-c"]
# On Windows just resolves recipe shebangs through the shell named here; without
# it just falls back to `cygpath`, which Git for Windows does not put on PATH,
# so every shebang recipe fails with "could not find cygpath executable".
set windows-shell := ["bash.exe", "-c"]
# `set lists` enables which() (used by the imported celestia-devtools.just);
# `set unstable` gates it.
set unstable
set lists
import "./celestia-devtools.just"

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
ci: fmt-check clippy test

# ── npx distribution (local dry-run) ─────────────────────────────────────────
#
# Wraps the shared recipe from celestia-devtools.just with seia's metadata. CI
# does the actual publish (see .github/workflows/npm-release.yml); locally this
# only stages ./dist and runs `npm pack --dry-run`.
#
#   just npm-dist-local                                       # reassemble root from existing dist/
#   just npm-dist-local 0.1.0 path/to/seia x86_64-pc-windows-msvc
npm-dist-local version='' binary='' target='':
    just npm-dist seia {{version}} {{binary}} {{target}}
