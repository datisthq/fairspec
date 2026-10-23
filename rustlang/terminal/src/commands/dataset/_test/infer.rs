use crate::_test::{run, write_file};

#[test]
fn test_should_infer_table_schema() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,english\n");

    let result = run(&["dataset", "infer", &path, "--json"]);

    assert_eq!(result.code, 0, "{}", result.stderr);
    let properties = &result.json()["resources"][0]["tableSchema"]["properties"];
    assert_eq!(properties["id"]["type"], "integer");
}

#[test]
fn test_should_pass_table_schema_options() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,english\n");

    let result = run(&["dataset", "infer", &path, "--json", "--keep-strings"]);

    assert_eq!(result.code, 0, "{}", result.stderr);
    let properties = &result.json()["resources"][0]["tableSchema"]["properties"];
    assert_eq!(properties["id"]["type"], "string");
}
