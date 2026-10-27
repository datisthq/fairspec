use serde_json::json;

use crate::_test::{run, write_file};

#[test]
fn test_should_list_resource_names_from_descriptor() {
    let folder = tempfile::tempdir().unwrap();
    let descriptor = json!({"resources": [{"name": "products", "data": "products.csv"}]});
    let path = write_file(folder.path(), "datapackage.json", &descriptor.to_string());

    let result = run(&["dataset", "list", &path, "--json"]);

    assert_eq!(result.code, 0, "{}", result.stderr);
    assert_eq!(result.json(), json!(["products"]));
}
