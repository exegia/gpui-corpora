#[allow(unused_imports)]
use super::*;

pub fn detect_system_locale() -> String {
    sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
}
