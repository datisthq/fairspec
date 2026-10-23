use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::json;

use crate::server::call_tool;
use crate::tools::file::preview::{PreviewFileInput, preview_file_tool};

#[test]
fn test_should_preview_csv_file() {
    let path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    let input: PreviewFileInput =
        serde_json::from_value(json!({"path": path, "maxLines": 2})).unwrap();
    let result = preview_file_tool(input).unwrap();
    assert_eq!(result.text, "id,name\n1,alice\n");
    assert!(result.truncated);
}

#[test]
fn test_should_reject_limits_over_cap() {
    let path = write_temp_file("id\n1\n", false, None, Some("csv")).unwrap();
    for limits in [
        json!({"maxLines": 201}),
        json!({"maxBytes": 20_001}),
        json!({"maxBytes": 0}),
    ] {
        let mut arguments = limits.clone();
        arguments["path"] = json!(path);
        let result = call_tool("previewFileTool", Some(arguments));
        assert_eq!(result.is_error, Some(true), "{limits}");
    }
}
