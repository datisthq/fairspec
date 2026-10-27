use std::fs;

use fairspec_metadata::Dataset;
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::models::dataset::SaveDatasetOptions;
use crate::plugin::DatasetPlugin;
use crate::plugins::frictionless::plugin::FrictionlessPlugin;

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
