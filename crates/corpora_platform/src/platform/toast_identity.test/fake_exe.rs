#[allow(unused_imports)]
use super::*;

pub(crate) fn fake_exe(dir: &std::path::Path, name: &str) -> PathBuf {
    let exe = dir.join(name);
    std::fs::write(&exe, b"stub exe").expect("stub exe");
    exe
}
