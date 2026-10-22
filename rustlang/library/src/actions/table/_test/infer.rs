use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::json;

use crate::actions::table::infer::infer_table;

#[test]
fn test_should_infer_and_load_table() {
    let path = write_temp_file("id,name\n1,english\n2,中文", false, None, Some("csv")).unwrap();
    let resource: Resource = serde_json::from_value(json!({"data": path})).unwrap();
    let table = infer_table(&resource, &Default::default())
        .unwrap()
        .unwrap();
    assert_eq!(table.collect().unwrap().height(), 2);
}

#[test]
fn test_should_return_none_for_empty_resource() {
    let table = infer_table(&Resource::default(), &Default::default()).unwrap();
    assert!(table.is_none());
}
