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

# install node.js tools
pnpm install

# add husky hooks
pnpm exec husky init
cat > .husky/pre-commit << 'HOOK'
task format && task lint && git add -A .
HOOK
cat > .husky/pre-push << 'HOOK'
task test
HOOK
echo "pnpm exec commitlint --edit \$1" > .husky/commit-msg
