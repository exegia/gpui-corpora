#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn forwarded_links_roundtrip_in_order() {
    let dir = tempdir().expect("tempdir");
    let queue = dir.path().join("forward.queue");

    append_forwarded_link(&queue, "corpora://settings");
    append_forwarded_link(&queue, "corpora://notifications");

    let links = drain_forwarded_links(&queue);
    assert_eq!(
        links,
        vec![
            "corpora://settings".to_string(),
            "corpora://notifications".to_string()
        ]
    );
    assert!(drain_forwarded_links(&queue).is_empty());
}
