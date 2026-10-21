#!/usr/bin/env python3
"""Generate app icons from the approved brand artwork on macOS."""

from pathlib import Path
import shutil
import struct
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parent.parent
SKETCHTOOL = "/Applications/Sketch.app/Contents/MacOS/sketchtool"
SPLASH_IDS = "B2311831-53CE-4613-BE6D-D536C5524BAC,FD5548EB-0881-4D5E-84EB-5A82CAE769E3"
BETA_ID = "AE40335B-2295-4E03-AC82-FB9684D882E9"


def run(*args):
    subprocess.run([str(arg) for arg in args], check=True, stdout=subprocess.DEVNULL)


def resize(source, size, output):
    output.parent.mkdir(parents=True, exist_ok=True)
    run("sips", "-z", size, size, source, "--out", output)


def main():
    icons = ROOT / "assets/app-icon"
    if len(sys.argv) > 1:
        source = Path(sys.argv[1]).resolve()
        for folder, ids, scale in ((ROOT / "assets/splash", SPLASH_IDS, 2), (icons, BETA_ID, 1)):
            folder.mkdir(parents=True, exist_ok=True)
            run(SKETCHTOOL, "export", "layers", source, f"--items={ids}", "--formats=png",
                f"--scales={scale}", "--cropped=YES", f"--output={folder}", "--overwriting=YES")

    for variant in ("light", "dark"):
        resize(ROOT / f"assets/brand/{variant}.png", 1024, icons / f"{variant}.png")

    with tempfile.TemporaryDirectory() as temp:
        temp = Path(temp)
        for variant in ("light", "dark", "beta"):
            iconset = temp / f"{variant}.iconset"
            iconset.mkdir()
            for size in (16, 32, 128, 256, 512):
                for scale in (1, 2):
                    suffix = "@2x" if scale == 2 else ""
                    resize(icons / f"{variant}.png", size * scale,
                           iconset / f"icon_{size}x{size}{suffix}.png")
            run("iconutil", "-c", "icns", iconset, "-o", icons / f"{variant}.icns")

        shutil.copyfile(icons / "light.icns", icons / "corpora.icns")
        for size in (16, 22, 24, 32, 48, 64, 128, 256, 512):
            resize(icons / "light.png", size,
                   icons / f"hicolor/{size}x{size}/apps/corpora.png")

        sizes = (16, 24, 32, 48, 64, 128, 256)
        images = []
        for size in sizes:
            png = temp / f"{size}.png"
            resize(icons / "light.png", size, png)
            images.append(png.read_bytes())
        offset = 6 + 16 * len(images)
        entries = []
        for size, image in zip(sizes, images):
            entries.append(struct.pack("<BBBBHHII", size % 256, size % 256,
                                       0, 0, 1, 32, len(image), offset))
            offset += len(image)
        (icons / "corpora.ico").write_bytes(
            struct.pack("<HHH", 0, 1, len(images)) + b"".join(entries) + b"".join(images))
    print("Generated app icon variants, macOS ICNS, Windows ICO, and Linux PNGs.")


if __name__ == "__main__":
    main()
