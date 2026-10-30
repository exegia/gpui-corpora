#[allow(unused_imports)]
use super::*;

pub(crate) fn empty(
    id: &'static str,
    icon: IconName,
    title: &'static str,
    body: &'static str,
) -> EmptyState {
    EmptyState::new(id, icon, title).body(body)
}
