#[allow(unused_imports)]
use super::*;

pub(crate) struct FakeSink {
    pub(crate) events: Arc<Mutex<Vec<String>>>,
}

impl TelemetrySink for FakeSink {
    fn record_event(&self, name: &str) -> Result<(), TelemetryError> {
        self.events.lock().expect("lock").push(name.to_string());
        Ok(())
    }

    fn record_error(&self, error: &str) -> Result<(), TelemetryError> {
        self.events
            .lock()
            .expect("lock")
            .push(format!("err:{error}"));
        Ok(())
    }

    fn set_user_properties(&self, key: &str, value: &str) -> Result<(), TelemetryError> {
        self.events
            .lock()
            .expect("lock")
            .push(format!("prop:{key}={value}"));
        Ok(())
    }

    fn flush(&self) -> Result<(), TelemetryError> {
        Ok(())
    }
}
