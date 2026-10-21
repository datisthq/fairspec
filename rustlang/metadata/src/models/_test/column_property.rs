use serde_json::json;

use super::{parse, round_trip};
use crate::models::column::base::{BasePropertyType, NullPosition};
use crate::models::column::categorical::CategoricalColumnProperty;
use crate::models::column::column::{Column, ColumnProperty};

#[test]
fn test_dispatch_by_type_and_format() {
    let cases = [
        (json!({"type": "boolean"}), "Boolean"),
        (json!({"type": "integer"}), "Integer"),
        (
            json!({"type": "integer", "format": "categorical"}),
            "IntegerCategorical",
        ),
        (json!({"type": "number"}), "Number"),
        (json!({"type": "string"}), "String"),
        (json!({"type": "string", "format": null}), "String"),
        (json!({"type": "string", "format": "list"}), "List"),
        (json!({"type": "string", "format": "base64"}), "Base64"),
        (json!({"type": "string", "format": "hex"}), "Hex"),
        (json!({"type": "string", "format": "email"}), "Email"),
        (json!({"type": "string", "format": "url"}), "Url"),
        (json!({"type": "string", "format": "date-time"}), "DateTime"),
        (
            json!({"type": "string", "format": "local-date-time"}),
            "LocalDateTime",
        ),
        (json!({"type": "string", "format": "date"}), "Date"),
        (json!({"type": "string", "format": "time"}), "Time"),
        (
            json!({"type": "string", "format": "local-time"}),
            "LocalTime",
        ),
        (json!({"type": "string", "format": "duration"}), "Duration"),
        (json!({"type": "string", "format": "wkt"}), "Wkt"),
        (json!({"type": "string", "format": "wkb"}), "Wkb"),
        (
            json!({"type": "string", "format": "categorical"}),
            "StringCategorical",
        ),
        (json!({"type": "string", "format": "decimal"}), "Decimal"),
        (json!({"type": "array"}), "Array"),
        (json!({"type": "object"}), "Object"),
        (json!({"type": "object", "format": "geojson"}), "Geojson"),
        (json!({"type": "object", "format": "topojson"}), "Topojson"),
        (json!({"title": "no type"}), "Unknown"),
        (json!({"type": null}), "Unknown"),
        (json!({"type": "null"}), "Unknown"),
    ];
    for (input, variant) in cases {
        let property: ColumnProperty = parse(input.clone());
        let debug = format!("{property:?}");
        assert!(
            debug.starts_with(&format!("{variant}(")),
            "{input} parsed as {debug}"
        );
    }
}

#[test]
fn test_round_trip_nullable_types() {
    for input in [
        json!({"type": "integer"}),
        json!({"type": ["integer", "null"]}),
        json!({"type": ["null", "integer"]}),
        json!({"type": ["null", "string"], "format": "date"}),
        json!({"type": ["object", "null"], "format": "geojson"}),
    ] {
        assert_eq!(round_trip::<ColumnProperty>(&input), input);
    }
}

#[test]
fn test_nullable_helpers() {
    let mut property: ColumnProperty = parse(json!({"type": "string", "format": "email"}));
    assert_eq!(property.base_type(), Some(BasePropertyType::String));
    assert!(!property.is_nullable());
    property.set_nullable();
    assert!(property.is_nullable());
    assert_eq!(property.null_position(), Some(NullPosition::After));
    assert_eq!(
        serde_json::to_value(&property).unwrap(),
        json!({"type": ["string", "null"], "format": "email"})
    );

    let unknown: ColumnProperty = parse(json!({}));
    assert_eq!(unknown.base_type(), None);
    assert!(!unknown.is_nullable());
}

#[test]
fn test_round_trip_property_fields() {
    for input in [
        json!({"type": "integer", "title": "Id", "rdfType": "x", "minimum": 1, "multipleOf": 2,
               "missingValues": [-1, "NA", {"value": 0, "label": "zero"}]}),
        json!({"type": "number", "minimum": 1.5, "default": 1.0, "decimalChar": ","}),
        json!({"type": "boolean", "trueValues": ["Y"], "missingValues": ["-", 0, {"value": "x", "label": "y"}]}),
        json!({"type": ["string", "null"], "format": "list", "itemType": "local-date-time", "delimiter": ";", "minItems": 1}),
        json!({"type": "string", "format": "categorical", "categories": ["a", {"value": "b", "label": "B"}], "withOrder": true}),
        json!({"type": "integer", "format": "categorical", "categories": [1, {"value": 2, "label": "b"}]}),
        json!({"type": "string", "format": "decimal", "minimum": 3.0, "groupChar": " ", "withText": true}),
        json!({"type": "string", "format": "date-time", "temporalFormat": "%Y", "enum": ["2024"]}),
        json!({"type": "array", "items": {"type": "integer"}, "minItems": 2.0, "uniqueItems": true, "if": {}, "else": {}, "not": {}}),
        json!({"type": "object", "const": {"a": 1}, "required": ["a"], "minProperties": 1.0, "additionalProperties": false}),
        json!({"enum": [1, "a"], "default": 3, "examples": [null], "missingValues": ["x", {"value": "y", "label": "z"}]}),
    ] {
        assert_eq!(round_trip::<ColumnProperty>(&input), input);
    }
}

#[test]
fn test_float_fields_serialize_as_floats() {
    let output = round_trip::<ColumnProperty>(&json!({"type": "number", "minimum": 0}));
    assert_eq!(output, json!({"type": "number", "minimum": 0.0}));
}

#[test]
fn test_unknown_fields_are_dropped() {
    let output =
        round_trip::<ColumnProperty>(&json!({"type": "string", "format": "date", "minimum": 1}));
    assert_eq!(output, json!({"type": "string", "format": "date"}));
}

#[test]
fn test_round_trips_json_schema_spellings() {
    for input in [
        json!({"type": ["integer"]}),
        json!({"type": ["number", "integer", "null"]}),
        json!({"type": ["string", "null"], "enum": ["a", null]}),
        json!({"type": "string", "format": "weird"}),
        json!({"type": "boolean", "format": "categorical"}),
    ] {
        assert_eq!(round_trip::<ColumnProperty>(&input), input);
    }
}

#[test]
fn test_rejects_what_python_rejects() {
    for input in [
        json!({"type": "foo"}),
        json!({"type": ["string", "integer"]}),
        json!({"type": ["null"]}),
        json!({"type": "string", "enum": [1]}),
        json!({"type": "string", "default": ["x"]}),
    ] {
        assert!(
            serde_json::from_value::<ColumnProperty>(input.clone()).is_err(),
            "{input} should not parse"
        );
    }
}

#[test]
fn test_column_tagged_by_type() {
    let input = json!({"type": "categorical", "name": "c", "property": {"type": "integer", "format": "categorical"}});
    let column: Column = parse(input.clone());
    let Column::Categorical(categorical) = &column else {
        panic!("expected a categorical column, got {column:?}");
    };
    assert!(matches!(
        categorical.property,
        CategoricalColumnProperty::Integer(_)
    ));
    assert_eq!(serde_json::to_value(&column).unwrap(), input);

    let input = json!({"type": "local-date-time", "name": "d", "nullable": true,
                       "property": {"type": ["string", "null"], "format": "local-date-time"}});
    assert_eq!(round_trip::<Column>(&input), input);
}
