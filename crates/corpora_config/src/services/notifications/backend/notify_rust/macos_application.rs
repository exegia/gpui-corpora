use std::sync::OnceLock;

use objc2_foundation::NSBundle;

#[cfg(test)]
#[path = "macos_application.test.rs"]
mod tests;

pub(super) fn macos_application() -> anyhow::Result<()> {
    static APPLICATION: OnceLock<Result<(), String>> = OnceLock::new();
    let result = APPLICATION.get_or_init(|| {
        let bundle_id = NSBundle::mainBundle()
            .bundleIdentifier()
            .ok_or_else(|| "macOS notification fallback requires an app bundle".to_string())?;
        // Explicit identity avoids the dependency's AppleScript lookup of "use_default".
        notify_rust::set_application(&bundle_id.to_string()).map_err(|error| error.to_string())
    });
    result
        .as_ref()
        .copied()
        .map_err(|reason| anyhow::anyhow!("{reason}"))
}
