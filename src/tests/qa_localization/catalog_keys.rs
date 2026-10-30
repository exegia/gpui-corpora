#[allow(unused_imports)]
use super::*;

pub(crate) fn catalog_keys(path: &Path) -> BTreeSet<String> {
    let text = read_file(path);
    let mut keys = BTreeSet::new();
    let mut parent = String::new();
    for (index, raw_line) in text.lines().enumerate() {
        let line_number = index + 1;
        if raw_line.trim().is_empty() || raw_line.trim_start().starts_with('#') {
            continue;
        }
        let line = raw_line.trim();
        let colon = line.find(':').unwrap_or_else(|| {
            panic!(
                "{}:{line_number}: missing key colon: {raw_line}",
                path.display()
            )
        });
        let name = &line[..colon];
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            panic!(
                "{}:{line_number}: malformed key {name:?}: {raw_line}",
                path.display()
            );
        }
        let value = line[colon + 1..].trim();
        if !parent.is_empty() && !raw_line.starts_with(' ') {
            parent.clear();
        }
        let location = format!("{}:{line_number}", path.display());
        if value.is_empty() {
            assert!(
                !raw_line.starts_with(' '),
                "{location}: nested map inside a nested map: {raw_line}"
            );
            parent = name.to_string();
            continue;
        }
        assert!(
            value.starts_with('"') && value.ends_with('"') && value.len() >= 2,
            "{location}: value must be a double-quoted scalar: {raw_line}"
        );
        if raw_line.starts_with(' ') {
            assert!(
                !parent.is_empty(),
                "{location}: indented key outside a map: {raw_line}"
            );
            keys.insert(format!("{parent}.{name}"));
        } else {
            keys.insert(name.to_string());
        }
    }
    keys
}
