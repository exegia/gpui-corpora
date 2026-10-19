#[allow(unused_imports)]
use super::*;

/// Verify the base64 Ed25519 `signature_b64` over the SHA-256 of the file at
/// `file_path`. Synchronous; dispatch on a background executor for large files.
/// Also used by `apply::check_pending_swap` to re-verify at swap time — the
/// temp file sits in a shared dir between download and install.
pub(crate) fn verify_ed25519_signature(
    file_path: &std::path::Path,
    signature_b64: &str,
) -> Result<(), String> {
    use base64::Engine as _;
    use ed25519_dalek::{Signature, Verifier as _, VerifyingKey};
    use sha2::Digest;
    use std::io::Read as _;

    let sig_bytes = base64::engine::general_purpose::STANDARD
        .decode(signature_b64)
        .map_err(|e| format!("failed to decode base64 signature: {e}"))?;
    let sig_len = sig_bytes.len();
    let sig_array: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| format!("invalid signature length: expected 64 bytes, got {sig_len}"))?;
    let signature = Signature::from_bytes(&sig_array);

    let pubkey_bytes: [u8; 32] = *UPDATER_PUBLIC_KEY;
    let verifying_key = VerifyingKey::from_bytes(&pubkey_bytes)
        .map_err(|e| format!("invalid updater public key: {e}"))?;

    let mut file = std::fs::File::open(file_path)
        .map_err(|e| format!("failed to open downloaded file for signature check: {e}"))?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file
            .read(&mut buf)
            .map_err(|e| format!("failed to read downloaded file for signature check: {e}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let hash = hasher.finalize();

    verifying_key
        .verify(&hash, &signature)
        .map_err(|e| format!("Ed25519 signature verification failed: {e}"))?;

    Ok(())
}
