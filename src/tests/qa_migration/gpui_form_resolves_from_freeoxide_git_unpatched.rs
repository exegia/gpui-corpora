#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn gpui_form_resolves_from_freeoxide_git_unpatched() {
    let manifest = repo_file("Cargo.toml");
    assert!(
        !manifest.contains("[patch.\"https://github.com/freeoxide/gpui-form\"]"),
        "the retired gpui-form [patch] table must not reappear; the pinned \
         rev is reachable upstream and resolves from git without it"
    );

    let lock = repo_file("Cargo.lock");
    let blocks = lock_blocks(&lock, "gpui-form");
    assert_eq!(
        blocks.len(),
        1,
        "expected exactly one gpui-form in Cargo.lock"
    );
    assert!(
        blocks[0].contains(
            "source = \"git+https://github.com/freeoxide/gpui-form?rev=710b52439f0a4bb7756b6c15eff83c1340d2e157"
        ),
        "gpui-form must resolve from the freeoxide git source at the pinned rev"
    );
}
