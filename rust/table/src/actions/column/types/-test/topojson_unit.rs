use fairspec_metadata::TopojsonColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::topojson::inspect_topojson_column;

fn create_column(name: &str) -> TopojsonColumn {
    serde_json::from_value(json!({
        "name": name,
        "property": {"type": "object", "format": "topojson"},
    }))
    .unwrap()
}

fn inspect(column: &TopojsonColumn, name: &str, cells: &[Option<&str>]) -> Vec<Value> {
    let table = df!(name => cells).unwrap().lazy();
    let errors = inspect_topojson_column(column, table, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

#[test]
fn test_valid_topojson() {
    let cells = [
        Some(
            r#"{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[{"type":"Point","coordinates":[0,0]}]}},"arcs":[]}"#,
        ),
        Some(
            r#"{"type":"Topology","objects":{"collection":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}"#,
        ),
    ];
    assert!(inspect(&create_column("topology"), "topology", &cells).is_empty());
}

#[test]
fn test_invalid_topojson_structure() {
    let cells = [
        Some(
            r#"{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}"#,
        ),
        Some(r#"{"type":"Topology","objects":{}}"#),
        Some(r#"{"type":"Topology"}"#),
    ];

    let errors = inspect(&create_column("topology"), "topology", &cells);

    let located: Vec<(Value, Value)> = errors
        .iter()
        .map(|error| (error["rowNumber"].clone(), error["cell"].clone()))
        .collect();
    assert_eq!(
        located,
        vec![
            (json!(2), json!(r#"{"type":"Topology","objects":{}}"#)),
            (json!(3), json!(r#"{"type":"Topology"}"#)),
        ]
    );
}

#[test]
fn test_topojson_geometry_objects() {
    let cells = [
        Some(r#"{"type":"Point","coordinates":[0,0]}"#),
        Some(r#"{"type":"LineString","arcs":[0,1]}"#),
        Some(r#"{"type":"Polygon","arcs":[[0,1,2]]}"#),
    ];
    assert!(inspect(&create_column("geometry"), "geometry", &cells).is_empty());
}

#[test]
fn test_null_values() {
    let mut column = create_column("topology");
    column.nullable = Some(true);
    let cells = [
        Some(
            r#"{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}"#,
        ),
        None,
    ];
    assert!(inspect(&column, "topology", &cells).is_empty());
}
