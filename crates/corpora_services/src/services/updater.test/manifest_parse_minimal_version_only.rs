#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn manifest_parse_minimal_version_only() {
    let json = r#"{ "version": "1.0.0" }"#;
    let manifest: UpdateManifest = serde_json::from_str(json).expect("parse minimal manifest");
    assert_eq!(manifest.version, "1.0.0");
    assert!(manifest.release_notes.is_empty());
    assert!(manifest.platforms.is_empty());
}
