#!/usr/bin/env bash
#
# Reset the checkout and the app's on-disk state back to first-run condition.
#
# Scopes (default: build app repo — everything except `cargo`):
#   build   target/ via `cargo clean`, plus packaging staging residue
#   app     Corpora's config/data/cache dirs (state.json, logs, crash
#           reports, deep-link queue) — the app comes up as a fresh install
#   repo    gitignored junk in the working copy (single-instance lock,
#           __pycache__, .DS_Store)
#   cargo   ~/.cargo registry and git caches, shared by every crate on this
#           machine; opt-in because the next build re-downloads them
#   all     every scope above, including cargo
#
# Never touched: .env, *.local.json, *.local.toml, and anything git tracks.
# Those hold credentials and local overrides you would have to re-create.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

DRY_RUN="${DRY_RUN:-}"
ASSUME_YES=""
DO_BUILD=0
DO_APP=0
DO_REPO=0
DO_CARGO=0
REMOVED=0

usage() {
  cat <<'EOF'
Usage: scripts/clean.sh [options] [scope...]

Scopes:
  build     cargo clean + packaging staging residue
  app       Corpora config/data/cache dirs (fresh-install state)
  repo      gitignored working-copy junk (locks, __pycache__, .DS_Store)
  cargo     ~/.cargo registry + git caches (opt-in; re-downloaded later)
  all       every scope, cargo included

With no scope: build app repo.

Options:
  -n, --dry-run   List what would be removed, remove nothing (or DRY_RUN=1)
  -y, --yes       Skip the confirmation prompt for the app scope
  -h, --help      Show this help

Examples:
  just clean                 # build + app + repo
  just clean build           # just the target dir
  just clean-all             # adds the ~/.cargo caches
  scripts/clean.sh -n all    # see the full blast radius first
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    build) DO_BUILD=1 ;;
    app)   DO_APP=1 ;;
    repo)  DO_REPO=1 ;;
    cargo) DO_CARGO=1 ;;
    all)   DO_BUILD=1; DO_APP=1; DO_REPO=1; DO_CARGO=1 ;;
    -n|--dry-run) DRY_RUN=1 ;;
    -y|--yes) ASSUME_YES=1 ;;
    -h|--help) usage; exit 0 ;;
    *)
      echo "error: unknown argument '$1'" >&2
      usage >&2
      exit 1
      ;;
  esac
  shift
done

if (( !DO_BUILD && !DO_APP && !DO_REPO && !DO_CARGO )); then
  DO_BUILD=1
  DO_APP=1
  DO_REPO=1
fi

# rm -rf with a literal path is one typo away from a catastrophe; every
# deletion in this script goes through here.
remove() {
  local path="$1"
  case "$path" in
    ""|"/"|"$HOME"|"$HOME/")
      echo "error: refusing to remove '$path'" >&2
      exit 1
      ;;
  esac
  if [ ! -e "$path" ] && [ ! -L "$path" ]; then
    return
  fi
  local size
  size="$(du -sh "$path" 2>/dev/null | cut -f1 || true)"
  printf '    rm  %s%s\n' "${path/#$HOME/\~}" "${size:+  ($size)}"
  [ -n "$DRY_RUN" ] || rm -rf "$path"
  REMOVED=$((REMOVED + 1))
}

# Matches AppPaths::new (directories v6, ProjectDirs::from("com","corpora","Corpora")).
app_dirs() {
  case "$(uname -s)" in
    Darwin)
      printf '%s\n' \
        "$HOME/Library/Application Support/com.corpora.Corpora" \
        "$HOME/Library/Caches/com.corpora.Corpora"
      ;;
    Linux)
      printf '%s\n' \
        "${XDG_CONFIG_HOME:-$HOME/.config}/corpora" \
        "${XDG_DATA_HOME:-$HOME/.local/share}/corpora" \
        "${XDG_CACHE_HOME:-$HOME/.cache}/corpora"
      ;;
    MINGW*|MSYS*|CYGWIN*)
      printf '%s\n' \
        "${APPDATA:-$HOME/AppData/Roaming}/corpora/Corpora" \
        "${LOCALAPPDATA:-$HOME/AppData/Local}/corpora/Corpora"
      ;;
    *)
      echo "warning: unknown OS '$(uname -s)'; skipping app state" >&2
      ;;
  esac
}

app_is_running() {
  command -v pgrep >/dev/null 2>&1 || return 1
  pgrep -x gpui-corpora >/dev/null 2>&1 || pgrep -x Corpora >/dev/null 2>&1
}

confirm() {
  [ -z "$ASSUME_YES" ] || return 0
  [ -z "$DRY_RUN" ] || return 0
  [ -t 0 ] || {
    echo "error: app state removal needs a TTY; pass --yes to proceed" >&2
    exit 1
  }
  local reply
  read -r -p "$1 [y/N] " reply
  case "$reply" in
    y|Y|yes|YES) return 0 ;;
    *) echo "Aborted."; exit 1 ;;
  esac
}

cd "$ROOT_DIR"
echo "==> Cleaning $ROOT_DIR${DRY_RUN:+ (dry run)}"

if (( DO_BUILD )); then
  echo "--> build"
  if [ -e "$ROOT_DIR/target" ]; then
    had_target=1
  else
    had_target=0
  fi
  if [ -z "$DRY_RUN" ] && [ -f "$ROOT_DIR/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
    if cargo clean; then
      # cargo prints its own byte count; the summary still counts target/ once.
      if (( had_target )) && [ ! -e "$ROOT_DIR/target" ]; then
        REMOVED=$((REMOVED + 1))
      fi
    else
      echo "warning: cargo clean failed; removing target/ directly" >&2
    fi
  fi
  # cargo clean leaves behind whatever is not a cargo artifact, so the
  # packaging stages (appimage-tools, deb-stage, ...) outlive it.
  remove "$ROOT_DIR/target"
  remove "$ROOT_DIR/.flatpak-builder"
fi

if (( DO_APP )); then
  echo "--> app"
  existing=()
  while IFS= read -r dir; do
    if [ -n "$dir" ] && [ -e "$dir" ]; then
      existing+=("$dir")
    fi
  done < <(app_dirs)
  if (( ${#existing[@]} == 0 )); then
    echo "    already clean"
  else
    if app_is_running; then
      echo "warning: Corpora looks like it is running; it will rewrite state on exit" >&2
    fi
    if [ -z "$ASSUME_YES" ] && [ -z "$DRY_RUN" ]; then
      printf '    %s\n' "${existing[@]/#$HOME/\~}"
    fi
    confirm "Delete Corpora settings, logs, and crash reports?"
    for dir in "${existing[@]}"; do
      remove "$dir"
    done
  fi
fi

if (( DO_REPO )); then
  echo "--> repo"
  remove "$ROOT_DIR/com.corpora.app.instance"
  while IFS= read -r -d '' path; do
    remove "$path"
  done < <(find "$ROOT_DIR" \
    -name .git -prune -o -name target -prune -o \
    \( -name __pycache__ -o -name .DS_Store \) -print0)
fi

if (( DO_CARGO )); then
  echo "--> cargo"
  cargo_home="${CARGO_HOME:-$HOME/.cargo}"
  confirm "Delete the shared cargo download caches in ${cargo_home/#$HOME/\~}?"
  # registry/index and git/db stay: they are metadata, and re-fetching an
  # index costs more time than dropping it reclaims space.
  for path in \
    "$cargo_home/registry/cache" \
    "$cargo_home/registry/src" \
    "$cargo_home/git/checkouts"
  do
    remove "$path"
  done
fi

echo
if [ -n "$DRY_RUN" ]; then
  echo "==> Dry run: $REMOVED path(s) would be removed"
else
  cold=""
  if (( DO_BUILD || DO_CARGO )); then
    cold=" The next build is a cold build."
  fi
  echo "==> Removed $REMOVED path(s).${cold}"
fi
