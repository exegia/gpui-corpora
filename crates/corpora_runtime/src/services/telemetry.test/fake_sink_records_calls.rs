#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn fake_sink_records_calls() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let sink = FakeSink {
        events: events.clone(),
    };
    sink.record_event("evt").expect("event");
    sink.record_error("oops").expect("error");
    sink.set_user_properties("k", "v").expect("prop");

    let data = events.lock().expect("lock").clone();
    assert_eq!(data, vec!["evt", "err:oops", "prop:k=v"]);
}
