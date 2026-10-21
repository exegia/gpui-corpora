# Corpora GPUI

A native macOS workspace bootstrap using Ely's default components and theme,
with `gpui-router` for navigation. The attached design guides the title bar,
activity rail, overview cards, and status bar.

```sh
cargo run
```

Requires Rust 1.95 or newer. Runtime Metal shaders are enabled, so the build
does not require the full Xcode Metal toolchain.

On Linux, install the native dependencies listed in
[`scripts/install-linux-deps.sh`](scripts/install-linux-deps.sh). If Homebrew's
`pkg-config` is first on `PATH`, select the system executable so Cargo can find
the system font and display libraries:

```sh
export PKG_CONFIG=/usr/bin/pkg-config
cargo run --locked
```

Keep that variable set for direct Cargo commands, Clippy, and tests.
`just dev` selects `/usr/bin/pkg-config` automatically on Linux when available;
an explicit `PKG_CONFIG` setting takes precedence.

Automatic updates are disabled unless `GPUI_UPDATE_MANIFEST_URL` is set to a
release manifest URL at build time.

## Development watcher

Quit any running Corpora instance, then start dev mode:

```sh
just mac
# Equivalent alternatives:
just dev
python3 scripts/dev.py
```

Save changes in your editor (`Cmd+S` on macOS) to rebuild and restart the app.
The watcher covers `src/`, `crates/`, `locales/`, `assets/`, root Cargo files,
`build.rs`, and `rust-toolchain.toml`. It ignores build output and temporary
editor files, and groups rapid saves into one rebuild. A failed build leaves
the current app running; saving a fix retries it. Saves during a build trigger
another rebuild. Press `Ctrl+C` to stop the watcher and its app.

On macOS the watcher builds, signs, and launches `target/debug/Corpora.app` so
native notifications have the `com.corpora.app` identity. Unbundled launches
use in-app notifications when native delivery is unavailable.

Development builds include a floating annotation toolbar for native UI feedback.
Saved notes survive rebuilds and are available to local agent tools at
`http://127.0.0.1:4748`. The published
[gpui-agentation 0.3.0](https://emmanuel-defreitas.github.io/gpui-agentation/)
library supplies the overlay with its `native` feature. Release builds exclude
the annotation API and UI.

Install its MCP bridge once:

```sh
cargo install gpui-agentation --version 0.3.0 --locked
```

The checkout's `.codex/config.toml` runs `gpui-agentation --url
http://127.0.0.1:4748/mcp`. Restart the MCP client after changing that configuration
and keep Corpora running in debug mode while using the tools. The native listener
accepts loopback requests and rejects browser origins.

Feedback stays in `target/agentation/feedback.json`. Set `GPUI_AGENTATION_STORE`
to change the path, `GPUI_AGENTATION_PORT` to change the port, or
`GPUI_AGENTATION=0` to disable the overlay. Match a changed port in the MCP
configuration. Run only one app process per feedback store.

`corpora_ui::ely` exposes Agentation's bundled Ely in debug builds and the
standalone Ely crate in release builds. App controls and feedback share one
initialization, asset source, theme, and focus system; linking both Ely copies
would register duplicate GPUI actions. Feedback follows route changes.

## Window

The Ely title bar supplies native macOS traffic-light spacing and its built-in
`drag_region` for dragging and double-click zoom. The activity rail navigates
within the selected tab. Each tab owns its route, retained pages, scroll position,
sidebar visibility, and inspector visibility and width.

The sidebar has a fixed gap beside the workspace; the inspector has a draggable
splitter. Both panels use the shared window background. The title-bar controls
animate their visibility and respect reduced motion. Use Ctrl+B for the sidebar
and Ctrl+Shift+B for the inspector (Cmd on macOS).

The activity dock starts closed and uses Ely's `Dock` with a movable activity
panel. A splash window precedes the workspace, which briefly shows `Skeleton`
placeholders. Unconnected pages use `EmptyState`.

The menu-bar tray opens the workspace or navigates to its main sections.
Settings provides light/dark mode, connectivity previews, a loading preview,
and a native Dock badge preview. The status bar includes `OfflineIndicator`.
Connectivity defaults to offline; the preview does not detect network status.

## Routes

| Page | Path |
| --- | --- |
| Home | `/` |
| Explore / Search (index) | `/explore` |
| Explore detail | `/explore/detail/:type/:id` |
| Explore viewer | `/explore/detail/:type/:id/viewer` |
| Projects / List (index) | `/projects` |
| Project detail | `/projects/detail/:projectId` |
| Create project | `/projects/create` |
| Statuses / List (index) | `/projects/statuses` |
| Licenses / List (index) | `/projects/licenses` |
| License detail | `/projects/licenses/detail/:licenseId` |
| Create license | `/projects/licenses/create` |
| Settings | `/settings` |
| Profile / Information (index) | `/profile` |
| Profile information | `/profile/information` |
| Profile authentication | `/profile/authentication` |

Unknown paths show a page-not-found state. All navigation shares the same
route tree, including when the activity dock is open.

This is a UI bootstrap: corpus search, project/license creation, profile data,
and authentication remain placeholders. Project detail/create pages use Ely's
`PriorityIndicator` and interactive `StatusSelect`; changes stay in memory for
this session. No runtime metrics or saved records are simulated.

## Dependencies and checks

Ely and `gpui-router` are pinned to Git revisions. The linked router README
describes version 0.7, which was not published to crates.io when bootstrapped.
Ely, the router, and the application all resolve to `gpui-pre` 0.3.8.

GPUI Kit is pinned to 0.7.1, which shares that GPUI version. The application
uses Kit's platform entry point and window root, with Ely's assets loaded
before Kit's assets. Both libraries are initialized at startup; theme and
locale changes update both. Ely's root `FocusScope` and inspector remain
installed. See [Ely's interoperability guide](https://elygpui.com/docs/gpui-kit/).

```sh
cargo fmt --check
cargo check --locked
cargo build --locked
```

## GitHub automation

Rust CI checks formatting, Clippy, and tests on `dev`, `next`, and `main`.
Dependabot checks Cargo and GitHub Actions dependencies. The release flow is
`dev` → `next` → `main`; this repository does not use the template's Python
wheel or Make-based release pipeline.

Copilot agent jobs require the organization's **Allow use of Copilot CLI
billed to the organization** policy. See [.github/WORKFLOW.md](.github/WORKFLOW.md)
for configuration and verification commands.
