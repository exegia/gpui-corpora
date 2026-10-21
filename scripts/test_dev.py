"""Exercise the watcher with a fake build and app, without launching GPUI."""

import os
from pathlib import Path
import shutil
import runpy
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch


class DevWatcherTest(unittest.TestCase):
    def test_linux_build_uses_system_pkg_config_unless_overridden(self):
        build_environment = runpy.run_path(str(Path(__file__).with_name("dev.py")))["build_environment"]
        with patch("sys.platform", "linux"), patch("os.access", return_value=True):
            with patch.dict(os.environ, {"PATH": "/home/linuxbrew/.linuxbrew/bin"}, clear=True):
                self.assertEqual(build_environment()["PKG_CONFIG"], "/usr/bin/pkg-config")
                self.assertNotIn("PKG_CONFIG", os.environ)
            with patch.dict(os.environ, {"PKG_CONFIG": "/custom/pkg-config"}, clear=True):
                self.assertEqual(build_environment()["PKG_CONFIG"], "/custom/pkg-config")

    def test_other_builds_preserve_pkg_config_discovery(self):
        build_environment = runpy.run_path(str(Path(__file__).with_name("dev.py")))["build_environment"]
        for platform, available in [("darwin", True), ("win32", True), ("linux", False)]:
            with self.subTest(platform=platform, available=available):
                with patch("sys.platform", platform), patch("os.access", return_value=available):
                    with patch.dict(os.environ, {}, clear=True):
                        self.assertNotIn("PKG_CONFIG", build_environment())

    def test_stop_cleans_up_build_children(self):
        stop = runpy.run_path(str(Path(__file__).with_name("dev.py")))["stop"]
        with tempfile.TemporaryDirectory() as directory:
            heartbeat = Path(directory) / "heartbeat"
            child = "import time; from pathlib import Path; p=Path('heartbeat'); " \
                    "exec('while True:\\n p.write_text(str(time.time()))\\n time.sleep(0.05)')"
            parent = subprocess.Popen(
                [sys.executable, "-c",
                 "import subprocess, sys; subprocess.Popen([sys.executable, '-c', sys.argv[1]]).wait()",
                 child], cwd=directory, start_new_session=True,
            )
            try:
                deadline = time.monotonic() + 5
                while not heartbeat.exists() and time.monotonic() < deadline:
                    time.sleep(0.05)
                self.assertTrue(heartbeat.exists())
                stop(parent)
                time.sleep(0.1)
                stopped = heartbeat.read_text()
                time.sleep(0.2)
                self.assertEqual(heartbeat.read_text(), stopped)
            finally:
                stop(parent)

    def test_saved_changes_restart_only_after_success_and_cleanup_on_exit(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ("scripts", "src", "bin", "target/debug"):
                (root / name).mkdir(parents=True)
            shutil.copy(Path(__file__).with_name("dev.py"), root / "scripts/dev.py")
            (root / "scripts/macos-dev-app.sh").write_text(
                "#!/bin/sh\nset -eu\n"
                "cd \"$(dirname \"$0\")/..\"\n"
                "cargo build --bin gpui-corpora\n"
                "mkdir -p target/debug/Corpora.app/Contents/MacOS\n"
                "cp target/debug/gpui-corpora target/debug/Corpora.app/Contents/MacOS/Corpora\n"
            )
            source = root / "src/main.rs"
            source.write_text("initial")
            cargo = root / "bin/cargo"
            cargo.write_text(
                f"#!{sys.executable}\n"
                "from pathlib import Path\n"
                "import time\n"
                "with open('builds', 'a') as log: log.write('build\\n')\n"
                "if Path('slow').exists(): time.sleep(0.8)\n"
                "raise SystemExit(int(Path('fail').exists()))\n"
            )
            app = root / "target/debug/gpui-corpora"
            app.write_text(
                f"#!{sys.executable}\n"
                "import os, time\n"
                "with open('starts', 'a') as log: log.write(f'{os.getpid()}\\n')\n"
                "with open('executables', 'a') as log: log.write(__file__ + '\\n')\n"
                "while True: time.sleep(0.1)\n"
            )
            cargo.chmod(0o755)
            app.chmod(0o755)
            env = {**os.environ, "PATH": str(root / "bin") + os.pathsep + os.environ["PATH"]}
            with (root / "watcher.log").open("w+") as output:
                watcher = subprocess.Popen([sys.executable, str(root / "scripts/dev.py")], env=env,
                                           stdout=output, stderr=output)
                try:
                    def lines(name):
                        path = root / name
                        return path.read_text().splitlines() if path.exists() else []

                    def wait_for(predicate):
                        deadline = time.monotonic() + 8
                        while not predicate():
                            self.assertIsNone(watcher.poll())
                            if time.monotonic() >= deadline:
                                self.fail((root / "watcher.log").read_text())
                            time.sleep(0.05)

                    wait_for(lambda: len(lines("starts")) == 1)
                    if sys.platform == "darwin":
                        wait_for(lambda: len(lines("executables")) == 1)
                        self.assertEqual(lines("executables"), [str((root / "target/debug/Corpora.app/Contents/MacOS/Corpora").resolve())])
                    first_pid = int(lines("starts")[0])
                    (root / "target/output").write_text("ignored")
                    (root / "src/.main.rs.swp").write_text("ignored")
                    time.sleep(1)
                    self.assertEqual(len(lines("builds")), 1)
                    (root / "fail").touch()
                    source.write_text("broken")
                    wait_for(lambda: len(lines("builds")) == 2)
                    time.sleep(0.2)
                    self.assertEqual(len(lines("starts")), 1)
                    os.kill(first_pid, 0)
                    (root / "fail").unlink()
                    source.write_text("fixed")
                    time.sleep(0.05)
                    replacement = root / "src/.main.rs.tmp"
                    replacement.write_text("fixed again")
                    replacement.replace(source)
                    wait_for(lambda: len(lines("starts")) == 2)
                    self.assertEqual(len(lines("builds")), 3)
                    with self.assertRaises(ProcessLookupError):
                        os.kill(first_pid, 0)
                    (root / "slow").touch()
                    source.write_text("build in progress")
                    wait_for(lambda: len(lines("builds")) == 4)
                    source.write_text("saved during build")
                    wait_for(lambda: len(lines("starts")) == 4)
                    self.assertEqual(len(lines("builds")), 5)
                    last_pid = int(lines("starts")[-1])
                finally:
                    watcher.send_signal(signal.SIGINT)
                    watcher.wait(timeout=8)
                self.assertEqual(watcher.returncode, 0)
                with self.assertRaises(ProcessLookupError):
                    os.kill(last_pid, 0)


if __name__ == "__main__":
    unittest.main()
