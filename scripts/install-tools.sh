#!/bin/bash

# test runner
if ! command -v cargo-nextest &> /dev/null; then
    echo "Installing cargo-nextest..."
    cargo install cargo-nextest --locked
else
    echo "cargo-nextest already installed"
fi

# security & license checker
if ! command -v cargo-deny &> /dev/null; then
    echo "Installing cargo-deny..."
    cargo install cargo-deny --locked
else
    echo "cargo-deny already installed"
fi

# unused dependency checker
if ! command -v cargo-machete &> /dev/null; then
    echo "Installing cargo-machete..."
    cargo install cargo-machete --locked
else
    echo "cargo-machete already installed"
fi

# database CLI
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli --locked --no-default-features --features postgres
else
    echo "sqlx-cli already installed"
fi

# install node.js tools
pnpm install

# install lefthook (git hooks manager) if missing — pinned via @vX.Y.Z
if ! command -v lefthook >/dev/null 2>&1; then
  if command -v go >/dev/null 2>&1; then
    go install github.com/evilmartians/lefthook/v2@v2.1.10
  else
    echo "go not found on PATH; skipping lefthook install (git hooks will NOT be wired)"
  fi
fi

# wire git hooks via lefthook (config lives in lefthook.yml)
if command -v lefthook >/dev/null 2>&1; then
  lefthook install
fi
