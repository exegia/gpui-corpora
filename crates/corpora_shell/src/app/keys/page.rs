#[allow(unused_imports)]
use super::*;

/// NavigateToSection keystroke for section slot i (1-based in the UI, 0-based action).
pub fn page(i: usize) -> String {
    format!("secondary-{}", i + 1)
}
