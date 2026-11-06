#[allow(unused_imports)]
use super::*;

pub fn validate_config(cfg: &AppConfig) -> Vec<ConfigLint> {
    let mut lints = Vec::new();

    lints.extend(validate_window_bounds(cfg.window_bounds.as_ref()));
    lints.extend(validate_update_channel(&cfg.update_channel));
    lints.extend(validate_locale(&cfg.locale));
    lints.extend(validate_active_path(&cfg.active_path));
    lints.extend(validate_granted_permissions(&cfg.granted_permissions));

    if !lints.is_empty() {
        tracing::warn!(
            target: LOG_TARGET,
            count = lints.len(),
            "non-fatal config validation produced warnings; config will still load"
        );
    }

    lints
}
