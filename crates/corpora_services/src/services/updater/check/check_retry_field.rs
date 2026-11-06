#[allow(unused_imports)]
use super::*;

pub(crate) fn check_retry_field(snap: &mut UpdateSnapshot) -> &mut u32 {
    &mut snap.check_retry_count
}
