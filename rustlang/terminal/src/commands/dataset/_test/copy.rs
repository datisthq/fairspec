use serde_json::json;

use crate::_test::{run, write_file};

#[test]
fn test_should_copy_dataset_to_folder() {
    let source = tempfile::tempdir().unwrap();
    write_file(source.path(), "users.csv", "id,name\n1,Alice\n");
    let descriptor = json!({"resources": [{"name": "users", "data": "users.csv"}]});
    let path = write_file(source.path(), "datapackage.json", &descriptor.to_string());
    let target = source.path().join("out");

    let result = run(&[
        "dataset",
        "copy",
        &path,
        "--to-path",
        &target.to_string_lossy(),
    ]);

    assert_eq!(result.code, 0, "{}{}", result.stdout, result.stderr);
    assert!(target.join("dataset.json").is_file());
    assert!(target.join("users.csv").is_file());
}
