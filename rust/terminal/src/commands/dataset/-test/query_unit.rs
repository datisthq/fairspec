use serde_json::json;

use crate::_test::{run, write_file};

#[test]
fn test_should_join_two_resources_by_name() {
    let folder = tempfile::tempdir().unwrap();
    let descriptor = json!({"resources": [
        {"name": "users", "data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]},
        {"name": "posts", "data": [{"id": 1, "user_id": 2}, {"id": 2, "user_id": 2}]},
    ]});
    let path = write_file(folder.path(), "datapackage.json", &descriptor.to_string());

    let result = run(&[
        "dataset",
        "query",
        &path,
        "SELECT users.name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id GROUP BY users.name",
        "--json",
    ]);

    assert_eq!(result.code, 0, "{}{}", result.stdout, result.stderr);
    assert_eq!(result.json(), json!([{"name": "bob", "n": 2}]));
}
