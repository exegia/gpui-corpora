#[allow(unused_imports)]
use super::*;

// `secondary-` is gpui's cross-platform modifier: cmd on macOS, ctrl on
// Windows and Linux. Binding with cmd- would hit the platform
// modifier, which is the Windows key there and mostly OS-intercepted.
pub const TOGGLE_SEARCH: &str = "secondary-k";
