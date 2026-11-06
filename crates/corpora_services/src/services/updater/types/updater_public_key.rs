#[allow(unused_imports)]
use super::*;

/// Ed25519 public key matching `UPDATE_SIGNING_KEY` in CI; swap it to deploy.
pub(crate) const UPDATER_PUBLIC_KEY: &[u8; 32] = include_bytes!("../../updater_public_key.bin");
