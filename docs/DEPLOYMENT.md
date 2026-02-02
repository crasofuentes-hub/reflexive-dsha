# Deployment

## Local (Windows/Linux/macOS)
Prereqs:
- Rust toolchain
- (Optional) Lean toolchain for formal proofs

Commands:
- Build: cargo build
- Run demo: cargo run -p agent_cli -- run-demo
- Verify trace: cargo run -p agent_cli -- verify-trace

## CI
- rustfmt + clippy
- unit tests
- run demo + verify-trace smoke test
- build formal proofs (lake build)

