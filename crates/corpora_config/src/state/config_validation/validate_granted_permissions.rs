#[allow(unused_imports)]
use super::*;

pub(crate) fn validate_granted_permissions(granted: &HashSet<String>) -> Vec<ConfigLint> {
    let known: HashSet<&str> = KNOWN_PERMISSIONS.iter().copied().collect();
    let mut out = Vec::new();
    // Sort for deterministic lint ordering (HashSet iteration is randomized).
    let mut unknown: Vec<&String> = granted
        .iter()
        .filter(|p| !known.contains(p.as_str()))
        .collect();
    unknown.sort();
    for perm in unknown {
        out.push(ConfigLint::new(
            format!("granted_permissions.{perm}"),
            LintSeverity::Info,
            format!(
                "granted permission {perm:?} is not recognised by this build \
                 (known: {:?}); it will be ignored",
                KNOWN_PERMISSIONS
            ),
        ));
    }
    out
}
