use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Severity {
    HIGH,
    MEDIUM,
    LOW,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueKind {
    DuplicateKey,
    InvalidToken,
    MissingRequired,
    OutOfRange,
    NonCanonicalOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub severity: Severity,
    pub kind: IssueKind,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    // Raw input is preserved for determinism and traceability.
    pub raw_input: String,

    // Parsed entries (keeps duplicates/order for detection).
    pub entries: Vec<(String, String)>,

    // Canonical map (sorted keys).
    pub canonical: BTreeMap<String, String>,

    pub step: u32,
}

impl State {
    #[must_use]
    pub fn new(raw_input: String) -> Self {
        Self {
            raw_input,
            entries: Vec::new(),
            canonical: BTreeMap::new(),
            step: 0,
        }
    }
}
