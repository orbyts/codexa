#!/usr/bin/env bash
set -euo pipefail

: "${CRATES:?CRATES must point to your local crates workspace}"
source_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
destination="$CRATES/codexa"

if [[ -e "$destination" ]]; then
  printf 'Destination already exists: %s\n' "$destination" >&2
  exit 1
fi

mkdir -p "$CRATES"
cp -R "$source_dir" "$destination"
rm -rf "$destination/.git"
cd "$destination"
git init -b main
printf 'Installed Codexa scaffold at %s\n' "$destination"
