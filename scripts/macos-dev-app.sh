#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_NAME="Corpora"
BUNDLE_ID="com.corpora.app"
TARGET_DIR="$ROOT_DIR/target/debug"
APP_DIR="$TARGET_DIR/$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"
ICON_VARIANT="${CORPORA_ICON_VARIANT:-light}"
case "$ICON_VARIANT" in
    light|dark|beta) ;;
    *) echo "Invalid CORPORA_ICON_VARIANT: $ICON_VARIANT" >&2; exit 1 ;;
esac

cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --bin gpui-corpora --target-dir "$ROOT_DIR/target"

rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

cp "$TARGET_DIR/gpui-corpora" "$MACOS_DIR/$APP_NAME"
cp "$ROOT_DIR/crates/corpora_shell/src/shell/assets/app-icon/$ICON_VARIANT.icns" "$RESOURCES_DIR/corpora.icns"

cat > "$CONTENTS_DIR/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>
  <key>CFBundleIdentifier</key>
  <string>$BUNDLE_ID</string>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>corpora.icns</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.14</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

codesign --force --sign - "$APP_DIR"
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f "$APP_DIR"

echo "$APP_DIR"
