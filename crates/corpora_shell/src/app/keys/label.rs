#[allow(unused_imports)]
use super::*;

/// User-facing label for a keystroke source: Ctrl+K on Windows, ⌘K on macOS.
/// Panics on malformed constants, same contract as KeyBinding::new.
pub fn label(source: &str) -> String {
    let platform = Platform::current();
    let labels = keystroke_labels(&keystroke(source, platform), platform);
    if platform == Platform::Mac {
        labels.concat()
    } else {
        labels.join("+")
    }
}
