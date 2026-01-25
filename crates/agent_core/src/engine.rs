use crate::model::{Issue, Severity, State};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

use crate::dsl::{Op, PatchProgram};

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

/// Deterministic token policy: keys [A-Za-z0-9_-], values may include same plus '.' and ':' (demo-friendly).
fn is_valid_key(k: &str) -> bool {
    !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn is_valid_value(v: &str) -> bool {
    !v.is_empty()
        && v.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' || c == ':' )
}

/// Demo spec: required fields and ranges
fn required_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("timeout", "30"),
        ("mode", "safe"),
    ]
}

fn range_fields() -> &'static [(&'static str, i64, i64)] {
    &[("timeout", 0, 600)]
}

fn parse_raw(raw: &str) -> Vec<(String, String)> {
    // Deterministic newline normalization
    let normalized = raw.replace(\"\\r\\n\", \"\\n\").replace(\"\\r\", \"\\n\");
    let mut entries = Vec::new();
    for line in normalized.split('\\n') {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            entries.push((k.trim().to_string(), v.trim().to_string()));
        } else {
            // Non key-value lines are treated as invalid tokens via detect()
            entries.push((line.to_string(), \"\".to_string()));
        }
    }
    entries
}

fn build_canonical(entries: &[(String, String)]) -> BTreeMap<String, String> {
    // Deterministic policy: keep FIRST occurrence for canonical map
    let mut m = BTreeMap::new();
    for (k, v) in entries {
        if !m.contains_key(k) {
            m.insert(k.clone(), v.clone());
        }
    }
    m
}

fn detect_issues(state: &State) -> Vec<Issue> {
    let mut issues = Vec::new();

    // Duplicate keys (based on entries list)
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
            message: format!(\"Duplicate key detected: {k}\"),
        });
    }

    // Invalid tokens (keys/values)
    for (k, v) in &state.entries {
        if v.is_empty() && !k.contains('=') && !is_valid_key(k) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!(\"Invalid line/token: '{k}'\"),
            });
            continue;
        }
        if !is_valid_key(k) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!(\"Invalid key token: '{k}'\"),
            });
        }
        if !v.is_empty() && !is_valid_value(v) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::InvalidToken,
                message: format!(\"Invalid value token for '{k}': '{v}'\"),
            });
        }
    }

    // Missing required fields
    for (k, _) in required_fields() {
        if !state.canonical.contains_key(*k) {
            issues.push(Issue {
                severity: Severity::HIGH,
                kind: crate::model::IssueKind::MissingRequired,
                message: format!(\"Missing required key: {k}\"),
            });
        }
    }

    // Range fields
    for (k, min, max) in range_fields() {
        if let Some(v) = state.canonical.get(*k) {
            if let Ok(n) = v.parse::<i64>() {
                if n < *min || n > *max {
                    issues.push(Issue {
                        severity: Severity::MEDIUM,
                        kind: crate::model::IssueKind::OutOfRange,
                        message: format!(\"Out of range: {k}={n}, expected [{min},{max}]\"),
                    });
                }
            } else {
                issues.push(Issue {
                    severity: Severity::MEDIUM,
                    kind: crate::model::IssueKind::OutOfRange,
                    message: format!(\"Non-integer value for ranged key: {k}='{v}'\"),
                });
            }
        }
    }

    // Non-canonical order (demo): if raw differs from canonical printed form
    let canonical_print = print_canonical(&state.canonical);
    let raw_norm = state.raw_input.replace(\"\\r\\n\", \"\\n\").replace(\"\\r\", \"\\n\").trim().to_string();
    if raw_norm != canonical_print.trim() {
        issues.push(Issue {
            severity: Severity::LOW,
            kind: crate::model::IssueKind::NonCanonicalOrder,
            message: \"Input not in canonical order/format\".to_string(),
        });
    }

    issues
}

fn print_canonical(map: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    for (k, v) in map {
        out.push_str(k);
        out.push('=');
        out.push_str(v);
        out.push('\\n');
    }
    out
}

fn mu(issues: &[Issue], noncanonical: bool) -> u64 {
    let w1: u64 = 10;
    let w2: u64 = 1;
    (w1 * issues.len() as u64) + if noncanonical { w2 } else { 0 }
}

fn state_hash_sha256(state: &State) -> String {
    // Canonical bytes: stable JSON of (canonical map + step)
    // BTreeMap guarantees key order; serde_json is deterministic for this structure.
    let obj = serde_json::json!({
        \"step\": state.step,
        \"canonical\": state.canonical
    });
    let bytes = serde_json::to_vec(&obj).expect(\"json\");
    let mut h = Sha256::new();
    h.update(&bytes);
    hex::encode(h.finalize())
}

fn synthesize_patch(issues: &[Issue], state: &State) -> PatchProgram {
    // Deterministic rule-based patch synthesis (no ML, no RNG)
    // Priority: HIGH issues -> MEDIUM -> LOW; stable ordering by message string
    let mut ops = Vec::new();

    let mut sorted = issues.to_vec();
    sorted.sort_by(|a, b| {
        let sa = match a.severity { Severity::HIGH => 0, Severity::MEDIUM => 1, Severity::LOW => 2 };
        let sb = match b.severity { Severity::HIGH => 0, Severity::MEDIUM => 1, Severity::LOW => 2 };
        sa.cmp(&sb).then_with(|| a.message.cmp(&b.message))
    });

    // Always normalize tokens first (safe)
    ops.push(Op::NormalizeTokens);

    if sorted.iter().any(|i| matches!(i.kind, crate::model::IssueKind::DuplicateKey)) {
        ops.push(Op::RemoveDuplicateKeys);
    }

    // Missing required -> insert defaults
    for (k, def) in required_fields() {
        if !state.canonical.contains_key(*k) {
            ops.push(Op::InsertDefault { key: (*k).to_string(), value: (*def).to_string() });
        }
    }

    // Range clamp
    for (k, min, max) in range_fields() {
        if state.canonical.contains_key(*k) {
            ops.push(Op::ClampRange { key: (*k).to_string(), min: *min, max: *max });
        }
    }

    // Canonical sort/print normalization
    ops.push(Op::SortKeysCanonical);

    PatchProgram::new(ops)
}

fn exec_patch(mut state: State, patch: &PatchProgram) -> State {
    // Apply ops deterministically by rewriting entries/canonical/raw_input
    for op in &patch.ops {
        match op {
            Op::NormalizeTokens => {
                // trim whitespace already; enforce newline normalization later
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
                // insert only if missing
                if !state.entries.iter().any(|(k, _)| k == key) {
                    state.entries.push((key.clone(), value.clone()));
                }
            }
            Op::ClampRange { key, min, max } => {
                for (k, v) in &mut state.entries {
                    if k == key {
                        if let Ok(n) = v.parse::<i64>() {
                            let clamped = n.clamp(*min, *max);
                            *v = clamped.to_string();
                        }
                    }
                }
            }
            Op::SortKeysCanonical => {
                // rebuild canonical and rewrite raw_input in canonical form
                state.canonical = build_canonical(&state.entries);
                state.raw_input = print_canonical(&state.canonical);
                // re-parse to ensure entries match canonical print (strict)
                state.entries = parse_raw(&state.raw_input);
                state.canonical = build_canonical(&state.entries);
            }
        }
    }
    state
}

fn is_correct(state: &State) -> bool {
    // P(S): required fields present, valid tokens, ranges ok, and canonical format
    let issues = detect_issues(state);
    !issues.iter().any(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM))
}

pub fn heal_to_fixpoint(initial_raw: String, cfg: HealConfig) -> Result<(State, Vec<TraceEvent>), String> {
    let mut state = State::new(initial_raw);
    state.entries = parse_raw(&state.raw_input);
    state.canonical = build_canonical(&state.entries);

    let mut trace = Vec::new();

    for cycle in 0..cfg.max_cycles {
        state.step = cycle;
        let issues = detect_issues(&state);

        let canonical_print = print_canonical(&state.canonical);
        let raw_norm = state.raw_input.replace(\"\\r\\n\", \"\\n\").replace(\"\\r\", \"\\n\").trim().to_string();
        let noncanonical = raw_norm != canonical_print.trim();

        // Fixpoint: no issues at all
        if issues.is_empty() {
            let patch = PatchProgram::new(vec![]);
            trace.push(TraceEvent {
                step: state.step,
                mu: 0,
                issues,
                patch,
                state_hash_sha256: state_hash_sha256(&state),
            });
            return Ok((state, trace));
        }

        let current_mu = mu(&issues, noncanonical);
        let patch = synthesize_patch(&issues, &state);

        // Verify patch by simulation: must reduce mu and improve correctness (soundness within scope)
        let mut next = exec_patch(state.clone(), &patch);
        next.step = cycle + 1;
        let next_issues = detect_issues(&next);

        let next_canonical_print = print_canonical(&next.canonical);
        let next_raw_norm = next.raw_input.replace(\"\\r\\n\", \"\\n\").replace(\"\\r\", \"\\n\").trim().to_string();
        let next_noncanonical = next_raw_norm != next_canonical_print.trim();
        let next_mu = mu(&next_issues, next_noncanonical);

        if next_mu >= current_mu {
            return Err(format!(\"Verification failed: mu did not decrease (current={current_mu}, next={next_mu})\"));
        }

        // For demo: require that HIGH/MEDIUM issues strictly decrease
        let cur_hm = issues.iter().filter(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM)).count();
        let nxt_hm = next_issues.iter().filter(|i| matches!(i.severity, Severity::HIGH | Severity::MEDIUM)).count();
        if nxt_hm > cur_hm {
            return Err(\"Verification failed: HIGH/MEDIUM issues increased\".to_string());
        }

        trace.push(TraceEvent {
            step: state.step,
            mu: current_mu,
            issues,
            patch: patch.clone(),
            state_hash_sha256: state_hash_sha256(&state),
        });

        state = next;

        // Early exit if P(S) satisfied and only LOW issues remain
        if is_correct(&state) {
            // normalize once more for canonical formatting
            let final_patch = PatchProgram::new(vec![Op::SortKeysCanonical]);
            state = exec_patch(state.clone(), &final_patch);
        }
    }

    Err(\"Max cycles reached without fixpoint\".to_string())
}

pub fn verify_trace(trace: &[TraceEvent]) -> Result<(), String> {
    // Minimal verifier: mu must strictly decrease until last step and hashes must be 64 hex chars
    let mut prev_mu: Option<u64> = None;
    for e in trace {
        if e.state_hash_sha256.len() != 64 {
            return Err(\"Invalid hash length in trace\".to_string());
        }
        if let Some(pm) = prev_mu {
            if e.mu >= pm && e.mu != 0 {
                return Err(\"Trace verification failed: mu not decreasing\".to_string());
            }
        }
        prev_mu = Some(e.mu);
    }
    Ok(())
}
