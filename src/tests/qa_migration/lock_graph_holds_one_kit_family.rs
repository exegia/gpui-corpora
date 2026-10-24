#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn lock_graph_holds_one_kit_family() {
    let lock = repo_file("Cargo.lock");

    for (package, version) in [
        ("gpui-kit", "0.7.3"),
        ("gpui-component", "0.7.3"),
        ("gpui-component-macros", "0.7.3"),
        ("gpui-base", "0.7.3"),
        ("gpui-kit-assets", "0.7.3"),
        ("gpui-pre", "0.3.9"),
        ("gpui-pre-platform", "0.3.9"),
    ] {
        assert_single_version(&lock, package, version);
    }
}
