use std::fs;

use fairspec_dataset::get_temp_folder_path;

use crate::actions::dataset::load::load_dataset;

#[test]
fn test_should_load_dataset_from_descriptor_path() {
    let folder = get_temp_folder_path(false).unwrap();
    let path = format!("{folder}/datapackage.json");
    fs::write(
        &path,
        r#"{"resources": [{"data": "data.csv", "name": "data"}]}"#,
    )
    .unwrap();
    let result = load_dataset(&path).unwrap().unwrap();
    assert!(result.contains_key("resources"));
}

#[test]
fn test_should_return_none_for_unsupported_source() {
    assert!(load_dataset("nonexistent-source").unwrap().is_none());
}
