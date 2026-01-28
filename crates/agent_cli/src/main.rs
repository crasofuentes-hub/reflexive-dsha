use std::path::PathBuf;

use agent_core::io::TraceFile;
use agent_core::io::{read_text_file, write_json_file, write_text_file};
use agent_core::{heal_to_fixpoint, verify_trace, HealConfig};

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  agent_cli run-demo --in <file> --out <dir>");
    eprintln!("  agent_cli verify-trace --trace <file>");
    std::process::exit(2);
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == key).map(|w| w[1].clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
    }
    match args[1].as_str() {
        "run-demo" => {
            let in_path = arg_value(&args, "--in")
                .map(PathBuf::from)
                .unwrap_or_else(|| usage());
            let out_dir = arg_value(&args, "--out")
                .map(PathBuf::from)
                .unwrap_or_else(|| usage());

            let raw = read_text_file(&in_path).unwrap_or_else(|e| panic!("{e}"));
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

            // Also write canonical config
            write_text_file(&out_dir.join("final.config"), &final_state.raw_input).unwrap();

            println!("OK: wrote {}", out_dir.display());
        }
        "verify-trace" => {
            let trace_path = arg_value(&args, "--trace")
                .map(PathBuf::from)
                .unwrap_or_else(|| usage());
            let raw = read_text_file(&trace_path).unwrap_or_else(|e| panic!("{e}"));
            let tf: TraceFile =
                serde_json::from_str(&raw).unwrap_or_else(|e| panic!("json decode: {e}"));
            verify_trace(&tf.trace).unwrap_or_else(|e| panic!("{e}"));
            println!("OK: trace verified");
        }
        _ => usage(),
    }
}
