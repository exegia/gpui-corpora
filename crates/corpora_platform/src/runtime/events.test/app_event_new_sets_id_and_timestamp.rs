#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn app_event_new_sets_id_and_timestamp() {
    let event = AppEvent::new(AppEventKind::DiagnosticsChanged);
    assert!(!event.id.to_string().is_empty());
    assert!(!event.emitted_at.to_rfc3339().is_empty());
}
