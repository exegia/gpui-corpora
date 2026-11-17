#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
cargo build --locked --bin gpui-corpora
BUILD_DIR="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
APP_DIR="$ROOT_DIR/target/Corpora QA.app"
mkdir -p "$APP_DIR/Contents/MacOS"
cp "$BUILD_DIR/debug/gpui-corpora" "$APP_DIR/Contents/MacOS/Corpora QA"
cat > "$APP_DIR/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>Corpora QA</string>
<key>CFBundleIdentifier</key><string>com.corpora.qa</string>
<key>CFBundleName</key><string>Corpora QA</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
codesign --force --sign - "$APP_DIR"
export CORPORA_QA_MODE=1
exec "$APP_DIR/Contents/MacOS/Corpora QA"
