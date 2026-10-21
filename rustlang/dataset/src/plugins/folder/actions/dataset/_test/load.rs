use std::fs;

use fairspec_metadata::{Dataset, FileDialect, PathOr};
use serde_json::{Value, json};

use crate::actions::file::temp::{get_temp_file_path, write_temp_file};
use crate::actions::folder::temp::get_temp_folder_path;
use crate::plugins::folder::actions::dataset::load::load_dataset_from_folder;
use crate::plugins::folder::actions::dataset::save::save_dataset_to_folder;

fn create_dataset(descriptor: Value) -> Dataset {
    serde_json::from_value(descriptor).unwrap()
}

fn roundtrip(descriptor: Value) -> Dataset {
    let folder = get_temp_file_path(false, None, None).unwrap();
    save_dataset_to_folder(&create_dataset(descriptor), &folder, false).unwrap();
    load_dataset_from_folder(&folder).unwrap()
}

#[test]
fn test_loads_basic_dataset_from_folder() {
    let result = roundtrip(json!({"resources": [{"name": "test_res", "data": [{"id": 1}]}]}));

    assert_eq!(result.resources.unwrap().len(), 1);
}

#[test]
fn test_loads_dataset_with_metadata() {
    let result = roundtrip(json!({
        "titles": [{"title": "Test Dataset"}],
        "descriptions": [{"description": "A test", "descriptionType": "Abstract"}],
        "version": "1.0",
        "resources": [{"name": "test_res", "data": [{"id": 1}]}],
    }));

    assert_eq!(result.datacite.titles.unwrap()[0].title, "Test Dataset");
    assert_eq!(
        result.datacite.descriptions.unwrap()[0].description,
        "A test"
    );
    assert_eq!(result.datacite.version.as_deref(), Some("1.0"));
}

#[test]
fn test_loads_dataset_with_inline_data_resources() {
    let data = json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]);
    let result = roundtrip(json!({"resources": [{"name": "test_res", "data": data}]}));

    assert_eq!(result.resources.unwrap()[0].data, Some(data));
}

#[test]
fn test_loads_dataset_with_file_resources() {
    let csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    let result = roundtrip(json!({"resources": [{
        "name": "test_res",
        "data": csv_path,
        "fileDialect": {"format": "csv"},
    }]}));

    assert!(result.resources.unwrap()[0].file_dialect.is_some());
}

#[test]
fn test_loads_dataset_with_table_schema() {
    let result = roundtrip(json!({"resources": [{
        "name": "test_res",
        "data": [{"id": 1}],
        "tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}},
    }]}));

    let resources = result.resources.unwrap();
    let Some(PathOr::Value(table_schema)) = &resources[0].table_schema else {
        panic!("expected an inline table schema");
    };
    assert_eq!(table_schema.properties.as_ref().unwrap().len(), 2);
}

#[test]
fn test_loads_dataset_with_multiple_resources() {
    let csv_path = write_temp_file("id,name\n1,alice\n", false, None, Some("csv")).unwrap();
    let result = roundtrip(json!({"resources": [
        {"name": "file_res", "data": csv_path},
        {"name": "inline_res", "data": [{"id": 1}]},
    ]}));

    let resources = result.resources.unwrap();
    assert_eq!(resources.len(), 2);
    assert_eq!(resources[0].name.as_deref(), Some("file_res"));
    assert_eq!(resources[1].name.as_deref(), Some("inline_res"));
}

#[test]
fn test_loads_dataset_with_delimiter() {
    let csv_path = write_temp_file("id;name\n1;alice\n", false, None, Some("csv")).unwrap();
    let result = roundtrip(json!({"resources": [{
        "name": "test_res",
        "data": csv_path,
        "fileDialect": {"format": "csv", "delimiter": ";"},
    }]}));

    let resources = result.resources.unwrap();
    let Some(PathOr::Value(FileDialect::Csv(dialect))) = &resources[0].file_dialect else {
        panic!("expected an inline csv dialect");
    };
    assert_eq!(dialect.delimiter.as_deref(), Some(";"));
}

#[test]
fn test_throws_error_for_non_existent_folder() {
    assert!(load_dataset_from_folder("/non/existent/folder").is_err());
}

#[test]
fn test_throws_error_for_folder_without_dataset_json() {
    let folder = get_temp_folder_path(false).unwrap();
    fs::create_dir_all(&folder).unwrap();

    assert!(load_dataset_from_folder(&folder).is_err());
}
