#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "macos")]
pub(crate) fn platform_primary_runtime_status() -> Result<(), String> {
    use objc2_foundation::NSBundle;

    match NSBundle::mainBundle().bundleIdentifier() {
        Some(bundle_id) => {
            tracing::info!(
                target: LOG,
                bundle_id = %bundle_id,
                "macOS bundle identifier detected"
            );
            Ok(())
        }
        None => Err(
            "macOS bundle identifier missing; launch the bundled app from scripts/macos-dev-app.sh instead of raw cargo run"
                .to_string(),
        ),
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn platform_primary_runtime_status() -> Result<(), String> {
    tracing::debug!(target: LOG, "primary runtime accepted on this platform");
    Ok(())
}
