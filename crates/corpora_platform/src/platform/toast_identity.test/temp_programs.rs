#[allow(unused_imports)]
use super::*;

pub(crate) fn temp_programs(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("corpora-toast-identity-{tag}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}
