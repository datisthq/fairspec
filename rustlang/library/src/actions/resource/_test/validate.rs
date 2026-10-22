use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::json;

use crate::actions::resource::validate::validate_resource;

fn create_resource(content: &str, with_schema: bool) -> Resource {
    let path = write_temp_file(content, false, None, Some("csv")).unwrap();
    let mut descriptor = json!({"data": path});
    if with_schema {
        descriptor["tableSchema"] =
            json!({"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}});
    }
    serde_json::from_value(descriptor).unwrap()
}

#[test]
fn test_should_validate_valid_resource() {
    let resource = create_resource("id,name\n1,english\n2,中文", true);
    assert!(
        validate_resource(&resource, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_detect_type_error() {
    let resource = create_resource("id,name\nBAD,english", true);
    assert!(
        !validate_resource(&resource, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_validate_resource_without_schema() {
    let resource = create_resource("id,name\n1,english", false);
    assert!(
        validate_resource(&resource, &Default::default())
            .unwrap()
            .valid
    );
}
