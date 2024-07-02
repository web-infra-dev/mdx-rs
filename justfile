#!/usr/bin/env -S just --justfile

# https://github.com/oxc-project/oxc/blob/main/justfile
_default:
  just --list -u

alias r := ready
alias c := codecov
alias t := test

# Initialize the project by installing all the necessary tools.
# Make sure you have cargo-binstall installed.
# You can download the pre-compiled binary from <https://github.com/cargo-bins/cargo-binstall#installation>
# or install via `cargo install cargo-binstall`
init:
  cargo binstall cargo-nextest cargo-watch cargo-insta cargo-edit typos-cli taplo-cli cargo-llvm-cov -y

# When ready, run the same CI commands
ready:
  typos
  cargo fmt
  just check
  just test
  just lint
  git status

# Format all files
fmt:
  cargo fmt
  taplo format

# Run cargo check
check:
  cargo check --workspace --all-targets --all-features --locked

# Run all the tests
test:
  cargo nextest run

# Lint the whole project
lint:
  cargo clippy --workspace --all-targets --all-features -- --deny warnings

# Get code coverage
codecov:
  cargo codecov --html