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
ICON_DIR="$ROOT_DIR/crates/corpora_shell/src/shell/assets/app-icon"
# `icon` compiles Corpora.icon (Icon Composer) into Assets.car for the macOS 26
# glass/dark/tinted icon; the others copy a flat .icns.
ICON_VARIANT="${CORPORA_ICON_VARIANT:-icon}"
case "$ICON_VARIANT" in
    icon|light|dark|beta) ;;
    *) echo "Invalid CORPORA_ICON_VARIANT: $ICON_VARIANT" >&2; exit 1 ;;
esac

cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --bin gpui-corpora --target-dir "$ROOT_DIR/target"

rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

cp "$TARGET_DIR/gpui-corpora" "$MACOS_DIR/$APP_NAME"

ICON_FILE="corpora.icns"
ICON_NAME_PLIST=""
if [ "$ICON_VARIANT" = icon ]; then
    ICON_BUILD="$(mktemp -d)"
    if xcrun actool "$ICON_DIR/Corpora.icon" \
        --compile "$ICON_BUILD" \
        --output-format human-readable-text --notices --warnings --errors \
        --output-partial-info-plist "$ICON_BUILD/partial.plist" \
        --app-icon Corpora --include-all-app-icons \
        --enable-on-demand-resources NO --development-region en \
        --target-device mac --minimum-deployment-target 26.0 --platform macosx \
        >"$ICON_BUILD/actool.log" 2>&1 \
        && [ -f "$ICON_BUILD/Assets.car" ] && [ -f "$ICON_BUILD/Corpora.icns" ]; then
        cp "$ICON_BUILD/Assets.car" "$ICON_BUILD/Corpora.icns" "$RESOURCES_DIR/"
        ICON_FILE="Corpora.icns"
        ICON_NAME_PLIST="  <key>CFBundleIconName</key>
  <string>Corpora</string>"
    else
        echo "warning: actool could not compile Corpora.icon (needs Xcode 26+); using light.icns" >&2
        cat "$ICON_BUILD/actool.log" >&2 || true
        ICON_VARIANT=light
    fi
    rm -rf "$ICON_BUILD"
fi
if [ "$ICON_VARIANT" != icon ]; then
    cp "$ICON_DIR/$ICON_VARIANT.icns" "$RESOURCES_DIR/corpora.icns"
fi

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
  <string>$ICON_FILE</string>
$ICON_NAME_PLIST
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
