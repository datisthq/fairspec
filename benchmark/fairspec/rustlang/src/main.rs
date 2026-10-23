use std::env;
use std::time::Instant;

use fairspec_library::fairspec_metadata::Resource;
use fairspec_library::validate_table;
use serde_json::json;

fn main() {
    let started = Instant::now();
    let args: Vec<String> = env::args().collect();
    let resource: Resource =
        serde_json::from_value(json!({ "data": args[1], "tableSchema": args[2] })).unwrap();
    let report = validate_table(&resource, &Default::default()).unwrap();
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    println!(
        "{}",
        json!({ "errorCount": report.errors.len(), "elapsedMs": elapsed_ms })
    );
}
