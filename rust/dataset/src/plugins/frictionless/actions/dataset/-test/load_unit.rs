use std::fs;

use fairspec_metadata::{Error, PathOr, resolve_file_dialect, resolve_table_schema};
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::plugins::frictionless::actions::dataset::load::load_dataset_from_frictionless;

fn write_package(files: &[(&str, Value)]) -> (tempfile::TempDir, String) {
    let folder = tempfile::tempdir().unwrap();
    for (name, content) in files {
        fs::write(folder.path().join(name), content.to_string()).unwrap();
    }
    let path = format_path(&folder.path().join("datapackage.json"));
    (folder, path)
}

#[test]
fn test_loads_package_with_schema_and_dialect_paths() {
    let (_folder, path) = write_package(&[
        (
            "datapackage.json",
            json!({"resources": [{
                "name": "my-items",
                "path": "items.csv",
                "schema": "schema.json",
                "dialect": "dialect.json",
            }]}),
        ),
        (
            "schema.json",
            json!({"fields": [{"name": "id", "type": "integer"}]}),
        ),
        ("dialect.json", json!({"delimiter": ";"})),
    ]);

    let dataset = load_dataset_from_frictionless(&path).unwrap();
    let resource: fairspec_metadata::Resource =
        serde_json::from_value(dataset["resources"][0].clone()).unwrap();

    assert_eq!(resource.name.as_deref(), Some("my_items"));
    let file_dialect = resolve_file_dialect(resource.file_dialect.as_ref()).unwrap();
    assert_eq!(
        serde_json::to_value(file_dialect).unwrap(),
        json!({"format": "csv", "delimiter": ";"})
    );
    assert!(matches!(resource.table_schema, Some(PathOr::Path(_))));
    let table_schema = resolve_table_schema(resource.table_schema.as_ref()).unwrap();
    assert_eq!(
        serde_json::to_value(table_schema).unwrap(),
        json!({"properties": {"id": {"type": ["integer", "null"]}}})
    );
}

#[test]
fn test_loads_fairspec_dataset_named_datapackage_json_as_is() {
    let (_folder, path) = write_package(&[(
        "datapackage.json",
        json!({
            "$schema": "https://fairspec.org/profiles/latest/dataset.json",
            "resources": [{"name": "items", "data": [{"id": 1}]}],
        }),
    )]);

    let dataset = load_dataset_from_frictionless(&path).unwrap();
    assert_eq!(
        dataset["resources"],
        json!([{"name": "items", "data": [{"id": 1}]}])
    );
}

#[test]
fn test_raises_metadata_errors_for_invalid_package() {
    let (_folder, path) = write_package(&[(
        "datapackage.json",
        json!({"resources": [{"path": ["a.csv", 1], "bytes": "x"}]}),
    )]);

    let Err(Error::Invalid { report, .. }) = load_dataset_from_frictionless(&path) else {
        panic!("expected an invalid package");
    };
    let errors = serde_json::to_value(report.unwrap().errors).unwrap();
    let pointers: Vec<&Value> = errors
        .as_array()
        .unwrap()
        .iter()
        .map(|error| &error["jsonPointer"])
        .collect();
    assert_eq!(pointers, vec!["/resources/0/path", "/resources/0/bytes"]);
}

#[test]
fn test_maps_dataset_errors_to_package_pointers() {
    let (_folder, path) = write_package(&[(
        "datapackage.json",
        json!({"resources": [
            {"name": "a-b", "path": "one.csv"},
            {"name": "a_b", "path": "two.csv"},
        ]}),
    )]);

    let Err(Error::Invalid { report, .. }) = load_dataset_from_frictionless(&path) else {
        panic!("expected an invalid package");
    };
    let errors = serde_json::to_value(report.unwrap().errors).unwrap();
    assert_eq!(errors[0]["jsonPointer"], "/resources/1/name");
}
