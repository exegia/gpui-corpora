#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_VERSION="v0.6.3"
CORE_COMMIT="c97f051de18b5f3d90b2073e821f7f9841859af3"
CLI_COMMIT="4cfdcc8eae21cdad3441b0ccb00bf20089e2a5be"
PYTHON_VERSION="3.13.15"
BUILD_ROOT="$ROOT_DIR/target/epub-runtime-build"
CORE_DIR="$BUILD_ROOT/corpora-core"
CLI_DIR="$BUILD_ROOT/corpora-cli"
CORE_TARGET="$BUILD_ROOT/cargo-target"
PYINSTALLER_VERSION="6.22.3"

need() {
    printf "Error: '%s' is required.\n" "$1" >&2
    exit 1
}

command -v cargo >/dev/null 2>&1 || need cargo
command -v uv >/dev/null 2>&1 || need uv
command -v git >/dev/null 2>&1 || need git

if [ "$(uname -s)" = "Darwin" ]; then
    PLATFORM="macos"
elif [ "$(uname -s)" = "Linux" ]; then
    PLATFORM="linux"
else
    printf 'Error: EPUB sidecar runtime is supported on macOS and Linux only.\n' >&2
    exit 1
fi

case "$(uname -m)" in
    arm64|aarch64) ARCH="aarch64" ;;
    x86_64|amd64) ARCH="x86_64" ;;
    *) printf "Error: unsupported CPU architecture '%s'.\n" "$(uname -m)" >&2; exit 1 ;;
esac

if [ -n "${CORPORA_CORE_SOURCE_DIR:-}" ]; then
    CORE_DIR="$(cd "$CORPORA_CORE_SOURCE_DIR" && pwd)"
    ACTUAL_COMMIT="$(git -C "$CORE_DIR" rev-parse HEAD)"
    [ "$ACTUAL_COMMIT" = "$CORE_COMMIT" ] || {
        printf 'Error: corpora-core must be %s (%s); found %s.\n' "$CORE_VERSION" "$CORE_COMMIT" "$ACTUAL_COMMIT" >&2
        exit 1
    }
else
    mkdir -p "$BUILD_ROOT"
    if [ ! -d "$CORE_DIR/.git" ]; then
        git clone --quiet --depth 1 --branch "$CORE_VERSION" https://github.com/exegia/corpora-core.git "$CORE_DIR"
    fi
    ACTUAL_COMMIT="$(git -C "$CORE_DIR" rev-parse HEAD)"
    [ "$ACTUAL_COMMIT" = "$CORE_COMMIT" ] || {
        printf 'Error: expected corpora-core %s (%s), found %s.\n' "$CORE_VERSION" "$CORE_COMMIT" "$ACTUAL_COMMIT" >&2
        exit 1
    }
fi

EXPORTER_PROJECT="$CORE_DIR/packages/epub-to-tf"

[ -f "$CORE_DIR/Cargo.lock" ] || { printf 'Error: Cargo.lock missing in corpora-core source.\n' >&2; exit 1; }
[ -f "$EXPORTER_PROJECT/uv.lock" ] || { printf 'Error: EPUB exporter uv.lock missing.\n' >&2; exit 1; }
if [ ! -d "$CLI_DIR/.git" ]; then
    rm -rf "$CLI_DIR"
    git init --quiet "$CLI_DIR"
    git -C "$CLI_DIR" remote add origin https://github.com/exegia/corpora-cli.git
fi
git -C "$CLI_DIR" fetch --quiet --depth 1 origin "$CLI_COMMIT"
git -C "$CLI_DIR" checkout --quiet --detach FETCH_HEAD
ACTUAL_CLI_COMMIT="$(git -C "$CLI_DIR" rev-parse HEAD)"
[ "$ACTUAL_CLI_COMMIT" = "$CLI_COMMIT" ] || {
    printf 'Error: expected corpora-cli %s, found %s.\n' "$CLI_COMMIT" "$ACTUAL_CLI_COMMIT" >&2
    exit 1
}
[ -f "$CLI_DIR/src/corpora_cli/sidecar.py" ] || { printf 'Error: corpora-cli sidecar source missing.\n' >&2; exit 1; }

CARGO_TARGET_DIR="$CORE_TARGET" cargo build \
    --manifest-path "$CORE_DIR/Cargo.toml" \
    --locked --release -p corpora-convert --bin corpora-epub-check

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/corpora-epub-runtime.XXXXXX")"
trap 'rm -rf "$WORK_DIR"' EXIT
uv python install "$PYTHON_VERSION" --install-dir "$WORK_DIR/python-install" --no-bin
PYTHON_HOME="$(find "$WORK_DIR/python-install" -mindepth 1 -maxdepth 1 -type d -name "cpython-$PYTHON_VERSION-*" -print -quit)"
[ -n "$PYTHON_HOME" ] && [ -x "$PYTHON_HOME/bin/python3.13" ] || {
    printf 'Error: uv did not install the bundled CPython runtime.\n' >&2
    exit 1
}
PYTHON_HOME="$(cd "$PYTHON_HOME" && pwd -P)"
cat > "$WORK_DIR/epub_to_tf_entry.py" <<'PY'
from epub_to_tf import main

if __name__ == "__main__":
    main()
PY

UV_PROJECT_ENVIRONMENT="$WORK_DIR/venv" uv run \
    --locked --project "$EXPORTER_PROJECT" --python 3.13 \
    --with "pyinstaller==$PYINSTALLER_VERSION" pyinstaller \
    --noconfirm --clean --onedir --name epub-to-tf \
    --paths "$EXPORTER_PROJECT/src" \
    --collect-all tf --collect-all lxml \
    --distpath "$WORK_DIR/dist" --workpath "$WORK_DIR/work" \
    --specpath "$WORK_DIR" \
    "$WORK_DIR/epub_to_tf_entry.py"

RUNTIME_DIR="$ROOT_DIR/target/corpora-epub-runtime/$PLATFORM-$ARCH"
STAGED="$WORK_DIR/staged"
mkdir -p "$STAGED"
install -m 755 "$CORE_TARGET/release/corpora-epub-check" "$STAGED/corpora-epub-check"
cp -R "$WORK_DIR/dist/epub-to-tf" "$STAGED/epub-to-tf"
cp -R "$PYTHON_HOME" "$STAGED/python"
PYTHON_SITE_SUFFIX="$("$PYTHON_HOME/bin/python3.13" -c 'import os,sys,sysconfig; print(os.path.relpath(sysconfig.get_paths()["purelib"], sys.prefix))')"
SITE_PACKAGES="$STAGED/python/$PYTHON_SITE_SUFFIX"
mkdir -p "$SITE_PACKAGES/corpora_cli"
for module in __init__.py sidecar.py epub_worker.py epub_runtime.py; do
    install -m 644 "$CLI_DIR/src/corpora_cli/$module" "$SITE_PACKAGES/corpora_cli/$module"
done
cat > "$STAGED/corpora-sidecar.py" <<'PY'
import json

import sys

if sys.argv[1:] != ["--protocol", "1"]:
    print(json.dumps({"protocol": 1, "type": "protocol_error", "code": "incompatible_protocol", "message": "Protocol version 1 is required."}, separators=(",", ":")))
    raise SystemExit(2)

from corpora_cli.sidecar import serve

raise SystemExit(serve())
PY
# Hash-checked bytecode stays valid when copies reset mtimes, so Python never rewrites a signed bundle.
"$STAGED/python/bin/python3.13" -m compileall -q -f -j 0 \
    --invalidation-mode unchecked-hash "$STAGED/python/lib"

if "$STAGED/corpora-epub-check" >"$WORK_DIR/checker-smoke.txt" 2>&1; then
    printf 'Error: corpora-epub-check unexpectedly accepted an empty invocation.\n' >&2
    exit 1
else
    CHECKER_STATUS=$?
fi
[ "$CHECKER_STATUS" -eq 2 ] || { printf 'Error: checker smoke test exited %s.\n' "$CHECKER_STATUS" >&2; exit 1; }
grep -q '^usage: corpora-epub-check INPUT.epub' "$WORK_DIR/checker-smoke.txt"
"$STAGED/epub-to-tf/epub-to-tf" --help >/dev/null
printf '' | CORPORA_EPUB_CHECK_BIN="$STAGED/corpora-epub-check" \
CORPORA_EPUB_TO_TF_BIN="$STAGED/epub-to-tf/epub-to-tf" \
PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 "$STAGED/python/bin/python3.13" "$STAGED/corpora-sidecar.py" --protocol 1 \
    >"$WORK_DIR/sidecar-smoke.jsonl"
grep -Fq '"capabilities":["epub_to_tf"]' "$WORK_DIR/sidecar-smoke.jsonl"

rm -rf "$RUNTIME_DIR"
mkdir -p "$(dirname "$RUNTIME_DIR")"
mv "$STAGED" "$RUNTIME_DIR"
printf '{"core_version":"%s","core_commit":"%s","cli_commit":"%s","python_version":"%s","platform":"%s","architecture":"%s"}\n' \
    "$CORE_VERSION" "$CORE_COMMIT" "$CLI_COMMIT" "$PYTHON_VERSION" "$PLATFORM" "$ARCH" > "$RUNTIME_DIR/runtime.json"
printf '' | CORPORA_EPUB_CHECK_BIN="$RUNTIME_DIR/corpora-epub-check" \
CORPORA_EPUB_TO_TF_BIN="$RUNTIME_DIR/epub-to-tf/epub-to-tf" \
PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 "$RUNTIME_DIR/python/bin/python3.13" "$RUNTIME_DIR/corpora-sidecar.py" --protocol 1 \
    >"$WORK_DIR/relocated-sidecar-smoke.jsonl"
grep -Fq '"capabilities":["epub_to_tf"]' "$WORK_DIR/relocated-sidecar-smoke.jsonl"
printf 'Bundled EPUB runtime: %s\n' "$RUNTIME_DIR"
