use std::{env, fs, process::ExitCode};
use da_vinci_crd::validate_crd_json;

fn main() -> ExitCode {
    let path = env::args().nth(1).expect("usage: validate-crd <file>");
    let json = fs::read_to_string(path).unwrap();
    let report = validate_crd_json(&json).unwrap();
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap());
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
