#[allow(unused_imports)]
use super::*;

pub(crate) struct DesktopActionsInner {
    pub(crate) next_watcher_id: u64,
    pub(crate) watchers: BTreeMap<u64, RecommendedWatcher>,
}
