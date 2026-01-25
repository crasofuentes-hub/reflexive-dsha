use crate::dsl::{Op, PatchProgram};
use crate::model::{Issue, Severity, State};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct HealConfig {
    pub max_cycles: u32,
}

impl Default for HealConfig {
    fn default() -> Self {
        Self { max_cycles: 16 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub step: u32,
    pub mu: u64,
    pub issues: Vec<Issue>,
    pub patch: PatchProgram,
    pub state_hash_sha256: String,
}

// ---------------------------
// Deterministic policy helpers
// ---------------------------

fn is_valid_key(k: &str) -> bool {
    !k.is_empty()
        && k.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn is_valid_value(v: &str) -> bool {
    !v.is_empty()
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ':')
}

fn is_allowed_mode(v: &str) -> bool {
    matches!(v, "safe" | "strict" | "hardened")
}

fn required_fields() -> &'static [(&'static str, &'static str)] {
    &[("timeout", "30"), ("mode", "safe")]
}

fn range_fields() -> &'static [(&'static str, i64, i64)] {
    &[("timeout", 0, 600)]
}

// ---------------------------
// Parsing / canonicalization
// ---------------------------

fn normalize_newlines(raw: &str) -> String {
    raw.replace("\r\n", "\n").replace('\r', "\n")
}

fn parse_raw(raw: &str) -> Vec<(String, String)> {
    let normalized = normalize_newlines(raw);
    let mut entries = Vec::new();

    for line in normalized.split('\n') {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            entries.push((k.trim().to_string(), v.trim().to_string()));
        } else {
            // Non key/value line -> represented as invalid token
            entries.push((line.to_string(), String::new()));
        }
    }

    entries
}

fn build_canonical(entries: &[(String, String)]) -> BTreeMap<String, String> {
    // Deterministic: keep FIRST occurrence only
    let mut map = BTreeMap::new();
    for (k, v) in entries {
        if !map.contains_key(k) {
            map.insert(k.clone(), v.clone());
        }
    }
    map
}

fn print_canonical(map: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    for (k, v) in map {
        out.push_str(k);
        out.push('=');
        out.push_str(v);
        out.push('\n');
    }
    out
}

fn state_hash_sha256(state: &State) -> String {
    // Canonical bytes: stable JSON of (step + canonical)
    // BTreeMap ensures key order; serde_json stable for this structure.
    let obj = serde_json::json!({
        "step": state.step,
        "canonical": state.canonical
    });
    let bytes = serde_json::to_vec(&obj).expect("json encode");
    let mut h = Sha256::new();
    h.update(&bytes);
    hex::encode(h.finalize())
}

// ---------------------------
// Detection (P(S) / issues)
// ---------------------------

fn detect_issues(state: &State) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Duplicate keys in entries
    let mut seen = BTreeSet::new();
    let mut dups = BTreeSet::new();
    for (k, _) in &state.entries {
        if seen.contains(k) {
            dups.insert(k.clone());
        } else {
            seen.insert(k.clone());
        }
    }
    for k in dups {
        issues.push(Issue {
            severity: Severity::MEDIUM,
            kind: crate::model::IssueKind::DuplicateKey,
            message: format!("Duplicate key detected: {k}"),
        });
    }

    // Token validity (keys/values)
    for (k, v) in &state.entries {
        // For malformed lines, v == "" and k is the whole line, so key usually invalid
        if !is_valid_key(k) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!("Invalid key token: '{k}'"),
            });
            continue;
        }
        if !is_valid_value(v) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!("Invalid value token for '{k}': '{v}'"),
            });
        }
    }

    // Required fields
    for (k, _) in required_fields() {
        if !state.canonical.contains_key(*k) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::MissingRequired,
                message: format!("Missing required key: {k}"),
            });
        }
    }

    // Range fields
    for (k, min, max) in range_fields() {
        if let Some(v) = state.canonical.get(*k) {
            match v.parse::<i64>() {
                Ok(n) => {
                    if n < *min || n > *max {
                        issues.push(Issue {
                            severity: Severity::MEDIUM,
                            kind: crate::model::IssueKind::OutOfRange,
                            message: format!("Out of range: {k}={n}, expected [{min},{max}]"),
                        });
                    }
                }
                Err(_) => {
                    issues.push(Issue {
                        severity: Severity::MEDIUM,
                        kind: crate::model::IssueKind::OutOfRange,
                        message: format!("Non-integer value for ranged key: {k}='{v}'"),
                    });
                }
            }
        }
    }

    // Mode allowlist (semantic validity)
    if let Some(v) = state.canonical.get("mode") {
        if !is_allowed_mode(v) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!("Mode not allowed: mode='{v}' (allowed: safe|strict|hardened)"),
            });
        }
    }

    // Non-canonical formatting
    let canonical_print = print_canonical(&state.canonical).trim().to_string();
    let raw_norm = normalize_newlines(&state.raw_input).trim().to_string();
    if raw_norm != canonical_print {
        issues.push(Issue {
            severity: Severity::LOW,
            kind: crate::model::IssueKind::NonCanonicalOrder,
            message: "Input not in canonical order/format".to_string(),
        });
    }

    issues
}

fn mu(issues: &[Issue]) -> u64 {
    // Weighted measure (well-founded for termination within finite domain):
    // HIGH=100, MEDIUM=10, LOW=1
    issues
        .iter()
        .map(|i| match i.severity {
            Severity::HIGH => 100u64,
            Severity::MEDIUM => 10u64,
            Severity::LOW => 1u64,
        })
        .sum()
}

fn is_correct(state: &State) -> bool {
    let issues = detect_issues(state);
    !issues
        .iter()
        .any(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM))
}

// ---------------------------
// Patch synthesis + execution
// ---------------------------

fn synthesize_patch(issues: &[Issue], state: &State) -> PatchProgram {
    // Deterministic, rule-based (no RNG, no ML)
    let mut ops = Vec::new();

    // Always normalize first (no-op placeholder, but stable ordering)
    ops.push(Op::NormalizeTokens);

    // If any invalid token exists (including mode policy), sanitize tokens
    if issues
        .iter()
        .any(|i| matches!(i.kind, crate::model::IssueKind::InvalidToken))
    {
        ops.push(Op::RemoveInvalidTokens);
    }

    // Dedup
    if issues
        .iter()
        .any(|i| matches!(i.kind, crate::model::IssueKind::DuplicateKey))
    {
        ops.push(Op::RemoveDuplicateKeys);
    }

    // Insert defaults for missing required
    for (k, def) in required_fields() {
        if !state.canonical.contains_key(*k) {
            ops.push(Op::InsertDefault {
                key: (*k).to_string(),
                value: (*def).to_string(),
            });
        }
    }

    // Clamp ranges
    for (k, min, max) in range_fields() {
        if state.canonical.contains_key(*k) {
            ops.push(Op::ClampRange {
                key: (*k).to_string(),
                min: *min,
                max: *max,
            });
        }
    }

    // Canonical rewrite at the end
    ops.push(Op::SortKeysCanonical);

    PatchProgram::new(ops)
}

fn exec_patch(mut state: State, patch: &PatchProgram) -> State {
    for op in &patch.ops {
        match op {
            Op::NormalizeTokens => {
                // Already trimmed during parse; kept for stable sequencing.
            }

            Op::RemoveInvalidTokens => {
                // Deterministic sanitization:
                // - Drop invalid keys/values
                // - Enforce semantic allowlists (mode)
                let mut new_entries = Vec::new();
                for (k, v) in &state.entries {
                    let mut keep = is_valid_key(k) && is_valid_value(v);
                    if keep && k == "mode" && !is_allowed_mode(v) {
                        keep = false;
                    }
                    if keep {
                        new_entries.push((k.clone(), v.clone()));
                    }
                }
                state.entries = new_entries;
            }

            Op::RemoveDuplicateKeys => {
                let mut seen = BTreeSet::new();
                let mut new_entries = Vec::new();
                for (k, v) in &state.entries {
                    if !seen.contains(k) {
                        seen.insert(k.clone());
                        new_entries.push((k.clone(), v.clone()));
                    }
                }
                state.entries = new_entries;
            }

            Op::InsertDefault { key, value } => {
                if !state.entries.iter().any(|(k, _)| k == key) {
                    state.entries.push((key.clone(), value.clone()));
                }
            }

            Op::ClampRange { key, min, max } => {
                for (k, v) in &mut state.entries {
                    if k == key {
                        if let Ok(n) = v.parse::<i64>() {
                            *v = n.clamp(*min, *max).to_string();
                        }
                    }
                }
            }

            Op::SortKeysCanonical => {
                state.canonical = build_canonical(&state.entries);
                state.raw_input = print_canonical(&state.canonical);
                // Re-parse to enforce strict canonical consistency
                state.entries = parse_raw(&state.raw_input);
                state.canonical = build_canonical(&state.entries);
            }
        }
    }
    state
}

// ---------------------------
// Healing loop + verification
// ---------------------------

pub fn heal_to_fixpoint(
    initial_raw: String,
    cfg: HealConfig,
) -> Result<(State, Vec<TraceEvent>), String> {
    let mut state = State::new(initial_raw);
    state.entries = parse_raw(&state.raw_input);
    state.canonical = build_canonical(&state.entries);

    let mut trace: Vec<TraceEvent> = Vec::new();

    for cycle in 0..cfg.max_cycles {
        state.step = cycle;

        let issues = detect_issues(&state);
        let current_mu = mu(&issues);

        if issues.is_empty() {
            trace.push(TraceEvent {
                step: state.step,
                mu: 0,
                issues,
                patch: PatchProgram::new(vec![]),
                state_hash_sha256: state_hash_sha256(&state),
            });
            return Ok((state, trace));
        }

        let patch = synthesize_patch(&issues, &state);
        let mut next = exec_patch(state.clone(), &patch);
        next.step = cycle + 1;

        let next_issues = detect_issues(&next);
        let next_mu = mu(&next_issues);

        // Strict progress (termination measure)
        if next_mu >= current_mu {
            return Err(format!(
                "Verification failed: mu did not decrease (current={current_mu}, next={next_mu})"
            ));
        }

        // Extra guard: HIGH/MEDIUM must not increase
        let cur_hm = issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM))
            .count();
        let nxt_hm = next_issues
            .iter()
            .filter(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM))
            .count();
        if nxt_hm > cur_hm {
            return Err("Verification failed: HIGH/MEDIUM issues increased".to_string());
        }

        trace.push(TraceEvent {
            step: state.step,
            mu: current_mu,
            issues,
            patch: patch.clone(),
            state_hash_sha256: state_hash_sha256(&state),
        });

        state = next;

        // Optional normalization pass once correctness is reached (only LOW may remain)
        if is_correct(&state) {
            let norm = PatchProgram::new(vec![Op::SortKeysCanonical]);
            state = exec_patch(state.clone(), &norm);
        }
    }

    Err("Max cycles reached without fixpoint".to_string())
}

pub fn verify_trace(trace: &[TraceEvent]) -> Result<(), String> {
    // Strict trace checks:
    // - hash length
    // - mu strictly decreases until it reaches 0 (equal allowed only at mu=0)
    let mut prev_mu: Option<u64> = None;

    for e in trace {
        if e.state_hash_sha256.len() != 64 {
            return Err("Invalid hash length in trace".to_string());
        }

        if let Some(pm) = prev_mu {
            if e.mu > pm {
                return Err("Trace verification failed: mu increased".to_string());
            }
            if pm != 0 && e.mu == pm {
                return Err("Trace verification failed: mu did not decrease".to_string());
            }
        }

        prev_mu = Some(e.mu);
    }

    Ok(())
}
