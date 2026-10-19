# Corpora GPUI

A native macOS workspace bootstrap using Ely's default components and theme,
with `gpui-router` for navigation. The attached design guides the title bar,
sidebar, overview cards, and status bar.

```sh
cargo run
```

Requires Rust 1.95 or newer. Runtime Metal shaders are enabled, so the build
does not require the full Xcode Metal toolchain.

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

## Window

The Ely title bar supplies native macOS traffic-light spacing and its built-in
`drag_region` for dragging and double-click zoom. Home, Explore, and Projects
use an Ely `TabBar`; the `ActivityBar` also opens Settings and Profile.

The title-bar panel buttons toggle the sidebar and activity dock. The dock
starts closed and uses Ely's `Dock` with a movable activity panel. A splash
window precedes the workspace, which briefly shows `Skeleton` placeholders.
Unconnected pages use `EmptyState`.

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

```sh
cargo fmt --check
cargo check --locked
cargo build --locked
```
