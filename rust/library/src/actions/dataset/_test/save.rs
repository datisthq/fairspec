use std::path::Path;

use fairspec_dataset::{SaveDatasetOptions, get_temp_file_path};
use fairspec_metadata::Dataset;
use serde_json::json;

use crate::actions::dataset::save::save_dataset;

fn create_dataset() -> Dataset {
    serde_json::from_value(json!({"resources": [{"data": [{"id": 1}], "name": "data"}]})).unwrap()
}

fn save(target: String) -> Option<fairspec_dataset::SaveDatasetResult> {
    let options = SaveDatasetOptions {
        target,
        ..Default::default()
    };
    save_dataset(&create_dataset(), &options).unwrap()
}

#[test]
fn test_should_save_dataset_to_zip() {
    assert!(save(get_temp_file_path(false, None, Some("zip")).unwrap()).is_some());
}

#[test]
fn test_should_save_dataset_to_folder() {
    let path = get_temp_file_path(false, None, None).unwrap();
    assert!(save(path.clone()).is_some());
    assert!(Path::new(&path).join("dataset.json").is_file());
}

#[test]
fn test_should_return_none_for_unsupported_target() {
    let folder = tempfile::tempdir().unwrap();
    let target = folder
        .path()
        .join("unknown.xyz")
        .to_string_lossy()
        .into_owned();
    assert!(save(target).is_none());
}
