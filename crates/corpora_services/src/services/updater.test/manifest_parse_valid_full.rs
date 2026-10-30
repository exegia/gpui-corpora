#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn manifest_parse_valid_full() {
    let json = r#"{
        "version": "2.0.0",
        "release_notes": "Big update",
        "platforms": {
            "macos-aarch64": {
                "url": "https://example.com/app.tar.gz",
                "signature": "abc123",
                "size": 12345678
            }
        }
    }"#;
    let manifest: UpdateManifest = serde_json::from_str(json).expect("parse full manifest");
    assert_eq!(manifest.version, "2.0.0");
    assert_eq!(manifest.release_notes, "Big update");
    assert_eq!(manifest.platforms.len(), 1);
    let asset = manifest
        .platforms
        .get("macos-aarch64")
        .expect("platform key");
    assert_eq!(asset.url, "https://example.com/app.tar.gz");
    assert_eq!(asset.signature, "abc123");
    assert_eq!(asset.size, 12345678);
}
