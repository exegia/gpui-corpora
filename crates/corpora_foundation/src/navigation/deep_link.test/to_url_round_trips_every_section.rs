#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn to_url_round_trips_every_section() {
    for &path in crate::navigation::SECTIONS {
        assert_eq!(parse_deep_link(&to_url(path)).unwrap(), path);
    }
    assert_eq!(to_url("/"), "corpora://home");
}
