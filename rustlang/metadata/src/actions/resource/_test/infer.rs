use serde_json::{Value, json};

use crate::actions::resource::infer::infer_resource_name;
use crate::models::resource::Resource;

fn create_resource(data: Option<Value>) -> Resource {
    Resource {
        data,
        ..Resource::default()
    }
}

#[test]
fn test_infer_resource_name() {
    let cases = [
        (Some(json!("/data/users.csv")), "users"),
        (
            Some(json!(["/data/users.csv", "/data/backup.csv"])),
            "users",
        ),
        (
            Some(json!("https://example.com/data/products.json")),
            "products",
        ),
        (None, "resource"),
        (Some(json!("/data/folder/")), "resource"),
        (Some(json!("/data/file.backup.csv")), "file_backup"),
        (Some(json!("/data/My Data File!.csv")), "my_data_file"),
    ];
    for (data, expected) in cases {
        assert_eq!(
            infer_resource_name(&create_resource(data.clone()), None),
            expected,
            "{data:?}"
        );
    }
}

#[test]
fn test_returns_numbered_default() {
    assert_eq!(
        infer_resource_name(&create_resource(None), Some(1)),
        "resource1"
    );
}
