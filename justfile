set shell := ["bash", "-c"]
# `set windows-shell` only governs linewise (non-shebang) recipes on Windows.
# Shebang recipes bypass it and force `just` to call `cygpath` to translate the
# interpreter path — which Git for Windows keeps off PATH, so they die with
# "could not find cygpath executable". To avoid that, every multi-line recipe
# below uses the `[script('bash')]` attribute instead of a `#!` shebang:
# `[script]` resolves the interpreter via PATH (PATHEXT-aware) and never calls
# Windows: PowerShell (the 5.1 floor ships with every Windows; pwsh 7 is
# NOT assumed). Linewise recipes must stay PS-5.1-safe: no `&&` chains,
# `cd X; cmd` instead of `cd X && cmd`. Bash-only recipes use
# [script('bash')] and need Git Bash (or WSL) when actually run.
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command", "[Console]::OutputEncoding=[System.Text.Encoding]::UTF8; $PSDefaultParameterValues['*:Encoding']='utf8';"]
# `set lists` enables which() (used by the imported celestia-devtools.just);
# `set unstable` gates it.
set unstable
set lists
import "./celestia-devtools.just"

default:
    @just --list
fmt:
    just fmt-toml
    cargo fmt --all
fmt-check:
    cargo fmt --all -- --check
check:
    cargo check --all-targets
clippy:
    cargo clippy --all-targets -- -D warnings
test:
    cargo test --all-features
[unix]
test-proxy:
    SEIA_TEST_PROXY=http://localhost:7890 cargo test

[windows]
test-proxy:
    $env:SEIA_TEST_PROXY='http://localhost:7890'; cargo test
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
