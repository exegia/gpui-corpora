#[allow(unused_imports)]
use super::*;

pub(crate) fn db_path(cx: &App) -> PathBuf {
    crate::app_state::paths(cx).data_dir.join("app.db")
}
