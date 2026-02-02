# Architecture

## Goals
- Determinism
- Trace verifiability (hashable trace)
- Fixpoint convergence (no infinite loops)
- Formal verification layer (Lean)

## High-level components
- agent_core (healing engine + trace + verification)
- agent_cli (demo runner + verify-trace)
- formal/ (Lean specifications & proofs)

## Data flow
1) Input config/text
2) heal_to_fixpoint(...) -> (final_state, trace)
3) Persist trace.json + final.json + final.config
4) verify-trace(trace) -> OK/ERR
5) Optional: reproducibility hash: state_hash_sha256

## Determinism contract
- Same input + same version => same final state + same trace hashes

## Threat model / Non-goals
- Not an LLM agent runtime
- No network dependence for core verification

