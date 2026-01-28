use crate::engine::TraceEvent;
use crate::model::State;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
/// Reads a UTF-8 text file into memory.
///
/// # Errors
/// Returns an error message if the file cannot be read as UTF-8 or the path is inaccessible.
pub fn read_text_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))
}

/// Writes a UTF-8 text file (no BOM).
///
/// Creates parent directories if needed.
///
/// # Errors
/// Returns an error message if directories cannot be created or the file cannot be written.
pub fn write_text_file(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    fs::write(path, contents).map_err(|e| format!("Failed to write {}: {e}", path.display()))
}

/// Writes a JSON file in a stable, pretty-printed form.
///
/// Creates parent directories if needed.
///
/// # Errors
/// Returns an error message if serialization fails or the file cannot be written.
pub fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
    }
    let s = serde_json::to_string_pretty(value).map_err(|e| format!("json encode: {e}"))?;
    fs::write(path, s).map_err(|e| format!("Failed to write {}: {e}", path.display()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceFile {
    pub trace: Vec<TraceEvent>,
    pub final_state: State,
}
