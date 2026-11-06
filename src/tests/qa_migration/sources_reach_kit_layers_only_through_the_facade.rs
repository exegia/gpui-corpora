#[allow(unused_imports)]
use super::*;

// The `gpui` crate stays only for gpui-form's derives; hand-written imports
// go through the kit, and the compiler now rejects the layer crates.
#[test]
pub(crate) fn sources_reach_kit_layers_only_through_the_facade() {
    let bans = [
        // (banned path prefix, remedy)
        (
            "use gpui::",
            "import from gpui_kit (root glob or named items)",
        ),
        ("gpui_platform::", "import gpui_kit::platform"),
        ("gpui_component::", "import gpui_kit::component"),
        ("gpui_kit_assets::", "import gpui_kit::assets"),
    ];
    for (path, content) in app_sources() {
        for (banned, remedy) in bans {
            assert!(
                !content.contains(banned),
                "{path} bypasses the kit facade with `{banned}`; {remedy}"
            );
        }
    }
}
