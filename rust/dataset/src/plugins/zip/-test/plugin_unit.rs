use std::path::Path;

use fairspec_metadata::{Dataset, Descriptor};
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::helpers::test_server::serve;
use crate::models::dataset::SaveDatasetOptions;
use crate::plugin::DatasetPlugin;
use crate::plugins::zip::actions::dataset::load::load_dataset_from_zip;
use crate::plugins::zip::actions::dataset::save::save_dataset_to_zip;
use crate::plugins::zip::plugin::ZipPlugin;

fn create_dataset(name: &str, data: Value) -> Dataset {
    serde_json::from_value(json!({"resources": [{"name": name, "data": data}]})).unwrap()
}

mod load_dataset {
    use super::*;

    fn save_zip(folder: &Path, name: &str, dataset: &Dataset) -> String {
        let path = folder.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let path = format_path(&path);
        save_dataset_to_zip(dataset, &path, false).unwrap();
        path
    }

    #[test]
    fn test_loads_from_zip_file() {
        let folder = tempfile::tempdir().unwrap();
        let path = save_zip(
            folder.path(),
            "test.zip",
            &create_dataset("test_resource", json!([{"id": 1}])),
        );

        let result = ZipPlugin.load_dataset(&path).unwrap().unwrap();

        assert_eq!(
            result["resources"],
            json!([{"name": "test_resource", "data": [{"id": 1}]}])
        );
    }

    #[test]
    fn test_returns_none_for_non_zip() {
        assert_eq!(ZipPlugin.load_dataset("test.json").unwrap(), None);
    }

    #[test]
    fn test_handles_paths_with_directories() {
        let folder = tempfile::tempdir().unwrap();
        let dataset = create_dataset("test_resource", json!([{"id": 1}]));
        let path = save_zip(folder.path(), "path/to/file.zip", &dataset);

        let result = ZipPlugin.load_dataset(&path).unwrap().unwrap();

        assert_eq!(
            result["resources"],
            json!([{"name": "test_resource", "data": [{"id": 1}]}])
        );
    }

    #[test]
    fn test_returns_none_for_no_extension() {
        assert_eq!(ZipPlugin.load_dataset("test").unwrap(), None);
    }
}

mod save_dataset {
    use super::*;

    fn options(target: &str, with_remote: bool) -> SaveDatasetOptions {
        SaveDatasetOptions {
            target: target.to_string(),
            with_remote,
        }
    }

    fn load_resources(path: &str) -> Value {
        let dataset = load_dataset_from_zip(path).unwrap();
        let descriptor: Descriptor = match serde_json::to_value(dataset).unwrap() {
            Value::Object(descriptor) => descriptor,
            _ => Descriptor::new(),
        };
        descriptor["resources"].clone()
    }

    #[test]
    fn test_saves_to_zip_file() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output.zip"));

        let result = ZipPlugin
            .save_dataset(
                &create_dataset("test_resource", json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap()
            .unwrap();

        assert_eq!(result.path, None);
        assert_eq!(
            load_resources(&target),
            json!([{"name": "test_resource", "data": [{"id": 1}]}])
        );
    }

    #[test]
    fn test_returns_none_for_non_zip() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output.json"));

        let result = ZipPlugin
            .save_dataset(
                &create_dataset("test_resource", json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap();

        assert_eq!(result, None);
        assert!(!Path::new(&target).exists());
    }

    #[test]
    fn test_passes_with_remote_option() {
        let (url, server) = serve(vec![(200, b"id\n1\n".to_vec())]);
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output.zip"));
        let dataset = create_dataset("test_resource", json!(format!("{url}/table.csv")));

        let result = ZipPlugin
            .save_dataset(&dataset, &options(&target, true))
            .unwrap();
        server.join().unwrap();

        assert!(result.is_some());
        let resources = load_resources(&target);
        assert!(
            resources[0]["data"]
                .as_str()
                .unwrap()
                .ends_with("/table.csv")
        );
        assert!(!resources[0]["data"].as_str().unwrap().starts_with("http"));
    }

    #[test]
    fn test_with_remote_defaults_to_false() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output.zip"));
        let remote = "http://127.0.0.1:9/table.csv";
        let options = SaveDatasetOptions {
            target: target.clone(),
            ..Default::default()
        };

        ZipPlugin
            .save_dataset(&create_dataset("test_resource", json!(remote)), &options)
            .unwrap();

        assert_eq!(load_resources(&target)[0]["data"], json!(remote));
    }

    #[test]
    fn test_handles_paths_with_directories() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("path/to/output.zip"));
        std::fs::create_dir_all(folder.path().join("path/to")).unwrap();

        let result = ZipPlugin
            .save_dataset(
                &create_dataset("test_resource", json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap();

        assert!(result.is_some());
        assert!(Path::new(&target).is_file());
    }

    #[test]
    fn test_returns_none_for_no_extension() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output"));

        let result = ZipPlugin
            .save_dataset(
                &create_dataset("test_resource", json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap();

        assert_eq!(result, None);
        assert!(!Path::new(&target).exists());
    }

    #[test]
    fn test_result_has_path_none() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("output.zip"));

        let result = ZipPlugin
            .save_dataset(
                &create_dataset("test_resource", json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap()
            .unwrap();

        assert_eq!(result.path, None);
    }
}
