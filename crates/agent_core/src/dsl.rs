use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Op {
    RemoveDuplicateKeys,
    NormalizeTokens,
    RemoveInvalidTokens,
    InsertDefault { key: String, value: String },
    ClampRange { key: String, min: i64, max: i64 },
    SortKeysCanonical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PatchProgram {
    pub ops: Vec<Op>,
}

impl PatchProgram {
    pub fn new(ops: Vec<Op>) -> Self {
        Self { ops }
    }
}
