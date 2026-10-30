#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrashReport {
    pub id: String,
    pub panic_message: String,
    pub backtrace: String,
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub timestamp: String,
    pub render_path: bool,
    pub recent_errors: Vec<String>,
}

impl CrashReport {
    pub fn new(
        panic_message: String,
        backtrace: String,
        render_path: bool,
        recent_errors: Vec<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            panic_message: scrub(&panic_message, MAX_PANIC_MESSAGE_CHARS),
            backtrace: scrub(&backtrace, MAX_BACKTRACE_CHARS),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            render_path,
            recent_errors: recent_errors
                .into_iter()
                .take(MAX_RECENT_ERRORS)
                .map(|err| scrub(&err, MAX_RECENT_ERROR_CHARS))
                .collect(),
        }
    }
}
