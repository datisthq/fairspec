use serde_json::json;

use crate::_test::write_file;
use crate::helpers::console::Console;
use crate::helpers::resource::select_resource;
use crate::session::Session;

#[test]
fn test_should_select_resource_by_name() {
    let folder = tempfile::tempdir().unwrap();
    let descriptor = json!({"resources": [
        {"name": "users", "data": "users.csv"},
        {"name": "orders", "data": "orders.csv"},
    ]});
    let path = write_file(folder.path(), "datapackage.json", &descriptor.to_string());
    let console = Console::from_writers(Box::new(std::io::sink()), Box::new(std::io::sink()));

    let session = Session::new(&console, false, false, true);
    let resource = select_resource(&session, Some(&path), Some("orders")).unwrap();

    assert_eq!(resource.name.as_deref(), Some("orders"));
}
