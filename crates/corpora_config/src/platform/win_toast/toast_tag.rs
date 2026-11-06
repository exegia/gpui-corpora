#[allow(unused_imports)]
use super::*;

/// Windows replaces a toast only on a matching (group, tag) pair, so the tag
/// carries the thread id — capped at the pre-Creators-Update limit of 16
/// characters, the same cap user-notify's uuid tags use. Unthreaded sends
/// get a fresh uuid prefix and stack instead of replacing.
pub(crate) fn toast_tag(request: &NotificationRequest) -> String {
    match &request.thread_id {
        Some(thread_id) => thread_id.chars().take(16).collect(),
        None => uuid::Uuid::new_v4().to_string()[..16].to_owned(),
    }
}
