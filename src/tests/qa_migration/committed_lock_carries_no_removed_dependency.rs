#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn committed_lock_carries_no_removed_dependency() {
    let committed_lock = committed_file("Cargo.lock");
    let own = lock_blocks(&committed_lock, "gpui-starter");
    assert_eq!(
        own.len(),
        1,
        "expected one gpui-starter block in HEAD's Cargo.lock"
    );
    for package in ["gpui-pre-platform", "gpui-component", "gpui-kit-assets"] {
        assert!(
            !own[0].contains(&format!("\"{package}\"")),
            "HEAD's Cargo.lock still lists {package} as a direct dependency of \
             gpui-starter while Cargo.toml has dropped it. Cargo rewrites a \
             stale working-tree lock during resolution, so the gates stay \
             green while `cargo build --locked` on the commit fails; stage the \
             regenerated Cargo.lock together with the Cargo.toml change"
        );
    }
}
