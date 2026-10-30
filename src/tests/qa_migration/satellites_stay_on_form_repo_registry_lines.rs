#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn satellites_stay_on_form_repo_registry_lines() {
    let lock = repo_file("Cargo.lock");

    let koruma: Vec<&str> = lock
        .split("[[package]]")
        .skip(1)
        .filter(|block| {
            let name = block_name(block);
            name == "koruma" || name.starts_with("koruma-")
        })
        .collect();
    assert!(
        koruma.len() >= 2,
        "expected the koruma family in Cargo.lock, found {}",
        koruma.len()
    );
    for block in &koruma {
        let name = block_name(block);
        assert_eq!(
            lock_blocks(&lock, name).len(),
            1,
            "{name} must exist as exactly one copy in Cargo.lock"
        );
        assert!(
            block.contains("\nversion = \"0.9.0\""),
            "{name} must resolve to 0.9.0"
        );
        assert!(
            block.contains("source = \"registry+https://github.com/rust-lang/crates.io-index\""),
            "{name} must resolve from the registry, not a git/path duplicate"
        );
    }

    let own = lock_blocks(&lock, "gpui-starter");
    assert_eq!(
        own.len(),
        1,
        "expected one gpui-starter block in Cargo.lock"
    );
    assert!(
        !own[0].contains("es-fluent"),
        "gpui-starter must declare no es-fluent dependency; i18n runs on rust-i18n"
    );

    let mut es_fluent: Vec<&str> = lock
        .split("[[package]]")
        .skip(1)
        .map(block_name)
        .filter(|name| name.starts_with("es-fluent"))
        .collect();
    es_fluent.sort_unstable();
    assert_eq!(
        es_fluent,
        ["es-fluent-build", "es-fluent-shared", "es-fluent-toml"],
        "the app-facing es-fluent crates left with the rust-i18n migration; only \
         koruma-collection 0.9.0's non-optional build chain legitimately remains"
    );
    for name in es_fluent {
        let blocks = lock_blocks(&lock, name);
        assert_eq!(
            blocks.len(),
            1,
            "{name} must exist as exactly one copy in Cargo.lock"
        );
        assert!(
            blocks[0]
                .contains("source = \"registry+https://github.com/rust-lang/crates.io-index\""),
            "{name} must resolve from the registry, not a git/path duplicate"
        );
    }

    let i18n = lock_blocks(&lock, "rust-i18n");
    assert_eq!(
        i18n.len(),
        1,
        "expected exactly one rust-i18n in Cargo.lock — the app, gpui-kit, and \
         the gpui-form bridge must share one backend"
    );
    assert!(
        i18n[0].contains("\nversion = \"4."),
        "rust-i18n must resolve to major 4, the line gpui-kit 0.7.1 builds against"
    );
}
