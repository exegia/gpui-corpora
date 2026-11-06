//! Per-user URL-scheme registration for deep links. Windows only launches
//! `corpora://` URLs when a protocol handler exists under
//! `Software\Classes`, and unpackaged installs have none — browsers and the
//! shell silently do nothing until this module writes one. Packaged (MSIX or
//! installer) builds register the scheme themselves; this covers the rest.
//! [`ensure_deep_link_registered`] is idempotent and best-effort, mirroring
//! `toast_identity::ensure_toast_identity`.

use std::path::Path;

use crate::platform::process::single_instance::SCHEME;
use crate::platform::toast_identity::same_path;

#[cfg(test)]
#[path = "deep_link_registration.test/mod.rs"]
mod deep_link_registration_test;

mod command_matches;
mod ensure_deep_link_registered;
mod log;
mod scheme_key;

use command_matches::command_matches;
pub use ensure_deep_link_registered::ensure_deep_link_registered;
use log::LOG;
use scheme_key::scheme_key;
