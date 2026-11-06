#[allow(unused_imports)]
use super::*;

/// Returns every `.rs` file the crate compiles or runs as a test target —
/// the src/ tree, build.rs when present, and tests/*.rs — minus this file,
/// whose own assertions spell the banned paths they check for. Sorted so
/// failures name files stably.
pub(crate) fn app_sources() -> Vec<(String, String)> {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut files = Vec::new();
    let mut stack = vec![root.join("src")];
    while let Some(dir) = stack.pop() {
        let mut entries: Vec<_> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("read dir {}: {e}", dir.display()))
            .map(|e| e.unwrap().path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let content = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
                files.push((path.display().to_string(), content));
            }
        }
    }
    let build = root.join("build.rs");
    if build.exists() {
        files.push((
            build.display().to_string(),
            std::fs::read_to_string(&build).expect("read build.rs"),
        ));
    }
    let mut tests: Vec<_> = std::fs::read_dir(root.join("tests"))
        .unwrap_or_else(|e| panic!("read dir {}: {e}", root.join("tests").display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "rs"))
        .collect();
    tests.sort();
    for path in tests {
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        files.push((path.display().to_string(), content));
    }
    files.retain(|(path, _)| !path.ends_with("qa_migration.rs"));
    files
}
