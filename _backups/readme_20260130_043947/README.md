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
## Estado de las pruebas formales (Lean)

Este proyecto persigue garantías fuertes: determinismo, trazas verificables y pruebas formales tempranas.

### Teoremas demostrados
- **Idempotencia en fixpoint**: si mu(detect_issues(s)) = 0, entonces heal_to_fixpoint(n, s) = (s, []) para todo 
.
  - Archivo: ormal/ReflexiveDSHA/Proofs.lean (heal_to_fixpoint_idempotent_if_fixpoint)

- **Soundness (hash vs semántica)**: el hash reportado por el pipeline coincide con el hash de la traza generada por la semántica operacional modelada.
  - Archivo: ormal/ReflexiveDSHA/Proofs.lean (hash_matches_semantics)

### En progreso
- **Terminación por medida bien fundada**: bajo el invariante mu estrictamente decreciente en cada paso fuera de fixpoint, se alcanza un fixpoint en ≤ mu(s0) pasos (con fuel suficiente).
  - Archivo: ormal/ReflexiveDSHA/Proofs.lean (eaches_fixpoint_within_mu)