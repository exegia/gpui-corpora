#[allow(unused_imports)]
use super::*;

/// Reads a file from the committed HEAD blob. Cargo rewrites a stale
/// working-tree Cargo.lock during dependency resolution, so only the
/// committed copy shows what a `cargo build --locked` of this commit sees.
pub(crate) fn committed_file(path: &str) -> String {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let output = std::process::Command::new("git")
        .arg("show")
        .arg(format!("HEAD:{path}"))
        .current_dir(&dir)
        .output()
        .unwrap_or_else(|e| panic!("run `git show HEAD:{path}`: {e}"));
    assert!(
        output.status.success(),
        "`git show HEAD:{path}` failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap_or_else(|_| panic!("HEAD:{path} is not utf-8"))
}
