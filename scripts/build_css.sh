#!/usr/bin/env bash
# Builds static/css/app.css from templates using the pinned Tailwind standalone CLI (no Node required).
set -euo pipefail

TAILWIND_VERSION="v3.4.17"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/tailwindcss-$TAILWIND_VERSION"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)  PLATFORM="linux-x64" ;;
  Linux-aarch64) PLATFORM="linux-arm64" ;;
  Darwin-x86_64) PLATFORM="macos-x64" ;;
  Darwin-arm64)  PLATFORM="macos-arm64" ;;
  *) echo "Unsupported platform: $(uname -s)-$(uname -m)" >&2; exit 1 ;;
esac

if [[ ! -x "$BIN" ]]; then
  mkdir -p "$ROOT/target"
  curl -fsSL -o "$BIN" \
    "https://github.com/tailwindlabs/tailwindcss/releases/download/$TAILWIND_VERSION/tailwindcss-$PLATFORM"
  chmod +x "$BIN"
fi

cd "$ROOT"
"$BIN" -c tailwind.config.js -i styles/app.css -o static/css/app.css --minify "$@"
