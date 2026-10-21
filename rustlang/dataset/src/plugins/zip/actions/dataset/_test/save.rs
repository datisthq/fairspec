use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use fairspec_metadata::{Dataset, Error, PathOr};
use serde_json::{Value, json};

use crate::actions::file::temp::{get_temp_file_path, write_temp_file};
use crate::plugins::zip::actions::dataset::load::load_dataset_from_zip;
use crate::plugins::zip::actions::dataset::save::save_dataset_to_zip;

fn create_dataset(descriptor: Value) -> Dataset {
    serde_json::from_value(descriptor).unwrap()
}

fn save(descriptor: Value) -> String {
    let path = get_temp_file_path(false, None, Some("zip")).unwrap();
    save_dataset_to_zip(&create_dataset(descriptor), &path, false).unwrap();
    path
}

fn assert_saved(descriptor: Value) -> String {
    let path = save(descriptor);
    assert!(Path::new(&path).exists());
    path
}

fn table_schema_descriptor() -> Value {
    json!({"resources": [{
        "name": "test_res",
        "data": [{"id": 1}],
        "tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}},
    }]})
}

#[test]
fn test_saves_basic_dataset_to_zip() {
    let path = assert_saved(json!({"resources": [{"name": "test_res", "data": [{"id": 1}]}]}));

    assert!(fs::metadata(path).unwrap().len() > 0);
}

#[test]
fn test_saves_dataset_with_metadata() {
    let path = assert_saved(json!({
        "titles": [{"title": "Test Dataset"}],
        "descriptions": [{"description": "A test", "descriptionType": "Abstract"}],
        "version": "1.0",
        "resources": [{"name": "test_res", "data": [{"id": 1}]}],
    }));

    assert!(fs::metadata(path).unwrap().len() > 0);
}

#[test]
fn test_saves_dataset_with_inline_data_resources() {
    assert_saved(json!({"resources": [{"name": "test_res", "data": [{"id": 1}, {"id": 2}]}]}));
}

#[test]
fn test_saves_dataset_with_file_resources() {
    let csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    assert_saved(json!({"resources": [{"name": "test_res", "data": csv_path}]}));
}

#[test]
fn test_saves_dataset_with_multiple_resources() {
    let csv_path = write_temp_file("id,name\n1,alice\n", false, None, Some("csv")).unwrap();
    assert_saved(json!({"resources": [
        {"name": "file_res", "data": csv_path},
        {"name": "inline_res", "data": [{"id": 1}]},
    ]}));
}

#[test]
fn test_saves_dataset_with_table_schema() {
    assert_saved(table_schema_descriptor());
}

#[test]
fn test_saves_dataset_with_dialect() {
    let csv_path = write_temp_file("id;name\n1;alice\n", false, None, Some("csv")).unwrap();
    assert_saved(json!({"resources": [{
        "name": "test_res",
        "data": csv_path,
        "fileDialect": {"format": "csv", "delimiter": ";"},
    }]}));
}

#[test]
fn test_roundtrip_preserves_structure() {
    let path = save(json!({
        "titles": [{"title": "My Dataset"}],
        "descriptions": [{"description": "Desc", "descriptionType": "Abstract"}],
        "resources": [{"name": "test_res", "data": [{"id": 1}]}],
    }));

    let result = load_dataset_from_zip(&path).unwrap();

    assert_eq!(result.datacite.titles.unwrap()[0].title, "My Dataset");
    assert_eq!(result.datacite.descriptions.unwrap()[0].description, "Desc");
    assert_eq!(result.resources.unwrap().len(), 1);
}

#[test]
fn test_roundtrip_preserves_metadata() {
    let path = save(json!({
        "titles": [{"title": "My Dataset"}],
        "version": "2.0",
        "subjects": [{"subject": "science"}],
        "resources": [{"name": "test_res", "data": [{"id": 1}]}],
    }));

    let result = load_dataset_from_zip(&path).unwrap();

    assert_eq!(result.datacite.titles.unwrap()[0].title, "My Dataset");
    assert_eq!(result.datacite.version.as_deref(), Some("2.0"));
    assert_eq!(result.datacite.subjects.unwrap()[0].subject, "science");
}

#[test]
fn test_roundtrip_preserves_table_schema() {
    let path = save(table_schema_descriptor());

    let result = load_dataset_from_zip(&path).unwrap();

    let resources = result.resources.unwrap();
    let Some(PathOr::Value(table_schema)) = &resources[0].table_schema else {
        panic!("expected an inline table schema");
    };
    assert_eq!(table_schema.properties.as_ref().unwrap().len(), 2);
}

#[test]
fn test_roundtrip_preserves_file_resources() {
    let csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    let path = save(json!({"resources": [{
        "name": "test_res",
        "data": csv_path,
        "fileDialect": {"format": "csv"},
    }]}));

    let result = load_dataset_from_zip(&path).unwrap();

    assert!(result.resources.unwrap()[0].file_dialect.is_some());
}

#[test]
fn test_throws_error_for_existing_file() {
    let path = write_temp_file("existing content", false, None, Some("zip")).unwrap();
    let dataset = create_dataset(json!({"resources": [{"name": "test_res", "data": [{"id": 1}]}]}));

    let error = save_dataset_to_zip(&dataset, &path, false).unwrap_err();

    assert!(matches!(error, Error::Io(error) if error.kind() == ErrorKind::AlreadyExists));
}

#[test]
fn test_creates_valid_zip_structure() {
    let path = save(json!({"resources": [{"name": "test_res", "data": [{"id": 1}]}]}));

    let result = load_dataset_from_zip(&path).unwrap();

    assert_eq!(
        result.resources.unwrap()[0].name.as_deref(),
        Some("test_res")
    );
}

#[test]
fn test_roundtrip_with_multiple_file_resources() {
    let csv1 = write_temp_file("id,name\n1,alice\n", false, Some("data1.csv"), None).unwrap();
    let csv2 = write_temp_file("id,name\n2,bob\n", false, Some("data2.csv"), None).unwrap();
    let path = save(json!({"resources": [
        {"name": "first_res", "data": csv1},
        {"name": "second_res", "data": csv2},
    ]}));

    let result = load_dataset_from_zip(&path).unwrap();

    let resources = result.resources.unwrap();
    assert_eq!(resources.len(), 2);
    assert_eq!(resources[0].name.as_deref(), Some("first_res"));
    assert_eq!(resources[1].name.as_deref(), Some("second_res"));
}
