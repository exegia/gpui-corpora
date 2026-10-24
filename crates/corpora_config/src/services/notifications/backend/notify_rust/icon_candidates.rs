#[allow(unused_imports)]
use super::*;

#[cfg(target_os = "linux")]
pub(crate) fn icon_candidates(exe_dir: &Path) -> Vec<PathBuf> {
    // 48 is the typical banner size; daemons scale the rest.
    const SIZES: [u32; 5] = [48, 64, 128, 256, 512];
    let hicolor_roots = [
        exe_dir
            .parent()
            .map(|parent| parent.join("share/icons/hicolor")),
        Some(exe_dir.join("hicolor")),
    ];
    let mut candidates = Vec::with_capacity(SIZES.len() * 2 + 1);
    for root in hicolor_roots.into_iter().flatten() {
        for size in SIZES {
            candidates.push(root.join(format!("{size}x{size}/apps/{APP_ICON_NAME}.png")));
        }
    }
    candidates.push(exe_dir.join(format!("resources/{APP_ICON_NAME}.png")));
    candidates
}
