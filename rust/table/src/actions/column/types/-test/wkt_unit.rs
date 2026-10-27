use fairspec_metadata::WktColumn;
use polars::prelude::*;
use serde_json::json;

use crate::actions::column::types::wkt::inspect_wkt_column;

fn get_is_valid(cell: &str) -> bool {
    let column: WktColumn = serde_json::from_value(
        json!({"name": "name", "property": {"type": "string", "format": "wkt"}}),
    )
    .unwrap();
    let table = df!("name" => [cell]).unwrap().lazy();
    inspect_wkt_column(&column, table, None).unwrap().is_empty()
}

#[test]
fn test_wkt_validation() {
    let cases = [
        ("POINT (0 0)", true),
        ("MULTIPOINT ((0 0), (1 1))", true),
        ("ghijkl", false),
        ("0x1234", false),
        ("hello world", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell}");
    }
}

// Expected verdicts are shapely 2.1 (GEOS 3.13) `from_wkt` raising or not
#[test]
fn test_wkt_validation_matches_shapely() {
    let cases = [
        ("POINT(0 0)", true),
        ("point (0 0)", true),
        ("POINT EMPTY", true),
        ("POINT Z (1 2 3)", true),
        ("POINT M (1 2 3)", true),
        ("POINT ZM (1 2 3 4)", true),
        ("POINT (1 2 3)", true),
        ("POINT (1 2 3 4)", true),
        ("POINT (1 2 3 4 5)", false),
        ("POINT (1)", false),
        ("POINT ()", false),
        ("POINT", false),
        ("", false),
        ("  POINT (0 0)  ", true),
        ("POINT (0 0)\n", true),
        ("POINT (0 0) extra", false),
        ("SRID=4326;POINT (0 0)", false),
        ("LINESTRING (0 0)", false),
        ("LINESTRING (0 0, 1 1)", true),
        ("LINESTRING (0 0, 0 0)", true),
        ("LINESTRING EMPTY", true),
        ("LINESTRING (0 0, 1 1 1)", false),
        ("LINESTRING (0 0, 1 1, )", false),
        ("LINESTRING ((0 0, 1 1))", false),
        ("POLYGON ((0 0, 1 0, 1 1, 0 0))", true),
        ("POLYGON ((0 0, 1 0, 0 0))", true),
        ("POLYGON ((0 0, 1 0, 1 1))", false),
        ("POLYGON ((0 0, 0 0))", false),
        ("POLYGON ((0 0))", false),
        ("POLYGON ((nan 0, 1 0, nan 0))", false),
        ("POLYGON ((0 0, 1 0, 1 1, 0 0), EMPTY)", true),
        ("POLYGON (EMPTY)", true),
        ("POLYGON Z ((0 0 0, 1 0 0, 1 1 0, 0 0 1))", true),
        (
            "POLYGON ((0 0, 1 0, 1 1, 0 0), (0 0 0, 1 0 0, 1 1 0, 0 0 0))",
            false,
        ),
        ("MULTIPOINT (0 0, 1 1)", true),
        ("MULTIPOINT EMPTY", true),
        ("MULTIPOINT ((0 0), EMPTY)", true),
        ("MULTIPOINT ((0 0) , 1 1)", false),
        ("MULTIPOINT (0 0, (1 1))", false),
        ("MULTIPOINT ((0 0 0), (1 1))", false),
        ("MULTIPOINT Z (0 0 0, 1 1 1)", true),
        ("MULTIPOINTZ ((1 2 3))", true),
        ("MULTILINESTRING ((0 0, 1 1), (2 2, 3 3))", true),
        ("MULTILINESTRING (EMPTY, (0 0, 1 1))", true),
        ("MULTIPOLYGON (((0 0, 1 0, 1 1, 0 0)))", true),
        ("MULTIPOLYGON ((EMPTY))", true),
        ("MULTIPOLYGON (EMPTY)", true),
        (
            "GEOMETRYCOLLECTION (POINT (0 0), LINESTRING (0 0, 1 1))",
            true,
        ),
        ("GEOMETRYCOLLECTION EMPTY", true),
        ("GEOMETRYCOLLECTION (EMPTY)", false),
        ("GEOMETRYCOLLECTION (POINT (0 0), POINT (1 1 1))", true),
        ("GEOMETRYCOLLECTION (POINT Z (0 0 0), POINT (1 1))", true),
        ("GEOMETRYCOLLECTION Z (POINT (0 0 0))", true),
        ("GEOMETRYCOLLECTION Z (POINT (0 0))", false),
        ("GEOMETRYCOLLECTION Z (POINT M (0 0 0))", false),
        ("GEOMETRYCOLLECTION M (POINT (0 0 0))", false),
        ("GEOMETRYCOLLECTION Z (POINT EMPTY)", false),
        ("GEOMETRYCOLLECTION ZM (POINT (0 0 0 0))", true),
        ("GEOMETRYCOLLECTION (POINT (0 0)", false),
        ("LINEARRING (0 0, 1 0, 1 1, 0 0)", true),
        ("LINEARRING (0 0, 1 1, 0 0)", true),
        ("LINEARRING (0 0, 0 0)", false),
        ("LINEARRING (0 0, 1 1)", false),
        ("CIRCULARSTRING (0 0, 1 1, 2 0)", false),
        ("TRIANGLE ((0 0, 1 0, 0 1, 0 0))", false),
        ("POINT (nan nan)", true),
        ("POINT (inf 1)", true),
        ("POINT (INF 1)", true),
        ("POINT (infinity 1)", true),
        ("POINT (1e3 2E-2)", true),
        ("POINT (1e400 0)", true),
        ("POINT (+1 -2)", true),
        ("POINT (.5 1.)", true),
        ("POINT (0x10 1)", true),
        ("POINT (0x1p3 1)", true),
        ("POINT (00012 1)", true),
        ("POINT (1,2)", false),
        ("POINT (1e 0)", false),
        ("POINT (- 1)", false),
        ("POINT (1.5.5 1)", false),
        ("POINT (1d 1)", false),
        ("POINT EMPTY EMPTY", false),
        ("POINT ZM EMPTY", true),
        ("POINT ZZ (1 2)", false),
        ("POINT MZ (1 2 3 4)", false),
        ("POINTZM(1 2 3 4)", true),
        ("POINT Z(1 2 3)", true),
        ("POINT Z (1 2)", false),
        ("POINT Z (1 2 3 4)", false),
        ("POINT(1\t2)", true),
        ("POINT(0 0),", false),
        ("POINT (0 0", false),
        ("POINT((0 0))", false),
        ("EMPTY", false),
    ];
    for (cell, valid) in cases {
        assert_eq!(get_is_valid(cell), valid, "{cell:?}");
    }
}
