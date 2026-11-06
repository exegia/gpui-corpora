#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn localize_literals_resolve_in_the_catalog() {
    let en = catalog_keys(&manifest_path("locales/en.yml"));
    let mut sources = Vec::new();
    rust_sources(&manifest_path("src"), &mut sources);
    assert!(!sources.is_empty(), "no rust sources found under src/");

    let mut failures = Vec::new();
    let mut checked = 0;
    for path in &sources {
        let content = read_file(path);
        for (offset, key) in localize_literals(&content) {
            if MISSING_BY_DESIGN.contains(&key.as_str()) {
                continue;
            }
            checked += 1;
            let line_number = content[..offset].matches('\n').count() + 1;
            if !en.contains(&key) {
                failures.push(format!(
                    "{}:{line_number}: \"{key}\" missing from locales/en.yml",
                    path.display()
                ));
            }
        }
    }
    assert!(
        checked > 0,
        "scanner matched no localize literals; facade renamed?"
    );
    assert!(
        failures.is_empty(),
        "unresolved localize keys:\n{}",
        failures.join("\n")
    );
}
