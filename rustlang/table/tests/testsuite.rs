use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine;
use fairspec_dataset::{
    InferFileDialectOptions, describe_file, load_file, preview_file, validate_file, write_temp_file,
};
use fairspec_metadata::{
    DataError, Dataset, Descriptor, ExpectedResourceType, FairspecError, FileDialect,
    IntegrityType, PathOr, Report, Resource, ResourceError, ResourceTypeError, TableSchema,
    create_report, get_data_first_path, get_data_value, get_supported_file_dialect,
    infer_resource_name, inspect_json, load_dataset_descriptor, resolve_data_schema,
    resolve_table_schema, validate_data_schema, validate_table_schema,
};
use fairspec_table::polars::prelude::*;
use fairspec_table::{
    ArrowPlugin, CsvPlugin, InlinePlugin, JsonPlugin, LoadTableOptions, ParquetPlugin,
    SqlTableCheck, SqlitePlugin, Table, TableCheck, TablePlugin, XlsxPlugin,
    infer_table_schema_from_table, inspect_table, query_table,
};
use serde_json::{Map, Value, json};

const UNSPECIFIED_MESSAGE_ERRORS: [&str; 3] = ["data", "metadata", "cell/json"];

/// Operations that need table checks, inference or the library layer, not ported yet.
const SKIPPED: [&str; 3] = ["dataset-infer", "dataset-query", "dataset-validate"];

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
        "data-infer-schema" => Some(run_data_infer_schema),
        "data-validate" => Some(run_data_validate),
        "data-validate-schema" => Some(run_data_validate_schema),
        "dataset-list" => Some(run_dataset_list),
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
    let data = load_data(&resource).map_err(|error| error.to_string())?;
    Ok(data.as_ref().map_or(Value::Null, infer_value_schema))
}

fn run_data_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let resource = create_resource(group, test)?;
    let report = validate_data(&resource).map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

fn run_table_validate(group: &Value, test: &Value) -> Result<Value, String> {
    let resource = create_resource(group, test)?;
    let options = group
        .get("options")
        .cloned()
        .unwrap_or(Value::Object(Map::new()));
    let report = validate_table(resource, &options).map_err(|error| error.to_string())?;
    serde_json::to_value(report).map_err(|error| error.to_string())
}

/// The library's `validate_table`, over the table plugins ported so far.
fn validate_table(mut resource: Resource, options: &Value) -> fairspec_metadata::Result<Report> {
    if resource.file_dialect.is_none() {
        resource.file_dialect =
            infer_file_dialect(&resource, &Default::default())?.map(PathOr::Value);
    }

    let no_infer = options["noInfer"].as_bool().unwrap_or(false);
    let max_errors = options["maxErrors"]
        .as_u64()
        .map_or(1000, |value| value as usize);
    let checks: Vec<TableCheck> = match options.get("checks") {
        Some(checks) => serde_json::from_value::<Vec<SqlTableCheck>>(checks.clone())
            .map_err(parse_error)?
            .into_iter()
            .map(TableCheck::from)
            .collect(),
        None => Vec::new(),
    };
    let mut load_options: LoadTableOptions =
        serde_json::from_value(options.clone()).map_err(parse_error)?;

    let mut table_schema = resolve_table_schema(resource.table_schema.as_ref())?;
    if table_schema.is_none() && !no_infer {
        table_schema = infer_table_schema(&resource, &load_options)?;
    }

    if table_schema.is_none() && checks.is_empty() {
        return Ok(create_report(Vec::new(), None));
    }

    load_options.denormalized = Some(true);
    let Some(table) = load_table(&resource, &load_options)? else {
        let error = ResourceTypeError {
            resource_name: None,
            expected_resource_type: ExpectedResourceType::Table,
            referencing_resource_name: None,
        };
        return Ok(create_report(
            vec![FairspecError::Resource(ResourceError::Type(error))],
            None,
        ));
    };

    let errors = inspect_table(
        table,
        table_schema.as_ref(),
        Some(&checks),
        None,
        Some(max_errors),
        load_options.infer_table_schema_options.concurrency,
    )?;
    Ok(create_report(
        errors.into_iter().map(FairspecError::Table).collect(),
        None,
    ))
}

fn infer_file_dialect(
    resource: &Resource,
    options: &InferFileDialectOptions,
) -> fairspec_metadata::Result<Option<FileDialect>> {
    for plugin in get_table_plugins() {
        if let Some(dialect) = plugin.infer_file_dialect(resource, options)? {
            return Ok(Some(dialect));
        }
    }
    Ok(None)
}

fn infer_table_schema(
    resource: &Resource,
    options: &LoadTableOptions,
) -> fairspec_metadata::Result<Option<TableSchema>> {
    let infer_options = &options.infer_table_schema_options;
    for plugin in get_table_plugins() {
        if let Some(table_schema) = plugin.infer_table_schema(resource, infer_options)? {
            return Ok(Some(table_schema));
        }
    }
    let load_options = LoadTableOptions {
        denormalized: Some(true),
        ..Default::default()
    };
    match load_table(resource, &load_options)? {
        Some(table) => infer_table_schema_from_table(table, infer_options).map(Some),
        None => Ok(None),
    }
}

fn load_table(
    resource: &Resource,
    options: &LoadTableOptions,
) -> fairspec_metadata::Result<Option<Table>> {
    for plugin in get_table_plugins() {
        if let Some(table) = plugin.load_table(resource, options)? {
            return Ok(Some(table));
        }
    }
    Ok(None)
}

/// The library's `validate_data`.
fn validate_data(resource: &Resource) -> fairspec_metadata::Result<Report> {
    let Some(data_schema) = resolve_data_schema(resource.data_schema.as_ref())? else {
        return Ok(create_report(Vec::new(), None));
    };

    let Some(data) = load_data(resource)? else {
        let error = ResourceTypeError {
            resource_name: None,
            expected_resource_type: ExpectedResourceType::Data,
            referencing_resource_name: None,
        };
        return Ok(create_report(
            vec![FairspecError::Resource(ResourceError::Type(error))],
            None,
        ));
    };

    let errors = inspect_json(&data, data_schema, None)?
        .into_iter()
        .map(|issue| {
            FairspecError::Data(DataError {
                r#type: Default::default(),
                resource_name: None,
                message: issue.message,
                json_pointer: issue.json_pointer,
            })
        })
        .collect();
    Ok(create_report(errors, None))
}

/// The library's `load_data`.
fn load_data(resource: &Resource) -> fairspec_metadata::Result<Option<Value>> {
    if let Some(value) = get_data_value(resource) {
        return Ok(Some(value.clone()));
    }

    if let Some(first_path) = get_data_first_path(resource)
        && get_supported_file_dialect(resource, &["json"])?.is_some()
    {
        let bytes = load_file(&first_path, None)?;
        return Ok(Some(serde_json::from_slice(&bytes).map_err(parse_error)?));
    }

    Ok(None)
}

/// The library's `infer_data_schema`, value by value.
fn infer_value_schema(value: &Value) -> Value {
    match value {
        Value::Null => json!({"type": "null"}),
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Number(number) if number.is_f64() => {
            let is_integer = number.as_f64().is_some_and(|value| value.fract() == 0.0);
            json!({"type": if is_integer { "integer" } else { "number" }})
        }
        Value::Number(_) => json!({"type": "integer"}),
        Value::String(_) => json!({"type": "string"}),
        Value::Array(items) => {
            let items: Vec<Value> = items.iter().map(infer_value_schema).collect();
            match items.first() {
                Some(first) if items.iter().all(|item| item == first) => {
                    json!({"type": "array", "items": first})
                }
                _ => json!({"type": "array"}),
            }
        }
        Value::Object(object) if object.is_empty() => json!({"type": "object"}),
        Value::Object(object) => {
            let properties: Map<String, Value> = object
                .iter()
                .map(|(key, item)| (key.clone(), infer_value_schema(item)))
                .collect();
            json!({"type": "object", "properties": properties})
        }
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
    let options: LoadTableOptions = match group.get("options") {
        Some(options) => {
            serde_json::from_value(options.clone()).map_err(|error| error.to_string())?
        }
        None => LoadTableOptions::default(),
    };
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
    let options: LoadTableOptions = match group.get("options") {
        Some(options) => {
            serde_json::from_value(options.clone()).map_err(|error| error.to_string())?
        }
        None => LoadTableOptions::default(),
    };
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

fn parse_error(error: serde_json::Error) -> fairspec_metadata::Error {
    fairspec_metadata::Error::Parse(error.to_string())
}

/// The library's registry order: each plugin registers at the front, so the last
/// registered (xlsx) is asked first.
fn get_table_plugins() -> [&'static dyn TablePlugin; 7] {
    [
        &XlsxPlugin,
        &SqlitePlugin,
        &ParquetPlugin,
        &JsonPlugin,
        &InlinePlugin,
        &CsvPlugin,
        &ArrowPlugin,
    ]
}

fn get_suite_path(name: &str) -> String {
    format!("{}/../../testsuite/{name}", env!("CARGO_MANIFEST_DIR"))
}
