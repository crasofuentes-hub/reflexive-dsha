# API (CLI + File formats)

## CLI
- agent_cli --help
- agent_cli run-demo [--in <file>] [--out <dir>]
- agent_cli verify-trace [--trace <file>]

## Outputs
- trace.json: TraceFile { trace: [...], final_state: ... }
- final.json: pretty JSON of final_state
- final.config: canonical raw_input

## Trace verification
- verify-trace checks internal consistency rules
- expected deterministic hash field: state_hash_sha256 (if present)

