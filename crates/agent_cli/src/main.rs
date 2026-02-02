use std::path::PathBuf;

use agent_core::io::TraceFile;
use agent_core::io::{read_text_file, write_json_file, write_text_file};
use agent_core::{heal_to_fixpoint, verify_trace, HealConfig};

fn has_help_flag(args: &[String]) -> bool {
    args.iter().any(|a| a == "--help" || a == "-h")
}

fn print_help() {
    eprintln!("Reflexive-DSHA CLI");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  agent_cli --help");
    eprintln!("  agent_cli run-demo [--in <file>] [--out <dir>] [--help]");
    eprintln!("  agent_cli verify-trace [--trace <file>] [--help]");
    eprintln!();
    eprintln!("Defaults:");
    eprintln!("  run-demo --in  examples/demo_input.json");
    eprintln!("  run-demo --out target/demo_out");
    eprintln!("  verify-trace --trace target/demo_out/trace.json");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  agent_cli run-demo");
    eprintln!("  agent_cli run-demo --in examples/demo_input.json --out target/demo_out");
    eprintln!("  agent_cli verify-trace --trace target/demo_out/trace.json");
}

fn usage_ok() -> ! {
    print_help();
    std::process::exit(0);
}

fn usage_err(msg: &str) -> ! {
    eprintln!("error: {msg}");
    eprintln!();
    print_help();
    std::process::exit(2);
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}

/// Best-effort extract last "state_hash_sha256" from a TraceFile JSON representation.
fn extract_last_state_hash_sha256(tf: &TraceFile) -> Option<String> {
    let v = serde_json::to_value(tf).ok()?;
    let arr = v.get("trace")?.as_array()?;
    let last = arr.last()?;
    last.get("state_hash_sha256")?.as_str().map(|s| s.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || has_help_flag(&args) {
        usage_ok();
    }

    match args[1].as_str() {
        "run-demo" => {
            if has_help_flag(&args) {
                eprintln!("run-demo: ejecuta el healing determinista y escribe final+trace.");
                eprintln!("Usage: agent_cli run-demo [--in <file>] [--out <dir>]");
                eprintln!("Defaults: --in examples/demo_input.json  --out target/demo_out");
                std::process::exit(0);
            }

            let in_path = arg_value(&args, "--in")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("examples/demo_input.json"));

            let out_dir = arg_value(&args, "--out")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("target/demo_out"));

            std::fs::create_dir_all(&out_dir)
                .unwrap_or_else(|e| usage_err(&format!("no pude crear --out dir: {e}")));

            let raw = read_text_file(&in_path)
                .unwrap_or_else(|e| usage_err(&format!("no pude leer --in file: {e}")));

            let (final_state, trace) =
                heal_to_fixpoint(raw, &HealConfig::default()).unwrap_or_else(|e| panic!("{e}"));

            let tf = TraceFile {
                trace: trace.clone(),
                final_state: final_state.clone(),
            };

            write_json_file(&out_dir.join("trace.json"), &tf).unwrap();
            write_text_file(
                &out_dir.join("final.json"),
                &serde_json::to_string_pretty(&final_state).unwrap(),
            )
            .unwrap();

            // Canonical config
            write_text_file(&out_dir.join("final.config"), &final_state.raw_input).unwrap();

            println!("OK: wrote {}", out_dir.display());

            if let Some(h) = extract_last_state_hash_sha256(&tf) {
                println!("OK: final state_hash_sha256 = {h}");
            } else {
                println!("OK: (nota) trace.json no expone state_hash_sha256 en el nivel esperado");
            }
        }

        "verify-trace" => {
            if has_help_flag(&args) {
                eprintln!("verify-trace: valida la consistencia del trace.");
                eprintln!("Usage: agent_cli verify-trace [--trace <file>]");
                eprintln!("Default: --trace target/demo_out/trace.json");
                std::process::exit(0);
            }

            let trace_path = arg_value(&args, "--trace")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("target/demo_out/trace.json"));

            let raw = read_text_file(&trace_path)
                .unwrap_or_else(|e| usage_err(&format!("no pude leer --trace file: {e}")));

            let tf: TraceFile =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("json decode: {e}"));

            verify_trace(&tf.trace).unwrap_or_else(|e| panic!("{e}"));
            println!("OK: trace verified");

            if let Some(h) = extract_last_state_hash_sha256(&tf) {
                println!("OK: final state_hash_sha256 = {h}");
            }
        }

        _ => usage_err("comando desconocido"),
    }
}
