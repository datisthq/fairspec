use serde_json::json;

use super::{parse, round_trip};
use crate::models::dataset::Dataset;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::path::PathOr;
use crate::models::resource::Resource;

#[test]
fn test_round_trip_dataset_with_datacite() {
    let input = json!({
        "$schema": "https://fairspec.org/profiles/latest/dataset.json",
        "doi": "10.1234/x",
        "creators": [{"name": "Jane", "nameType": "Personal", "affiliation": [{"name": "Lab"}]}],
        "titles": [{"title": "T", "titleType": "Subtitle", "lang": "en"}],
        "publisher": {"name": "P"},
        "publicationYear": "2024",
        "contributors": [{"name": "Bob", "contributorType": "DataCurator"}],
        "dates": [{"date": "2024", "dateType": "Created"}],
        "types": {"resourceTypeGeneral": "Dataset"},
        "relatedIdentifiers": [{"relatedIdentifier": "10.1/a", "relatedIdentifierType": "DOI", "relationType": "Cites"}],
        "fundingReferences": [{"funderName": "F", "funderIdentifierType": "Crossref Funder ID"}],
        "geoLocations": [{"geoLocationPoint": {"pointLongitude": 1.5, "pointLatitude": -2.0}}],
        "relatedItems": [{"relatedItemType": "Book", "relationType": "IsPartOf", "titles": [{"title": "B"}],
                          "relatedItemIdentifier": {"relatedItemIdentifier": "x", "relatedItemIdentifierType": "arXiv"}}],
        "resources": [
            {"name": "a", "data": "a.csv", "fileDialect": "dialect.json", "tableSchema": "schema.json"},
            {"data": [{"x": 1}], "fileDialect": {"format": "json", "rowType": "object"},
             "tableSchema": {"properties": {"x": {"type": "integer"}}}, "dataSchema": {"type": "array"}},
            {"data": ["a.csv", "b.csv"], "integrity": {"type": "sha256", "hash": "h"}, "textual": true,
             "titles": [{"title": "R"}], "unstable_customMetadata": {"k": 1}}
        ]
    });
    assert_eq!(round_trip::<Dataset>(&input), input);
}

#[test]
fn test_path_or_value() {
    let resource: Resource = parse(json!({"fileDialect": "dialect.json"}));
    assert_eq!(
        resource.file_dialect,
        Some(PathOr::Path("dialect.json".to_string()))
    );

    let resource: Resource = parse(json!({"fileDialect": {"format": "csv"}}));
    assert!(matches!(
        resource.file_dialect,
        Some(PathOr::Value(FileDialect::Csv(_)))
    ));
}

#[test]
fn test_unknown_fields_are_kept() {
    let input = json!({"extra": 1, "resources": [{"data": "a.csv", "extra": 2}]});
    assert_eq!(round_trip::<Dataset>(&input), input);
}

#[test]
fn test_resource_converts_inline_frictionless_schema_and_dialect() {
    let resource: Resource = serde_json::from_value(json!({
        "data": "table.csv",
        "tableSchema": {"fields": [{"name": "id", "type": "integer"}], "primaryKey": "id"},
        "fileDialect": {"delimiter": ";"},
    }))
    .unwrap();
    assert_eq!(
        serde_json::to_value(resource).unwrap(),
        json!({
            "data": "table.csv",
            "fileDialect": {"format": "csv", "delimiter": ";"},
            "tableSchema": {
                "properties": {"id": {"type": ["integer", "null"]}},
                "primaryKey": ["id"],
            },
        })
    );
}
