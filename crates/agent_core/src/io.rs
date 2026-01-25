use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceFile {
    pub trace: Vec<crate::engine::TraceEvent>,
    pub final_state: crate::model::State,
}

pub fn read_text_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read {path:?}: {e}"))
}

pub fn write_text_file(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {parent:?}: {e}"))?;
    }
    fs::write(path, contents).map_err(|e| format!("Failed to write {path:?}: {e}"))
}

pub fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir {parent:?}: {e}"))?;
    }
    let s = serde_json::to_string_pretty(value).map_err(|e| format!("json encode: {e}"))?;
    fs::write(path, s).map_err(|e| format!("Failed to write {path:?}: {e}"))
}
