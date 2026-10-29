# Corpora artwork

Source: [exegia-ui, Owl Icon page](https://sketch.com/s/01945ba6-1dcb-4f55-bc46-c0cf4d09aa7c/p/290CF47E-E732-46EC-8052-025CCCCE55F5/canvas).

`brand/light.png` and `brand/dark.png` are the approved circular icons supplied
on October 20, 2026, preserved at their original 1159 × 1159 resolution with
transparency. The title bar and splash use these same files, selected by theme.

Regenerate the platform app icons on macOS:

```sh
python3 scripts/export-brand-assets.py
```

The dev app uses `app-icon/Corpora.icon`, the Icon Composer source for the
macOS 26 glass, dark, and tinted icon. `scripts/macos-dev-app.sh` compiles it
with `actool` (Xcode 26+) into `Assets.car` and a fallback `Corpora.icns`, and
sets `CFBundleIconName` to `Corpora`. If `actool` fails, it falls back to
`light.icns`. Set `CORPORA_ICON_VARIANT=light`, `dark`, or `beta` when running
`just mac` to use a flat `.icns` instead. `corpora.icns`, `corpora.ico`, and the
Linux hicolor PNGs use the light artwork.

The startup splash retains Ely's title, tagline, status, progress, and entrance
animation. Only its placeholder mark is replaced by the approved icon.

The earlier full splash wordmarks and beta icon can also be re-exported by
passing the downloaded Sketch document as the script's optional argument.
