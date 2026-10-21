use serde_json::{Value, json};

use crate::actions::resource::data::{
    get_data_first_path, get_data_path, get_data_paths, get_data_records, get_data_value,
};
use crate::models::data::ResourceDataPath;
use crate::models::resource::Resource;

fn create_resource(data: Option<Value>) -> Resource {
    Resource {
        data,
        ..Resource::default()
    }
}

#[test]
fn test_get_data_path() {
    let resource = create_resource(Some(json!("table.csv")));
    assert_eq!(
        get_data_path(&resource),
        Some(ResourceDataPath::Path("table.csv".into()))
    );
    let resource = create_resource(Some(json!(["a.csv", "b.csv"])));
    assert_eq!(
        get_data_path(&resource),
        Some(ResourceDataPath::Paths(vec![
            "a.csv".into(),
            "b.csv".into()
        ]))
    );
    assert_eq!(
        get_data_path(&create_resource(Some(json!([{"id": 1}])))),
        None
    );
    assert_eq!(get_data_path(&create_resource(None)), None);
}

#[test]
fn test_get_data_value() {
    let resource = create_resource(Some(json!([{"id": 1}])));
    assert_eq!(get_data_value(&resource), Some(&json!([{"id": 1}])));
    assert_eq!(
        get_data_value(&create_resource(Some(json!("table.csv")))),
        None
    );
}

#[test]
fn test_get_data_records() {
    let resource = create_resource(Some(json!([{"id": 1}])));
    assert_eq!(get_data_records(&resource), Some(&[json!({"id": 1})][..]));
    assert_eq!(
        get_data_records(&create_resource(Some(json!({"id": 1})))),
        None
    );
    assert_eq!(
        get_data_records(&create_resource(Some(json!("table.csv")))),
        None
    );
}

#[test]
fn test_get_data_paths() {
    assert_eq!(
        get_data_paths(&create_resource(Some(json!("table.csv")))),
        vec!["table.csv"]
    );
    assert_eq!(
        get_data_paths(&create_resource(Some(json!(["a.csv", "b.csv"])))),
        vec!["a.csv", "b.csv"]
    );
    assert!(get_data_paths(&create_resource(None)).is_empty());
}

#[test]
fn test_get_data_first_path() {
    let resource = create_resource(Some(json!("table.csv")));
    assert_eq!(get_data_first_path(&resource).as_deref(), Some("table.csv"));
    let resource = create_resource(Some(json!(["a.csv", "b.csv"])));
    assert_eq!(get_data_first_path(&resource).as_deref(), Some("a.csv"));
    assert_eq!(get_data_first_path(&create_resource(None)), None);
}
