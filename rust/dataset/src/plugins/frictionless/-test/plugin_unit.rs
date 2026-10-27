use std::fs;

use fairspec_metadata::{Dataset, Error};
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::models::dataset::SaveDatasetOptions;
use crate::plugin::DatasetPlugin;
use crate::plugins::frictionless::plugin::FrictionlessPlugin;

mod load_dataset {
    use super::*;

    fn write_package(name: &str, descriptor: &Value) -> (tempfile::TempDir, String) {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join(name);
        fs::write(&path, descriptor.to_string()).unwrap();
        fs::write(folder.path().join("data.csv"), "id\n1\n").unwrap();
        (folder, format_path(&path))
    }

    #[test]
    fn test_loads_datapackage_json() {
        let (_folder, path) = write_package(
            "datapackage.json",
            &json!({"resources": [{"name": "data-table", "path": "data.csv"}]}),
        );

        let dataset = FrictionlessPlugin.load_dataset(&path).unwrap().unwrap();
        let resource = &dataset["resources"][0];
        assert_eq!(resource["name"], "data_table");
        assert_eq!(resource["fileDialect"], json!({"format": "csv"}));
    }

    #[test]
    fn test_loads_fairspec_dataset_as_is() {
        let (_folder, path) = write_package(
            "datapackage.json",
            &json!({
                "$schema": "https://fairspec.org/profiles/latest/dataset.json",
                "resources": [{"name": "table", "data": "data.csv"}],
            }),
        );

        let dataset = FrictionlessPlugin.load_dataset(&path).unwrap().unwrap();
        assert_eq!(dataset["resources"][0]["name"], "table");
        assert!(dataset["resources"][0].get("fileDialect").is_none());
    }

    #[test]
    fn test_reports_model_mismatches() {
        let (_folder, path) = write_package(
            "datapackage.json",
            &json!({"resources": [{"path": ["a.csv", 1], "bytes": 1.5}]}),
        );

        let Err(Error::Invalid { report, .. }) = FrictionlessPlugin.load_dataset(&path) else {
            panic!("expected an invalid package");
        };
        let pointers: Vec<Value> = serde_json::to_value(report.unwrap().errors)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|error| error["jsonPointer"].clone())
            .collect();
        assert_eq!(
            pointers,
            vec![json!("/resources/0/path"), json!("/resources/0/bytes")]
        );
    }

    #[test]
    fn test_returns_none_for_other_json() {
        assert_eq!(
            FrictionlessPlugin.load_dataset("./dataset.json").unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_csv() {
        assert_eq!(FrictionlessPlugin.load_dataset("./data.csv").unwrap(), None);
    }

    #[test]
    fn test_returns_none_for_zenodo_url() {
        assert_eq!(
            FrictionlessPlugin
                .load_dataset("https://zenodo.org/record/123")
                .unwrap(),
            None
        );
    }
}

mod save_dataset {
    use super::*;

    fn save(target: &str) -> Option<String> {
        let dataset: Dataset =
            serde_json::from_value(json!({"resources": [{"data": [{"id": 1}]}]})).unwrap();
        let options = SaveDatasetOptions {
            target: target.to_string(),
            with_remote: false,
        };
        FrictionlessPlugin
            .save_dataset(&dataset, &options)
            .unwrap()
            .map(|result| result.path.unwrap())
    }

    #[test]
    fn test_saves_datapackage_json() {
        let folder = tempfile::tempdir().unwrap();
        let target = format_path(&folder.path().join("datapackage.json"));

        assert_eq!(save(&target), Some(target.clone()));
        let saved: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(
            saved,
            json!({
                "$schema": "https://datapackage.org/profiles/2.0/datapackage.json",
                "resources": [{"name": "resource1", "data": [{"id": 1}]}],
            })
        );
    }

    #[test]
    fn test_returns_none_for_other_json() {
        let folder = tempfile::tempdir().unwrap();
        assert_eq!(
            save(&format_path(&folder.path().join("dataset.json"))),
            None
        );
    }

    #[test]
    fn test_returns_none_for_remote_datapackage_json() {
        assert_eq!(save("https://example.com/datapackage.json"), None);
    }
}
