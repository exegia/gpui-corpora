#!/usr/bin/env python3
"""Rebuild and restart Corpora when development files are saved."""

import os
from pathlib import Path
import signal
import subprocess
import sys
import time


WATCH_DIRS = ("src", "crates", "locales", "assets")
WATCH_FILES = ("Cargo.toml", "Cargo.lock", "build.rs", "rust-toolchain.toml")
IGNORED_DIRS = {"target", ".git", "node_modules", "__pycache__"}


def snapshot(root):
    paths = [root / name for name in WATCH_FILES]
    for name in WATCH_DIRS:
        for directory, dirs, files in os.walk(root / name):
            dirs[:] = [d for d in dirs if not d.startswith(".") and d not in IGNORED_DIRS]
            paths.extend(
                Path(directory) / name for name in files
                if not name.startswith(".") and not name.endswith(("~", ".swp", ".tmp"))
            )
    result = {}
    for path in paths:
        try:
            stat = path.stat()
            result[str(path.relative_to(root))] = (stat.st_mtime_ns, stat.st_size)
        except FileNotFoundError:
            pass
    return result


def saves(root):
    previous = snapshot(root)
    yield
    changed_at = None
    while True:
        time.sleep(0.25)
        current = snapshot(root)
        if current != previous:
            previous = current
            changed_at = time.monotonic()
        elif changed_at is not None and time.monotonic() - changed_at >= 0.4:
            changed_at = None
            yield


def stop(app):
    if app is None:
        return
    try:
        if os.name == "posix":
            os.killpg(app.pid, signal.SIGTERM)
        elif app.poll() is None:
            app.terminate()
    except ProcessLookupError:
        return
    try:
        app.wait(timeout=5)
    except subprocess.TimeoutExpired:
        if os.name == "posix":
            os.killpg(app.pid, signal.SIGKILL)
        else:
            app.kill()
        app.wait()


def main():
    root = Path(__file__).resolve().parent.parent
    app = None
    builder = None
    executable = root / "target" / "debug" / ("gpui-corpora.exe" if os.name == "nt" else "gpui-corpora")
    build_command = ["cargo", "build", "--bin", "gpui-corpora", "--target-dir", str(root / "target")]
    if sys.platform == "darwin":
        build_command = ["bash", str(root / "scripts/macos-dev-app.sh")]
        executable = root / "target/debug/Corpora.app/Contents/MacOS/Corpora"

    def interrupted(_signal, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, interrupted)
    print("[dev] Watching src/, crates/, locales/, assets/ and Cargo files. Ctrl+C stops dev mode.", flush=True)
    try:
        for _ in saves(root):
            print("[dev] Building…", flush=True)
            builder = subprocess.Popen(
                build_command,
                cwd=root, start_new_session=True,
            )
            if builder.wait():
                print("[dev] Build failed; keeping the current app. Save a fix to retry.", flush=True)
                continue
            stop(app)
            app = subprocess.Popen([str(executable), *sys.argv[1:]], cwd=root, start_new_session=True)
            print(f"[dev] Started Corpora (PID {app.pid}). Save a file to rebuild.", flush=True)
    except KeyboardInterrupt:
        print("\n[dev] Stopping…", flush=True)
    finally:
        stop(builder)
        stop(app)


if __name__ == "__main__":
    main()
