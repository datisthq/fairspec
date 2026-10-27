use std::fs;
use std::path::Path;

use fairspec_metadata::{Dataset, Descriptor};
use serde_json::{Value, json};

use crate::actions::file::temp::format_path;
use crate::helpers::test_server::serve;
use crate::models::dataset::SaveDatasetOptions;
use crate::plugin::DatasetPlugin;
use crate::plugins::folder::plugin::FolderPlugin;

mod load_dataset {
    use super::*;

    #[test]
    fn test_loads_from_local_directory() {
        let folder = tempfile::tempdir().unwrap();
        fs::write(
            folder.path().join("dataset.json"),
            r#"{"resources":[{"name":"test","data":[{"id":1}]}]}"#,
        )
        .unwrap();

        let result = FolderPlugin
            .load_dataset(&format_path(folder.path()))
            .unwrap();

        let expected: Descriptor =
            serde_json::from_value(json!({"resources": [{"name": "test", "data": [{"id": 1}]}]}))
                .unwrap();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_returns_none_for_http() {
        assert_eq!(
            FolderPlugin
                .load_dataset("http://example.com/data")
                .unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_https() {
        assert_eq!(
            FolderPlugin
                .load_dataset("https://example.com/data")
                .unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_ftp() {
        assert_eq!(
            FolderPlugin.load_dataset("ftp://example.com/data").unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_github() {
        assert_eq!(
            FolderPlugin
                .load_dataset("https://github.com/owner/repo/data")
                .unwrap(),
            None
        );
    }

    #[test]
    fn test_returns_none_for_zenodo() {
        assert_eq!(
            FolderPlugin
                .load_dataset("https://zenodo.org/record/123")
                .unwrap(),
            None
        );
    }
}

mod save_dataset {
    use super::*;

    fn create_dataset(data: Value) -> Dataset {
        serde_json::from_value(json!({"resources": [{"name": "test", "data": data}]})).unwrap()
    }

    fn options(target: &str, with_remote: bool) -> SaveDatasetOptions {
        SaveDatasetOptions {
            target: target.to_string(),
            with_remote,
        }
    }

    fn read_resources(folder: &str) -> Value {
        let text = fs::read_to_string(Path::new(folder).join("dataset.json")).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()["resources"].clone()
    }

    #[test]
    fn test_saves_to_local_directory() {
        let root = tempfile::tempdir().unwrap();
        let target = format_path(&root.path().join("test"));

        let result = FolderPlugin
            .save_dataset(
                &create_dataset(json!([{"id": 1}])),
                &options(&target, false),
            )
            .unwrap()
            .unwrap();

        assert_eq!(result.path.as_deref(), Some(target.as_str()));
        assert_eq!(
            read_resources(&target),
            json!([{"name": "test", "data": [{"id": 1}]}])
        );
    }

    #[test]
    fn test_saves_with_remote_option() {
        let (url, server) = serve(vec![(200, b"id\n1\n".to_vec())]);
        let root = tempfile::tempdir().unwrap();
        let target = format_path(&root.path().join("test"));
        let dataset = create_dataset(json!(format!("{url}/table.csv")));

        let result = FolderPlugin
            .save_dataset(&dataset, &options(&target, true))
            .unwrap()
            .unwrap();
        server.join().unwrap();

        assert_eq!(result.path.as_deref(), Some(target.as_str()));
        assert_eq!(read_resources(&target)[0]["data"], json!("table.csv"));
        assert_eq!(
            fs::read_to_string(root.path().join("test/table.csv")).unwrap(),
            "id\n1\n"
        );
    }

    #[test]
    fn test_returns_none_for_http() {
        let result = FolderPlugin
            .save_dataset(
                &create_dataset(json!([{"id": 1}])),
                &options("http://example.com/data", false),
            )
            .unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn test_returns_none_for_https() {
        let result = FolderPlugin
            .save_dataset(
                &create_dataset(json!([{"id": 1}])),
                &options("https://example.com/data", false),
            )
            .unwrap();

        assert_eq!(result, None);
    }
}
