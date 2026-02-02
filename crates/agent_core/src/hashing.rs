use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// SHA-256 hex digest of bytes.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

/// Canonical state hash used by the engine:
/// sha256( stable_json({ step, canonical }) )
///
/// NOTE: BTreeMap ensures key order stability; JSON produced by serde_json for this
/// structure is stable enough for our deterministic contract.
#[must_use]
pub fn state_hash_sha256_from_parts(step: u32, canonical: &BTreeMap<String, String>) -> String {
    let obj = serde_json::json!({
        "step": step,
        "canonical": canonical
    });
    let bytes = serde_json::to_vec(&obj).expect("json encode");
    sha256_hex(&bytes)
}
