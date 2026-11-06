#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn gpui_pre_snapshot_family_is_unified() {
    let lock = repo_file("Cargo.lock");

    let family: Vec<&str> = lock
        .split("[[package]]")
        .skip(1)
        .filter(|block| block_name(block).starts_with("gpui-pre"))
        .collect();
    assert!(
        family.len() >= 20,
        "expected the full gpui-pre snapshot family, found {}",
        family.len()
    );

    for block in &family {
        let name = block_name(block);
        if name == "gpui-pre-reqwest" {
            continue;
        }
        assert!(
            block.contains("\nversion = \"0.3.9\""),
            "{name} must sit on the unified 0.3.9 snapshot"
        );
    }
}
