#!/usr/bin/env bash
set -euo pipefail

: "${CRATES:?CRATES must point to your local crates workspace}"

repo="$CRATES/codexa"
cd "$repo"

cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo package
cargo publish --dry-run

git status --short
git add --all
git commit -m "Initialize Codexa 0.0.1"
git tag -a v0.0.1 -m "Codexa 0.0.1"

gh repo create orbyts/codexa \
  --public \
  --description "Git-native content compiler for typed public and private knowledge artifacts." \
  --source=. \
  --remote=origin \
  --push

git push origin v0.0.1
cargo publish
