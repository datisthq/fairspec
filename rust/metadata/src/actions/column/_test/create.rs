use serde_json::{Value, json};

use crate::actions::column::create::create_column_from_property;
use crate::models::column::base::BasePropertyType;
use crate::models::column::column::{Column, ColumnType};

fn create_column(name: &str, property: Value) -> Column {
    let Value::Object(property) = property else {
        panic!("property must be an object");
    };
    create_column_from_property(name, &property).unwrap()
}

#[test]
fn test_creates_string_column() {
    let column = create_column("name", json!({"type": "string"}));
    assert_eq!(column.column_type(), ColumnType::String);
    assert_eq!(column.nullable(), None);
}

#[test]
fn test_creates_nullable_column_for_type_null() {
    let column = create_column("name", json!({"type": ["string", "null"]}));
    assert_eq!(column.column_type(), ColumnType::String);
    assert_eq!(column.nullable(), Some(true));
}

#[test]
fn test_creates_nullable_column_for_null_type() {
    let column = create_column("name", json!({"type": ["null", "string"]}));
    assert_eq!(column.column_type(), ColumnType::String);
    assert_eq!(column.nullable(), Some(true));
}

#[test]
fn test_creates_nullable_date_column() {
    let column = create_column(
        "created",
        json!({"type": ["string", "null"], "format": "date"}),
    );
    assert_eq!(column.column_type(), ColumnType::Date);
    assert_eq!(column.nullable(), Some(true));
}

#[test]
fn test_creates_column_by_type_and_format() {
    let cases = [
        (json!({"type": "integer"}), ColumnType::Integer),
        (json!({"type": "boolean"}), ColumnType::Boolean),
        (json!({"type": "number"}), ColumnType::Number),
        (json!({"type": "array"}), ColumnType::Array),
        (json!({"type": "object"}), ColumnType::Object),
        (
            json!({"type": "object", "format": "geojson"}),
            ColumnType::Geojson,
        ),
        (
            json!({"type": "object", "format": "topojson"}),
            ColumnType::Topojson,
        ),
        (
            json!({"type": "string", "format": "categorical"}),
            ColumnType::Categorical,
        ),
        (
            json!({"type": "integer", "format": "categorical"}),
            ColumnType::Categorical,
        ),
        (json!({}), ColumnType::Unknown),
        (
            json!({"type": "string", "format": "email"}),
            ColumnType::Email,
        ),
        (json!({"type": "string", "format": "url"}), ColumnType::Url),
    ];
    for (property, expected) in cases {
        let column = create_column("x", property.clone());
        assert_eq!(column.column_type(), expected, "{property}");
        assert_eq!(column.name(), "x");
    }
}

#[test]
fn test_creates_a_plain_column_for_an_unknown_format() {
    let Value::Object(property) = json!({"type": "string", "format": "weird"}) else {
        unreachable!()
    };
    let column = create_column_from_property("x", &property).unwrap();
    assert_eq!(
        column.property().base_type(),
        Some(BasePropertyType::String)
    );
}
