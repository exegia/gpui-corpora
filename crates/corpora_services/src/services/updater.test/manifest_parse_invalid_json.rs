#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn manifest_parse_invalid_json() {
    let json = r#"not valid json"#;
    assert!(serde_json::from_str::<UpdateManifest>(json).is_err());
}
