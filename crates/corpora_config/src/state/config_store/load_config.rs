#[allow(unused_imports)]
use super::*;

pub(crate) fn load_config(path: &Path) -> (AppConfig, Option<String>) {
    // Cap untrusted input before parsing: serde on an oversized file would
    // allocate unbounded memory at startup.
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return (AppConfig::default(), None);
        }
        Err(err) => return state_read_error(path, err),
    };

    match file.metadata() {
        Ok(meta) if meta.len() > MAX_STATE_FILE_BYTES => {
            quarantine_bad_config(path);
            return oversized_state_error(path);
        }
        Ok(_) => {}
        Err(err) => return state_read_error(path, err),
    }

    let mut json = String::new();
    if let Err(err) = file
        .take(MAX_STATE_FILE_BYTES + 1)
        .read_to_string(&mut json)
    {
        return state_read_error(path, err);
    }
    // The bounded read is the real cap; this catches a file that grew past
    // it between the metadata check and the read.
    if json.len() as u64 > MAX_STATE_FILE_BYTES {
        quarantine_bad_config(path);
        return oversized_state_error(path);
    }

    match serde_json::from_str::<AppConfig>(&json) {
        Ok(config) => {
            let config = crate::state::migrations::migrate(config).normalized();
            // Log-only tier: lints surface unusable-but-loadable values.
            crate::state::config_validation::validate_config(&config);
            (config, None)
        }
        Err(err) => {
            quarantine_bad_config(path);
            (
                AppConfig::default(),
                Some(
                    AppError::StateParse {
                        path: path.to_path_buf(),
                        details: err.to_string(),
                    }
                    .to_string(),
                ),
            )
        }
    }
}
