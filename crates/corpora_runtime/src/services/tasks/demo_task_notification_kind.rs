#[allow(unused_imports)]
use super::*;

#[derive(Clone, Copy)]
pub(crate) enum DemoTaskNotificationKind {
    Loading,
    Success,
    Cancelled,
}
