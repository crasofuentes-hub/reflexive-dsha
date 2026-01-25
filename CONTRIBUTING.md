# Contributing

## Requirements
- PowerShell 7+
- Rust stable (pinned by rust-toolchain.toml)

## Quality Gates
- \cargo fmt --check\
- \cargo clippy -- -D warnings\
- \cargo test\
- Reproducibility check (CI): demo output hashes must match between runs
