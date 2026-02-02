# Trace Spec v1 (Reflexive-DSHA)

## Purpose
A deterministic, verifiable trace of the healing loop. It must be stable across OS and reproducible.

## Version
trace_format_version: "1"

## Fields (TraceEvent)
- step: u32
- mu: u64
- issues: Vec<Issue>
- patch: PatchProgram
- state_hash_sha256: hex sha256 of stable_json({ step, canonical })

## Invariants
1) state_hash_sha256 length == 64 hex chars
2) mu strictly decreases until it reaches 0
3) if previous mu != 0, next mu must be < previous mu
4) final mu == 0 for successful heal_to_fixpoint
