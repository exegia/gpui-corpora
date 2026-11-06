#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn manifest_declares_only_justified_kit_deps() {
    let manifest = repo_file("Cargo.toml");

    // gpui stays beside gpui-kit for one reason: the pinned gpui-form rev's
    // derives emit ::gpui::Entity/Window/Context into this crate.
    assert!(
        manifest.contains("package = \"gpui-pre\", version = \"0.3.9\""),
        "gpui (gpui-pre) must stay on the kit-0.7.3 snapshot"
    );
    assert!(
        manifest.contains("\ngpui-kit = \"0.7.3\""),
        "Cargo.toml must pin gpui-kit to 0.7.3"
    );
    // The layer crates are reached through gpui-kit's own re-exports; the
    // anchor is line start so Cargo.toml's comment prose cannot trip this.
    for dropped in ["gpui_platform", "gpui-component", "gpui-kit-assets"] {
        assert!(
            !manifest.contains(&format!("\n{dropped}")),
            "Cargo.toml must not declare {dropped}; gpui-kit already provides \
             it and qa_migration bans its import paths"
        );
    }
}
