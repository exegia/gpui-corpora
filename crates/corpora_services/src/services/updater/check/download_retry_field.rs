#[allow(unused_imports)]
use super::*;

pub(crate) fn download_retry_field(snap: &mut UpdateSnapshot) -> &mut u32 {
    &mut snap.download_retry_count
}
