# reflexive-dsha

Deterministic self-healing agent core (Rust-first), offline, reproducible, trace-hashed, with formal-verification scaffolding.

## Quickstart (PowerShell)
\\\powershell
cargo build
cargo run -p agent_cli -- run-demo --in demo/inputs/config.txt --out out
cargo run -p agent_cli -- verify-trace --trace out/trace.json
Get-Content out/final.config
\\\

## Determinism Guarantees (Demo Scope)
- No RNG
- No floating point
- Canonical serialization (BTreeMap ordered keys)
- SHA-256 hash per step in trace

## Next: Formal proofs
- proofs/lean: DSL semantics + termination/soundness theorems
- proofs/tla: composability + deadlock-freedom model
