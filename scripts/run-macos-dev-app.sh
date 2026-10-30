#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_DIR="$("$ROOT_DIR/scripts/macos-dev-app.sh")"
EXECUTABLE="$APP_DIR/Contents/MacOS/Corpora"

export RUST_BACKTRACE=1
exec "$EXECUTABLE" "$@"
