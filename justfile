set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-c"]

[doc('Compile every crate and target without running it')]
check:
    cargo check --workspace --all-targets

[doc('Run the continuous integration pipeline')]
ci:
    cargo xtask ci

[doc('Format every crate')]
fmt:
    cargo +nightly fmt --all

[doc('Run a fuzzer, or smoke for a short run of each: just fuzz <fuzzer> [seed] [cases]')]
fuzz *arguments:
    cargo xtask fuzz {{ arguments }}

[doc('Rewrite the generated modules from the pinned specifications')]
generate:
    cargo xtask generate

[doc('Lint the library, the tests, the examples, and xtask with tigerstyle-lsp and clippy, and check the format')]
lint:
    tigerstyle-lsp check src tests examples xtask --format concise
    cargo clippy --workspace --all-targets
    cargo +nightly fmt --all --check

[default]
[doc('List every recipe')]
list:
    @just --list

[doc('Download the current Atlassian specifications into spec')]
specification:
    cargo xtask specification

[doc('Run every test suite')]
test:
    cargo test --workspace
