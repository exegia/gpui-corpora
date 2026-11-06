#[allow(unused_imports)]
use super::*;

pub(crate) fn temp_root(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("corpora-notify-icon-{tag}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("temp dir");
    dir
}
