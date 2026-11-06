#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_update_manifest_serialization() {
    let mut platforms = std::collections::HashMap::new();
    platforms.insert(
        "macos-aarch64".to_string(),
        PlatformAsset {
            url: "https://releases.example.com/app-2.0.0.tar.gz".to_string(),
            signature: "deadbeef".to_string(),
            size: 42_000_000,
        },
    );
    let manifest = UpdateManifest {
        version: "2.0.0".to_string(),
        release_notes: "Major release with new features.".to_string(),
        platforms,
    };

    insta::assert_yaml_snapshot!(
            &manifest,
    @r#"
        version: 2.0.0
        release_notes: Major release with new features.
        platforms:
          macos-aarch64:
            url: "https://releases.example.com/app-2.0.0.tar.gz"
            signature: deadbeef
            size: 42000000
        "#
        );
}
