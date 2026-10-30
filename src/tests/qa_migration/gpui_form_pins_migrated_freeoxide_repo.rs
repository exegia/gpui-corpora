#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn gpui_form_pins_migrated_freeoxide_repo() {
    let manifest = repo_file("Cargo.toml");
    assert!(
        manifest.contains(
            "gpui-form = { git = \"https://github.com/freeoxide/gpui-form\", \
             rev = \"710b52439f0a4bb7756b6c15eff83c1340d2e157\" }"
        ),
        "gpui-form must pin the freeoxide fluent-to-rust-i18n merge byte-exact"
    );
    assert!(
        !manifest.contains("stayhydated"),
        "the retired stayhydated fork must not reappear in Cargo.toml"
    );
    assert!(
        !manifest.contains("gpui-form-collection"),
        "gpui-form-collection has no counterpart in the migrated repo"
    );
    assert!(
        manifest.contains("gpui-query = { version = \"0.3\", features = [\"hook\"] }"),
        "the gpui-query requirement must keep matching the bridge's stamped version"
    );
    assert!(
        manifest.contains(
            "gpui-query = { git = \"https://github.com/hmziqagent/gpui-query\", rev = \"69a071f\" }"
        ),
        "the gpui-query [patch.crates-io] override must stay untouched"
    );

    let lock = repo_file("Cargo.lock");
    assert!(
        !lock.contains("stayhydated"),
        "Cargo.lock must carry zero stayhydated URLs"
    );
    assert!(
        lock_blocks(&lock, "gpui-form-collection").is_empty(),
        "gpui-form-collection must stay absent from the lock"
    );
    assert_single_version(&lock, "gpui-form", "0.5.2");
    let query = lock_blocks(&lock, "gpui-query");
    assert_eq!(
        query.len(),
        1,
        "expected exactly one gpui-query in Cargo.lock"
    );
    assert!(
        query[0].contains("source = \"git+https://github.com/hmziqagent/gpui-query?rev=69a071f"),
        "gpui-query must resolve from the hmziqagent git source"
    );
}
