use std::fs;
use std::path::Path;

use fairspec_metadata::{Dataset, Descriptor};
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::models::dataset::SaveDatasetOptions;
use crate::plugin::DatasetPlugin;
use crate::plugins::descriptor::plugin::DescriptorPlugin;

mod load_dataset {
    use super::*;

    fn write_descriptor(folder: &Path, name: &str) -> String {
        let path = folder.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(
            &path,
            r#"{"resources":[{"name":"test","data":[{"id":1}]}]}"#,
        )
        .unwrap();
        format_path(&path)
    }

    fn expected() -> Option<Descriptor> {
        match json!({"resources": [{"name": "test", "data": [{"id": 1}]}]}) {
            Value::Object(descriptor) => Some(descriptor),
            _ => None,
        }
    }

    #[test]
    fn test_loads_from_local_datapackage_json() {
        let folder = tempfile::tempdir().unwrap();
        let path = write_descriptor(folder.path(), "datapackage.json");

        assert_eq!(DescriptorPlugin.load_dataset(&path).unwrap(), expected());
    }

    #[test]
    fn test_loads_from_local_json() {
        let folder = tempfile::tempdir().unwrap();
        let path = write_descriptor(folder.path(), "dataset.json");

        assert_eq!(DescriptorPlugin.load_dataset(&path).unwrap(), expected());
    }

    #[test]
    fn test_loads_from_absolute_path() {
        let folder = tempfile::tempdir().unwrap();
        let path = write_descriptor(folder.path(), "absolute/path/datapackage.json");
        assert!(Path::new(&path).is_absolute());

        assert_eq!(DescriptorPlugin.load_dataset(&path).unwrap(), expected());
    }

    #[test]
    fn test_returns_none_for_csv() {
        assert_eq!(DescriptorPlugin.load_dataset("./data.csv").unwrap(), None);
    }

    #[test]
    fn test_returns_none_for_xlsx() {
        assert_eq!(DescriptorPlugin.load_dataset("./data.xlsx").unwrap(), None);
    }

    #[test]
    fn test_returns_none_for_parquet() {
        assert_eq!(
            DescriptorPlugin.load_dataset("./data.parquet").unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_zenodo_url() {
        assert_eq!(
            DescriptorPlugin
                .load_dataset("https://zenodo.org/record/123")
                .unwrap(),
            None
        );
    }
}

mod save_dataset {
    use super::*;

    fn create_dataset() -> Dataset {
        serde_json::from_value(json!({"resources": [{"name": "test", "data": [{"id": 1}]}]}))
            .unwrap()
    }

    fn save(target: &str, with_remote: bool) -> Option<String> {
        let options = SaveDatasetOptions {
            target: target.to_string(),
            with_remote,
        };
        DescriptorPlugin
            .save_dataset(&create_dataset(), &options)
            .unwrap()
            .map(|result| result.path.unwrap())
    }

    fn assert_saved(name: &str, with_remote: bool) {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join(name));

        assert_eq!(save(&target, with_remote), Some(target.clone()));
        let saved: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(
            saved["resources"],
            json!([{"name": "test", "data": [{"id": 1}]}])
        );
    }

    fn assert_not_saved(name: &str) {
        let folder = tempfile::tempdir().unwrap();
        let target = if name.contains("://") {
            name.to_string()
        } else {
            format_path(&folder.path().join(name))
        };

        assert_eq!(save(&target, false), None);
        assert!(!Path::new(&target).exists());
    }

    #[test]
    fn test_saves_to_local_datapackage_json() {
        assert_saved("datapackage.json", false);
    }

    #[test]
    fn test_saves_with_absolute_path() {
        assert_saved("absolute/path/datapackage.json", false);
    }

    #[test]
    fn test_returns_none_for_remote_https() {
        assert_not_saved("https://example.com/datapackage.json");
    }

    #[test]
    fn test_returns_none_for_remote_http() {
        assert_not_saved("http://example.com/datapackage.json");
    }

    #[test]
    fn test_returns_none_for_non_datapackage_json() {
        assert_not_saved("dataset.json");
    }

    #[test]
    fn test_returns_none_for_csv() {
        assert_not_saved("data.csv");
    }

    #[test]
    fn test_returns_none_for_xlsx() {
        assert_not_saved("data.xlsx");
    }

    #[test]
    fn test_returns_none_for_directory() {
        assert_not_saved("data");
    }

    #[test]
    fn test_ignores_with_remote_option() {
        assert_saved("datapackage.json", true);
    }
}
