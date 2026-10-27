use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine;
use fairspec_library::fairspec_dataset::{
    InferFileDialectOptions, SaveDatasetOptions, describe_file, preview_file, validate_file,
    write_temp_file,
};
use fairspec_library::fairspec_metadata::{
    Dataset, Descriptor, IntegrityType, Resource, denormalize_dataset, infer_resource_name,
    inspect_json, validate_data_schema, validate_table_schema,
};
use fairspec_library::fairspec_table::polars::prelude::*;
use fairspec_library::fairspec_table::{
    InferTableSchemaOptions, LoadTableOptions, Table, query_table,
};
use fairspec_library::{
    ValidateTableOptions, infer_data_schema, infer_dataset, infer_file_dialect, infer_table_schema,
    load_dataset, load_table, query_dataset, save_dataset, validate_data, validate_dataset,
    validate_table,
};
use serde_json::{Map, Value};

const UNSPECIFIED_MESSAGE_ERRORS: [&str; 3] = ["data", "metadata", "cell/json"];

type Operation = fn(&Value, &Value) -> Result<Value, String>;

#[test]
fn test_conformance() {
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

    println!("conformance: {passed} cases passed");
    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

fn get_operation(operation: &str) -> Option<Operation> {
    match operation {
        "data-infer-schema" => Some(run_data_infer_schema),
        "data-validate" => Some(run_data_validate),
        "data-validate-schema" => Some(run_data_validate_schema),
        "dataset-copy" => Some(run_dataset_copy),
        "dataset-infer" => Some(run_dataset_infer),
        "dataset-list" => Some(run_dataset_list),
        "dataset-query" => Some(run_dataset_query),
        "dataset-validate" => Some(run_dataset_validate),
        "file-describe" => Some(run_file_describe),
        "file-preview" => Some(run_file_preview),
        "file-validate" => Some(run_file_validate),
        "table-infer-dialect" => Some(run_table_infer_dialect),
        "table-infer-schema" => Some(run_table_infer_schema),
        "table-load" => Some(run_table_load),
        "table-query" => Some(run_table_query),
        "table-validate" => Some(run_table_validate),
        "table-validate-schema" => Some(run_table_validate_schema),
        _ => None,
    }
}

fn run_data_infer_schema(group: &Value, test: &Value) -> Result<Value, String> {
    let resource = create_resource(group, test)?;
    let data_schema = infer_data_schema(&resource).map_err(|error| error.to_string())?;
    serde_json::to_value(data_schema).map_err(|error| error.to_string())
}

fn run_data_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let resource = create_resource(group, test)?;
    let report = validate_data(&resource).map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn run_table_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let resource = create_resource(group, test)?;
    let options: ValidateTableOptions = get_options(group)?;
    let report = validate_table(&resource, &options).map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
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
    let dataset = load_test_dataset(&path)?;
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

fn run_dataset_copy(group: &Value, test: &Value) -> Result<Value, String> {
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = write_dataset(folder.path(), group, test)?;
    let target = folder.path().join("datapackage.json");
    let options = SaveDatasetOptions {
        target: target.to_string_lossy().into_owned(),
        with_remote: false,
    };
    save_dataset(&load_test_dataset(&path)?, &options).map_err(|error| error.to_string())?;
    let text = fs::read_to_string(&target).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

fn run_dataset_infer(group: &Value, test: &Value) -> Result<Value, String> {
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = write_dataset(folder.path(), group, test)?;
    let dataset = load_test_dataset(&path)?;
    let options: InferTableSchemaOptions = get_options(group)?;
    let inferred = infer_dataset(&dataset, &options).map_err(|error| error.to_string())?;
    let basepath = folder.path().to_string_lossy().replace('\\', "/");
    let denormalized =
        denormalize_dataset(&inferred, Some(&basepath)).map_err(|error| error.to_string())?;
    serde_json::to_value(denormalized).map_err(|error| error.to_string())
}

fn run_dataset_query(group: &Value, test: &Value) -> Result<Value, String> {
    let Some(query) = group["query"].as_str() else {
        return Ok(Value::Null);
    };
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = write_dataset(folder.path(), group, test)?;
    let table = query_dataset(path, query, &LoadTableOptions::default())
        .map_err(|error| error.to_string())?;
    read_rows(table)
}

fn run_dataset_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let folder = tempfile::tempdir().map_err(|error| error.to_string())?;
    let path = write_dataset(folder.path(), group, test)?;
    let report = validate_dataset(path, &ValidateTableOptions::default())
        .map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn load_test_dataset(path: &str) -> Result<Dataset, String> {
    let Some(descriptor) = load_dataset(path).map_err(|error| error.to_string())? else {
        return Err(format!("No plugin can load the dataset: {path}"));
    };
    serde_json::from_value(Value::Object(descriptor)).map_err(|error| error.to_string())
}

fn get_options<T: serde::de::DeserializeOwned + Default>(group: &Value) -> Result<T, String> {
    match group.get("options") {
        Some(options) => serde_json::from_value(options.clone()).map_err(|error| error.to_string()),
        None => Ok(T::default()),
    }
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

fn run_table_infer_dialect(group: &Value, test: &Value) -> Result<Value, String> {
    let options = InferFileDialectOptions {
        sample_bytes: group["options"]
            .get("sampleBytes")
            .and_then(Value::as_u64)
            .map(|value| value as usize),
    };
    let dialect = infer_file_dialect(&create_resource(group, test)?, &options)
        .map_err(|error| error.to_string())?;
    serde_json::to_value(dialect).map_err(|error| error.to_string())
}

fn run_table_infer_schema(group: &Value, test: &Value) -> Result<Value, String> {
    let options: InferTableSchemaOptions = get_options(group)?;
    let table_schema = infer_table_schema(&create_resource(group, test)?, &options)
        .map_err(|error| error.to_string())?;
    serde_json::to_value(table_schema).map_err(|error| error.to_string())
}

fn run_table_load(group: &Value, test: &Value) -> Result<Value, String> {
    match load_test_table(group, test)? {
        Some(table) => read_rows(table),
        None => Ok(Value::Null),
    }
}

fn run_table_query(group: &Value, test: &Value) -> Result<Value, String> {
    let (Some(table), Some(query)) = (load_test_table(group, test)?, group["query"].as_str())
    else {
        return Ok(Value::Null);
    };
    read_rows(query_table(table, query).map_err(|error| error.to_string())?)
}

fn load_test_table(group: &Value, test: &Value) -> Result<Option<Table>, String> {
    let options: LoadTableOptions = get_options(group)?;
    let resource = create_resource(group, test)?;
    load_table(&resource, &options).map_err(|error| error.to_string())
}

/// Renders temporal cells as text the way the Python runner's STRFTIME does.
fn read_rows(table: Table) -> Result<Value, String> {
    let frame = table.collect().map_err(|error| error.to_string())?;
    let exprs: Vec<Expr> = frame
        .schema()
        .iter()
        .map(|(name, dtype)| {
            let format = match dtype {
                DataType::Date => Some("%Y-%m-%d"),
                DataType::Time => Some("%H:%M:%S%.f"),
                DataType::Datetime(_, Some(_)) => Some("%Y-%m-%dT%H:%M:%S%.f%:z"),
                DataType::Datetime(_, None) => Some("%Y-%m-%dT%H:%M:%S%.f"),
                _ => None,
            };
            match format {
                Some(format) => col(name.clone()).dt().strftime(format),
                None => col(name.clone()),
            }
        })
        .collect();
    let rendered = frame
        .lazy()
        .select(exprs)
        .collect()
        .map_err(|error| error.to_string())?;

    let columns: Vec<Value> = rendered
        .get_column_names()
        .iter()
        .map(|name| Value::String(name.to_string()))
        .collect();
    let mut rows = Vec::new();
    for index in 0..rendered.height() {
        let row = rendered.get_row(index).map_err(|error| error.to_string())?;
        rows.push(Value::Array(row.0.iter().map(get_cell_value).collect()));
    }
    Ok(serde_json::json!({"columns": columns, "rows": rows}))
}

fn get_cell_value(value: &AnyValue) -> Value {
    match value {
        AnyValue::Null => Value::Null,
        AnyValue::Boolean(value) => Value::Bool(*value),
        AnyValue::String(value) => Value::String(value.to_string()),
        AnyValue::StringOwned(value) => Value::String(value.to_string()),
        AnyValue::Float32(value) => {
            serde_json::Number::from_f64(f64::from(*value)).map_or(Value::Null, Value::Number)
        }
        AnyValue::Float64(value) => {
            serde_json::Number::from_f64(*value).map_or(Value::Null, Value::Number)
        }
        AnyValue::List(series) => {
            Value::Array(series.iter().map(|item| get_cell_value(&item)).collect())
        }
        other => match other.extract::<i64>() {
            Some(integer) if other.is_integer() => Value::from(integer),
            _ => match other.get_str() {
                Some(text) => Value::String(text.to_string()),
                None => Value::String(other.to_string()),
            },
        },
    }
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
        Value::Array(_) => Ok(data.to_string().into_bytes()),
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
        fs::write(folder.join(name), encode_data(content)?).map_err(|error| error.to_string())?;
    }

    let (name, dataset) = match group.get("datapackage") {
        Some(datapackage) => ("datapackage.json", datapackage),
        None => ("dataset.json", &group["dataset"]),
    };
    let path = folder.join(name);
    let dataset: Descriptor =
        serde_json::from_value(dataset.clone()).map_err(|error| error.to_string())?;
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
    format!("{}/../../conformance/{name}", env!("CARGO_MANIFEST_DIR"))
}
