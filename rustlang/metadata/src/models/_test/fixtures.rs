use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::round_trip;
use crate::models::dataset::Dataset;
use crate::models::error::FairspecError;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::table_schema::TableSchema;

#[test]
fn test_standard_examples_round_trip() {
    let examples = repo_root().join("standard/examples");
    let dataset = read_json(&examples.join("dataset.json"));
    assert_json_eq(&round_trip::<Dataset>(&dataset), &dataset, "dataset.json");
    let table_schema = read_json(&examples.join("table-schema.json"));
    assert_json_eq(
        &round_trip::<TableSchema>(&table_schema),
        &table_schema,
        "table-schema.json",
    );
}

#[test]
fn test_testsuite_descriptors_round_trip() {
    let mut counts = [0; 4];
    for file in testsuite_files() {
        let name = file.display().to_string();
        let content = read_json(&file);
        let mut found = Vec::new();
        collect_descriptors(&content, &mut found);
        for (key, value) in found {
            match key {
                "tableSchema" => {
                    assert_json_subset(&round_trip::<TableSchema>(value), value, &name);
                    counts[0] += 1;
                }
                "dataset" => {
                    assert_json_subset(&round_trip::<Dataset>(value), value, &name);
                    counts[1] += 1;
                }
                "fileDialect" => {
                    assert_json_subset(&round_trip::<FileDialect>(value), value, &name);
                    counts[2] += 1;
                }
                _ => {
                    // Expected errors in the suite may omit `message`, which the model requires
                    match serde_json::from_value::<FairspecError>(value.clone()) {
                        Ok(error) => {
                            let output = serde_json::to_value(&error).unwrap();
                            assert_json_eq(&output, value, &name);
                            counts[3] += 1;
                        }
                        Err(_) => assert!(value.get("message").is_none(), "{name}: {value}"),
                    }
                }
            }
        }
    }
    assert!(counts.iter().all(|count| *count > 0), "{counts:?}");
}

fn collect_descriptors<'a>(value: &'a Value, found: &mut Vec<(&'static str, &'a Value)>) {
    match value {
        Value::Object(object) => {
            for (key, item) in object {
                match (key.as_str(), item) {
                    ("tableSchema", Value::Object(_)) => found.push(("tableSchema", item)),
                    ("dataset", Value::Object(_)) => found.push(("dataset", item)),
                    ("fileDialect", Value::Object(_)) => found.push(("fileDialect", item)),
                    ("errors", Value::Array(errors)) => found.extend(
                        errors
                            .iter()
                            .filter(|e| e.is_object())
                            .map(|e| ("error", e)),
                    ),
                    _ => {}
                }
                collect_descriptors(item, found);
            }
        }
        Value::Array(items) => items
            .iter()
            .for_each(|item| collect_descriptors(item, found)),
        _ => {}
    }
}

fn assert_json_eq(output: &Value, input: &Value, context: &str) {
    assert!(
        json_matches(output, input, false),
        "{context}\n  input:  {input}\n  output: {output}"
    );
}

// Unknown fields are dropped on parse, so the output may lack keys the input has
fn assert_json_subset(output: &Value, input: &Value, context: &str) {
    assert!(
        json_matches(output, input, true),
        "{context}\n  input:  {input}\n  output: {output}"
    );
}

// Numbers compare by value: a float field serializes `0` as `0.0`, as Python does
fn json_matches(output: &Value, input: &Value, subset: bool) -> bool {
    match (output, input) {
        (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| json_matches(a, b, subset))
        }
        (Value::Object(a), Value::Object(b)) => {
            (subset || a.len() == b.len())
                && a.iter()
                    .all(|(key, a)| b.get(key).is_some_and(|b| json_matches(a, b, subset)))
        }
        (a, b) => a == b,
    }
}

fn testsuite_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in fs::read_dir(repo_root().join("testsuite/tests")).unwrap() {
        for file in fs::read_dir(dir.unwrap().path()).unwrap() {
            let path = file.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "json") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
