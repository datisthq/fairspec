use fairspec_metadata::GeojsonColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::geojson::inspect_geojson_column;

fn create_column(name: &str) -> GeojsonColumn {
    serde_json::from_value(json!({
        "name": name,
        "property": {"type": "object", "format": "geojson"},
    }))
    .unwrap()
}

fn inspect(column: &GeojsonColumn, name: &str, cells: &[Option<&str>]) -> Vec<Value> {
    let table = df!(name => cells).unwrap().lazy();
    let errors = inspect_geojson_column(column, table, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn locate(errors: &[Value]) -> Vec<(Value, Value)> {
    errors
        .iter()
        .map(|error| (error["rowNumber"].clone(), error["cell"].clone()))
        .collect()
}

const POINT: &str = r#"{"type":"Point","coordinates":[0,0]}"#;
const EMPTY_FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;

#[test]
fn test_valid_geojson_point() {
    let cells = [
        Some(POINT),
        Some(r#"{"type":"Point","coordinates":[12.5,41.9]}"#),
        Some(r#"{"type":"Point","coordinates":[-73.9,40.7]}"#),
    ];
    assert!(inspect(&create_column("location"), "location", &cells).is_empty());
}

#[test]
fn test_valid_geojson_geometries() {
    let cells = [
        Some(r#"{"type":"LineString","coordinates":[[0,0],[1,1]]}"#),
        Some(r#"{"type":"Polygon","coordinates":[[[0,0],[1,0],[1,1],[0,1],[0,0]]]}"#),
        Some(r#"{"type":"MultiPoint","coordinates":[[0,0],[1,1]]}"#),
    ];
    assert!(inspect(&create_column("geometry"), "geometry", &cells).is_empty());
}

#[test]
fn test_valid_geojson_feature() {
    let cells = [
        Some(
            r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{"name":"Test"}}"#,
        ),
        Some(
            r#"{"type":"Feature","geometry":{"type":"LineString","coordinates":[[0,0],[1,1]]},"properties":{"id":1}}"#,
        ),
        Some(EMPTY_FEATURE),
    ];
    assert!(inspect(&create_column("feature"), "feature", &cells).is_empty());
}

#[test]
fn test_valid_geojson_feature_collection() {
    let cells = [
        Some(
            r#"{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{}}]}"#,
        ),
        Some(r#"{"type":"FeatureCollection","features":[]}"#),
    ];
    assert!(inspect(&create_column("collection"), "collection", &cells).is_empty());
}

#[test]
fn test_null_values() {
    let mut column = create_column("location");
    column.nullable = Some(true);
    let cells = [Some(POINT), None, Some(EMPTY_FEATURE)];
    assert!(inspect(&column, "location", &cells).is_empty());
}

#[test]
fn test_json_arrays_error() {
    let cells = [Some(POINT), Some("[[0,0],[1,1]]"), Some(EMPTY_FEATURE)];

    let errors = inspect(&create_column("data"), "data", &cells);

    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/type",
            "columnName": "data",
            "rowNumber": 2,
            "cell": "[[0,0],[1,1]]",
            "columnType": "geojson",
        })]
    );
}

#[test]
fn test_invalid_json_error() {
    let cells = [Some(POINT), Some("invalid json"), Some("{broken}")];

    let errors = inspect(&create_column("data"), "data", &cells);

    assert!(errors.iter().all(|error| error["type"] == "cell/type"));
    assert_eq!(
        locate(&errors),
        vec![
            (json!(2), json!("invalid json")),
            (json!(3), json!("{broken}"))
        ]
    );
}

#[test]
fn test_empty_strings_error() {
    let cells = [Some(POINT), Some(""), Some(EMPTY_FEATURE)];

    let errors = inspect(&create_column("data"), "data", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(locate(&errors), vec![(json!(2), json!(""))]);
}

#[test]
fn test_json_primitives_error() {
    let cells = [
        Some(r#""string""#),
        Some("123"),
        Some("true"),
        Some("false"),
        Some("null"),
    ];

    let errors = inspect(&create_column("data"), "data", &cells);

    assert_eq!(
        locate(&errors),
        vec![
            (json!(1), json!(r#""string""#)),
            (json!(2), json!("123")),
            (json!(3), json!("true")),
            (json!(4), json!("false")),
            (json!(5), json!("null")),
        ]
    );
}

#[test]
fn test_invalid_geojson_point_coordinates() {
    let cells = [
        Some(POINT),
        Some(r#"{"type":"Point","coordinates":[0]}"#),
        Some(r#"{"type":"Point","coordinates":[0,0,0,0]}"#),
    ];

    let errors = inspect(&create_column("location"), "location", &cells);

    assert_eq!(
        locate(&errors),
        vec![
            (json!(2), json!(r#"{"type":"Point","coordinates":[0]}"#)),
            (
                json!(3),
                json!(r#"{"type":"Point","coordinates":[0,0,0,0]}"#)
            ),
        ]
    );
}

#[test]
fn test_invalid_geojson_linestring() {
    let cells = [
        Some(r#"{"type":"LineString","coordinates":[[0,0],[1,1]]}"#),
        Some(r#"{"type":"LineString","coordinates":[[0,0]]}"#),
        Some(r#"{"type":"LineString","coordinates":[0,0]}"#),
    ];

    let errors = inspect(&create_column("line"), "line", &cells);

    assert_eq!(
        locate(&errors),
        vec![
            (
                json!(2),
                json!(r#"{"type":"LineString","coordinates":[[0,0]]}"#)
            ),
            (
                json!(3),
                json!(r#"{"type":"LineString","coordinates":[0,0]}"#)
            ),
        ]
    );
}

#[test]
fn test_incomplete_geojson_feature() {
    let cells = [
        Some(
            r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{}}"#,
        ),
        Some(r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]}}"#),
        Some(r#"{"type":"Feature","properties":{}}"#),
    ];

    let errors = inspect(&create_column("feature"), "feature", &cells);

    assert_eq!(
        locate(&errors),
        vec![
            (
                json!(2),
                json!(r#"{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]}}"#)
            ),
            (json!(3), json!(r#"{"type":"Feature","properties":{}}"#)),
        ]
    );
}

#[test]
fn test_invalid_geojson_feature_collection() {
    let cells = [
        Some(
            r#"{"type":"FeatureCollection","features":[{"type":"Feature","geometry":{"type":"Point","coordinates":[0,0]},"properties":{}}]}"#,
        ),
        Some(r#"{"type":"FeatureCollection"}"#),
    ];

    let errors = inspect(&create_column("collection"), "collection", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(
        locate(&errors),
        vec![(json!(2), json!(r#"{"type":"FeatureCollection"}"#))]
    );
}
