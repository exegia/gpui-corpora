#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_NAME="Corpora"
BUNDLE_ID="com.corpora.app"
TARGET_DIR="$ROOT_DIR/target/release"
ICON_DIR="$ROOT_DIR/crates/corpora_shell/src/shell/assets/app-icon"
ICON_SOURCE="$ICON_DIR/Corpora.icon"
ICON_BUILD="$(mktemp -d "${TMPDIR:-/tmp}/corpora-app-icon.XXXXXX")"
trap 'rm -rf "$ICON_BUILD"' EXIT
# Args: <version> [universal|aarch64|x86_64]  (arch defaults to universal)
VERSION="${1:-}"
ARCH="${2:-universal}"
if [ -z "$VERSION" ]; then
    VERSION="$(grep -m1 '^version' "$ROOT_DIR/Cargo.toml" | head -1 | sed 's/.*"\([^"]*\)".*/\1/')"
fi

# Parse version into integer build number (strip dots, keep leading zeros for sort).
BUILD_NUMBER="$(echo "$VERSION" | tr -d '.')"

ARM_BIN="$ROOT_DIR/target/aarch64-apple-darwin/release/gpui-corpora"
X86_BIN="$ROOT_DIR/target/x86_64-apple-darwin/release/gpui-corpora"
[ -d "$ICON_SOURCE" ] || { echo "Error: Corpora icon source not found: $ICON_SOURCE" >&2; exit 1; }
xcrun actool "$ICON_SOURCE" \
  --compile "$ICON_BUILD" \
  --output-format human-readable-text --notices --warnings --errors \
  --output-partial-info-plist "$ICON_BUILD/partial.plist" \
  --app-icon Corpora --include-all-app-icons \
  --enable-on-demand-resources NO --development-region en \
  --target-device mac --minimum-deployment-target 26.0 --platform macosx \
  >"$ICON_BUILD/actool.log" 2>&1 || {
    cat "$ICON_BUILD/actool.log" >&2
    exit 1
  }
[ -f "$ICON_BUILD/Assets.car" ] && [ -f "$ICON_BUILD/Corpora.icns" ] || {
  echo "Error: actool did not produce Corpora.icns and Assets.car" >&2
  cat "$ICON_BUILD/actool.log" >&2
  exit 1
}

RUNTIME_ROOT="${CORPORA_EPUB_RUNTIME_ROOT:-$ROOT_DIR/target/corpora-epub-runtime}"
case "$ARCH" in
  universal)
    RUNTIME_ARCHES=(aarch64 x86_64)
    APP_BINARIES=("$ARM_BIN" "$X86_BIN")
    ;;
  aarch64)
    RUNTIME_ARCHES=(aarch64)
    APP_BINARIES=("$ARM_BIN")
    ;;
  x86_64)
    RUNTIME_ARCHES=(x86_64)
    APP_BINARIES=("$X86_BIN")
    ;;
  *) echo "Error: unknown arch '$ARCH' (expected universal|aarch64|x86_64)" >&2; exit 1 ;;
esac
for binary in "${APP_BINARIES[@]}"; do
  [ -f "$binary" ] || { echo "Error: expected build output not found: $binary" >&2; exit 1; }
done
for runtime_arch in "${RUNTIME_ARCHES[@]}"; do
  runtime_id="macos-$runtime_arch"
  runtime_src="$RUNTIME_ROOT/$runtime_id"
  [ -x "$runtime_src/corpora-epub-check" ] || {
    echo "Error: bundled core checker missing at $runtime_src/corpora-epub-check" >&2
    echo "Build it with scripts/build-epub-runtime.sh on an $runtime_arch Mac first." >&2
    exit 1
  }
  [ -x "$runtime_src/epub-to-tf/epub-to-tf" ] || {
    echo "Error: bundled EPUB exporter missing at $runtime_src/epub-to-tf/epub-to-tf" >&2
    exit 1
  }
done

# Stage each arch under its own dir so the three .app bundles don't collide;
# the .app inside is always "GPUI Starter.app" (the name users see in the DMG).
APP_DIR="$TARGET_DIR/dist/$ARCH/$APP_NAME.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

rm -rf "$APP_DIR"
mkdir -p "$MACOS_DIR" "$RESOURCES_DIR"

cp "$ICON_BUILD/Assets.car" "$ICON_BUILD/Corpora.icns" "$RESOURCES_DIR/"

RUNTIME_BUNDLE_DIR="$RESOURCES_DIR/corpora-tools"
mkdir -p "$RUNTIME_BUNDLE_DIR"
for runtime_arch in "${RUNTIME_ARCHES[@]}"; do
  runtime_id="macos-$runtime_arch"
  runtime_src="$RUNTIME_ROOT/$runtime_id"
  cp -R "$runtime_src" "$RUNTIME_BUNDLE_DIR/$runtime_id"
done

# universal = fat Mach-O (arm64 + x86_64 slices, the default download); the
# slim builds copy a single arch for users who want a smaller download.
case "$ARCH" in
  universal)
    lipo -create "$ARM_BIN" "$X86_BIN" -output "$MACOS_DIR/$APP_NAME"
    ;;
  aarch64)
    cp "$ARM_BIN" "$MACOS_DIR/$APP_NAME"
    ;;
  x86_64)
    cp "$X86_BIN" "$MACOS_DIR/$APP_NAME"
    ;;
  *)
    echo "Error: unknown arch '$ARCH' (expected universal|aarch64|x86_64)" >&2
    exit 1
    ;;
esac
echo "$ARCH .app architectures: $(lipo -archs "$MACOS_DIR/$APP_NAME" 2>&1 || echo unknown)" >&2

cat > "$CONTENTS_DIR/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>
  <key>CFBundleIconFile</key>
  <string>Corpora.icns</string>
  <key>CFBundleIconName</key>
  <string>Corpora</string>
  <key>CFBundleIdentifier</key>
  <string>$BUNDLE_ID</string>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>$VERSION</string>
  <key>CFBundleVersion</key>
  <string>$BUILD_NUMBER</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.14</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

codesign --force --deep --sign - "$APP_DIR"

echo "$APP_DIR"
