use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::resource::save::{SaveFileProps, save_resource_files};

#[test]
fn test_save_resource_files() {
    let cases: [(&str, &str, Value, Value, bool, bool); 7] = [
        (
            "local path",
            "data",
            json!({
                "data": "data/table.csv",
                "dataSchema": "data/data-schema.json",
                "tableSchema": "data/table-schema.json",
            }),
            json!({
                "data": "table.csv",
                "dataSchema": "data-schema.json",
                "tableSchema": "table-schema.json",
            }),
            false,
            false,
        ),
        (
            "local paths",
            "data",
            json!({"data": ["data/table1.csv", "data/table2.csv"]}),
            json!({"data": ["table1.csv", "table2.csv"]}),
            false,
            false,
        ),
        (
            "local path and remote path",
            "data",
            json!({
                "data": "data/table.csv",
                "tableSchema": "https://example.com/schema.json",
            }),
            json!({
                "data": "table.csv",
                "tableSchema": "https://example.com/schema.json",
            }),
            false,
            false,
        ),
        (
            "local path and remote path using with_remote",
            "data",
            json!({
                "data": "data/table.csv",
                "tableSchema": "https://example.com/schema.json",
            }),
            json!({
                "data": "table.csv",
                "tableSchema": "schema.json",
            }),
            true,
            false,
        ),
        (
            "remote paths with same filename using with_remote",
            "data",
            json!({
                "data": [
                    "http://example1.com/table.csv",
                    "http://example2.com/table.csv",
                    "http://example3.com/table.csv",
                    "http://example4.com/table.csv.zip",
                    "http://example5.com/table.csv.zip",
                ],
            }),
            json!({
                "data": [
                    "table.csv",
                    "table-1.csv",
                    "table-2.csv",
                    "table.csv.zip",
                    "table-1.csv.zip",
                ],
            }),
            true,
            false,
        ),
        (
            "local paths in different folders",
            "data",
            json!({
                "data": "data/folder1/table.csv",
                "tableSchema": "data/folder2/schema.json",
            }),
            json!({
                "data": "folder1/table.csv",
                "tableSchema": "folder2/schema.json",
            }),
            false,
            false,
        ),
        (
            "local paths in different folders using without_folders",
            "data",
            json!({
                "data": "data/folder1/table.csv",
                "tableSchema": "data/folder2/schema.json",
            }),
            json!({
                "data": "folder1-table.csv",
                "tableSchema": "folder2-schema.json",
            }),
            false,
            true,
        ),
    ];
    for (description, basepath, resource, expected, with_remote, without_folders) in cases {
        let resource: Resource = serde_json::from_value(resource).unwrap();
        let mut identity_save = |props: &SaveFileProps| Ok(props.denormalized_path.clone());
        let result = save_resource_files(
            &resource,
            &mut identity_save,
            Some(basepath),
            with_remote,
            without_folders,
        )
        .unwrap();
        assert_eq!(Value::Object(result), expected, "{description}");
    }
}
