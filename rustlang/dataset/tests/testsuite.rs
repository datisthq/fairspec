use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine;
use fairspec_dataset::{describe_file, preview_file, validate_file, write_temp_file};
use fairspec_metadata::{
    Dataset, Descriptor, IntegrityType, Resource, infer_resource_name, inspect_json,
    load_dataset_descriptor, validate_data_schema, validate_table_schema,
};
use serde_json::{Map, Value};

const UNSPECIFIED_MESSAGE_ERRORS: [&str; 3] = ["data", "metadata", "cell/json"];

/// Operations that need the table and library layers, which no Rust crate has yet.
const SKIPPED: [&str; 10] = [
    "data-infer-schema",
    "data-validate",
    "dataset-infer",
    "dataset-query",
    "dataset-validate",
    "table-infer-dialect",
    "table-infer-schema",
    "table-load",
    "table-query",
    "table-validate",
];

type Operation = fn(&Value, &Value) -> Result<Value, String>;

#[test]
fn test_testsuite() {
    let mut failures = Vec::new();
    let mut passed = 0;

    for (operation, file) in list_files() {
        let id = format!(
            "{operation}/{}",
            file.file_stem().unwrap().to_string_lossy()
        );
        let groups: Value = serde_json::from_str(&fs::read_to_string(&file).unwrap()).unwrap();

        match inspect_json(&groups, get_suite_path("test-schema.json"), None) {
            Ok(issues) if issues.is_empty() => {}
            Ok(issues) => failures.push(format!("{id}: not a valid test file: {issues:?}")),
            Err(error) => failures.push(format!("{id}: cannot inspect: {error}")),
        }

        if SKIPPED.contains(&operation.as_str()) {
            continue;
        }
        let Some(run) = get_operation(&operation) else {
            failures.push(format!("{id}: unknown operation {operation}"));
            continue;
        };

        for group in groups.as_array().unwrap() {
            for test in group["tests"].as_array().unwrap() {
                let case_id = format!("{id} › {} › {}", group["description"], test["description"]);
                match run_case(run, group, test) {
                    Ok(()) => passed += 1,
                    Err(message) => failures.push(format!("{case_id}: {message}")),
                }
            }
        }
    }

    println!("testsuite: {passed} cases passed");
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

fn get_operation(operation: &str) -> Option<Operation> {
    match operation {
        "data-validate-schema" => Some(run_data_validate_schema),
        "dataset-list" => Some(run_dataset_list),
        "file-describe" => Some(run_file_describe),
        "file-preview" => Some(run_file_preview),
        "file-validate" => Some(run_file_validate),
        "table-validate-schema" => Some(run_table_validate_schema),
        _ => None,
    }
}

fn run_data_validate_schema(_group: &Value, test: &Value) -> Result<Value, String> {
    let Value::Object(data) = &test["data"] else {
        return Err("data-validate-schema data must be a schema object".to_string());
    };
    let mut report = validate_data_schema(data.clone(), None).map_err(|error| error.to_string())?;
    report.data_schema = None;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn run_table_validate_schema(_group: &Value, test: &Value) -> Result<Value, String> {
    let Value::Object(data) = &test["data"] else {
        return Err("table-validate-schema data must be a schema object".to_string());
    };
    let mut report =
        validate_table_schema(data.clone(), None).map_err(|error| error.to_string())?;
    report.table_schema = None;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn run_dataset_list(group: &Value, test: &Value) -> Result<Value, String> {
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = write_dataset(folder.path(), group, test)?;
    let dataset: Dataset = load_dataset_descriptor(&path).map_err(|error| error.to_string())?;
    let names = dataset
        .resources
        .unwrap_or_default()
        .iter()
        .enumerate()
        .map(|(index, resource)| {
            let name = resource
                .name
                .clone()
                .unwrap_or_else(|| infer_resource_name(resource, Some(index + 1)));
            Value::String(name)
        })
        .collect();
    Ok(Value::Array(names))
}

fn run_file_describe(group: &Value, test: &Value) -> Result<Value, String> {
    let hash_type = group["options"]
        .get("hashType")
        .cloned()
        .unwrap_or(Value::String("sha256".to_string()));
    let hash_type: IntegrityType =
        serde_json::from_value(hash_type).map_err(|error| error.to_string())?;
    let description = describe_file(&write_data(group, &test["data"])?, Some(hash_type))
        .map_err(|error| error.to_string())?;
    serde_json::to_value(description).map_err(|error| error.to_string())
}

fn run_file_preview(group: &Value, test: &Value) -> Result<Value, String> {
    let get_option = |name: &str| -> Result<Option<usize>, String> {
        match group["options"].get(name) {
            None => Ok(None),
            Some(value) => value
                .as_u64()
                .map(|value| Some(value as usize))
                .ok_or_else(|| format!("{name} must be a non-negative integer")),
        }
    };
    let preview = preview_file(
        &write_data(group, &test["data"])?,
        get_option("maxBytes")?,
        get_option("maxLines")?,
    )
    .map_err(|error| error.to_string())?;
    serde_json::to_value(preview).map_err(|error| error.to_string())
}

fn run_file_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let report =
        validate_file(&create_resource(group, test)?, None).map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn create_resource(group: &Value, test: &Value) -> Result<Resource, String> {
    let data = if group["inline"] == Value::Bool(true) {
        test["data"].clone()
    } else {
        Value::String(write_data(group, &test["data"])?)
    };
    let mut descriptor = Map::new();
    descriptor.insert("data".to_string(), data);
    for key in [
        "fileDialect",
        "tableSchema",
        "dataSchema",
        "integrity",
        "textual",
    ] {
        if let Some(value) = group.get(key) {
            descriptor.insert(key.to_string(), value.clone());
        }
    }
    serde_json::from_value(Value::Object(descriptor)).map_err(|error| error.to_string())
}

fn write_data(group: &Value, data: &Value) -> Result<String, String> {
    let format = group["format"].as_str().unwrap_or("csv");
    write_temp_file(encode_data(data)?, false, None, Some(format))
        .map_err(|error| error.to_string())
}

fn encode_data(data: &Value) -> Result<Vec<u8>, String> {
    match data {
        Value::String(text) => Ok(text.as_bytes().to_vec()),
        Value::Object(object) => match object.get("base64") {
            Some(Value::String(encoded)) => base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|error| error.to_string()),
            _ => Err("data must be a string or base64 object".to_string()),
        },
        _ => Err("data must be a string or base64 object".to_string()),
    }
}

fn run_case(run: Operation, group: &Value, test: &Value) -> Result<(), String> {
    if test["throws"] == Value::Bool(true) {
        return match run(group, test) {
            Ok(result) => Err(format!("expected to throw, got {result}")),
            Err(_) => Ok(()),
        };
    }

    let result = canonicalize(run(group, test)?);

    if let Some(expected) = test.get("expected")
        && &result != expected
    {
        return Err(format!("expected {expected}, got {result}"));
    }

    if let Some(Value::Bool(valid)) = test.get("valid") {
        if result["valid"] != Value::Bool(*valid) {
            return Err(format!("expected valid={valid}, got {result}"));
        }
        if test.get("errors").is_some() || *valid {
            let errors = test.get("errors").cloned().unwrap_or(Value::Array(vec![]));
            let mut expected = Map::new();
            expected.insert("valid".to_string(), Value::Bool(*valid));
            expected.insert("errors".to_string(), errors);
            if result != Value::Object(expected.clone()) {
                return Err(format!(
                    "expected {}, got {result}",
                    Value::Object(expected)
                ));
            }
        }
    }

    Ok(())
}

fn canonicalize(mut value: Value) -> Value {
    if let Some(Value::Array(errors)) = value.get_mut("errors") {
        for error in errors.iter_mut().filter_map(Value::as_object_mut) {
            let unspecified = error
                .get("type")
                .and_then(Value::as_str)
                .is_some_and(|r#type| UNSPECIFIED_MESSAGE_ERRORS.contains(&r#type));
            if unspecified {
                error.remove("message");
            }
        }
    }
    value
}

fn write_dataset(folder: &Path, group: &Value, test: &Value) -> Result<String, String> {
    let Value::Object(files) = &test["data"] else {
        return Err("dataset data must map file names to contents".to_string());
    };
    for (name, content) in files {
        let Value::String(content) = content else {
            return Err(format!("unsupported data for {name}"));
        };
        fs::write(folder.join(name), content).map_err(|error| error.to_string())?;
    }

    let path = folder.join("dataset.json");
    let dataset: Descriptor =
        serde_json::from_value(group["dataset"].clone()).map_err(|error| error.to_string())?;
    fs::write(&path, serde_json::to_string(&dataset).unwrap())
        .map_err(|error| error.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

fn list_files() -> Vec<(String, PathBuf)> {
    let mut operations: Vec<_> = fs::read_dir(get_suite_path("tests"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    operations.sort();

    let mut files = Vec::new();
    for operation in operations {
        let mut entries: Vec<_> = fs::read_dir(&operation)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        let name = operation
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        files.extend(entries.into_iter().map(|file| (name.clone(), file)));
    }
    files
}

fn get_suite_path(name: &str) -> String {
    format!("{}/../../testsuite/{name}", env!("CARGO_MANIFEST_DIR"))
}
