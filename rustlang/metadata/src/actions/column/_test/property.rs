use serde_json::json;

use crate::actions::column::property::{
    get_base_property_type, get_is_nullable_property_type, set_property_nullable,
};
use crate::models::column::base::{NullPosition, StringNullablePropertyType};
use crate::models::column::column::Column;
use crate::models::column::string::{StringColumn, StringColumnProperty};

#[test]
fn test_get_base_property_type() {
    assert_eq!(
        get_base_property_type(Some(&json!("string"))),
        Some("string")
    );
    assert_eq!(
        get_base_property_type(Some(&json!(["string", "null"]))),
        Some("string")
    );
    assert_eq!(
        get_base_property_type(Some(&json!(["null", "string"]))),
        Some("string")
    );
    assert_eq!(get_base_property_type(None), None);
}

#[test]
fn test_get_is_nullable_property_type() {
    assert!(!get_is_nullable_property_type(Some(&json!("string"))));
    assert!(get_is_nullable_property_type(Some(&json!([
        "string", "null"
    ]))));
    assert!(get_is_nullable_property_type(Some(&json!([
        "null", "string"
    ]))));
    assert!(!get_is_nullable_property_type(None));
}

fn create_string_column(null: NullPosition) -> Column {
    let mut property = StringColumnProperty::default();
    property.base.r#type = StringNullablePropertyType::new(null);
    Column::String(StringColumn {
        name: "name".to_string(),
        property,
        ..StringColumn::default()
    })
}

#[test]
fn test_widens_type_to_a_tuple() {
    let mut column = create_string_column(NullPosition::None);
    set_property_nullable(&mut column);
    let property = serde_json::to_value(column.property()).unwrap();
    assert_eq!(property, json!({"type": ["string", "null"]}));
}

#[test]
fn test_keeps_an_already_nullable_type() {
    let mut column = create_string_column(NullPosition::Before);
    set_property_nullable(&mut column);
    let property = serde_json::to_value(column.property()).unwrap();
    assert_eq!(property, json!({"type": ["null", "string"]}));
}
