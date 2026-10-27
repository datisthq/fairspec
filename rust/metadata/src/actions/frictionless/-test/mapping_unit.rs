use std::collections::HashMap;

use serde_json::{Value, json};

use crate::actions::frictionless::mapping::{
    FrictionlessMapping, FrictionlessPointer, FrictionlessRecorder, map_frictionless_errors,
};
use crate::models::error::error::FairspecError;

fn pointer(fairspec: &str, frictionless: &str, verbatim: bool) -> FrictionlessPointer {
    FrictionlessPointer {
        fairspec: fairspec.to_string(),
        frictionless: frictionless.to_string(),
        verbatim,
    }
}

fn map_error(error: Value, mapping: &FrictionlessMapping) -> Value {
    let error: FairspecError = serde_json::from_value(error).unwrap();
    let errors = map_frictionless_errors(vec![error], mapping);
    serde_json::to_value(&errors[0]).unwrap()
}

fn map_pointer(json_pointer: &str, mapping: &FrictionlessMapping) -> Value {
    let error = json!({"type": "metadata", "message": "x", "jsonPointer": json_pointer});
    map_error(error, mapping)["jsonPointer"].clone()
}

#[test]
fn test_recorder_nests_prefixes() {
    let mut pointers = Vec::new();
    let mut record = FrictionlessRecorder::new(&mut pointers, "/root", "/base");
    record.record("/a", "/b");
    record.nest("/c", "/d").record_verbatim("/e", "/f");
    assert_eq!(
        pointers,
        vec![
            pointer("/root/a", "/base/b", false),
            pointer("/root/c/e", "/base/d/f", true),
        ]
    );
}

#[test]
fn test_default_recorder_records_nothing() {
    let mut record = FrictionlessRecorder::default();
    record.record("/a", "/b");
    record.nest("/c", "/d").record("/e", "/f");
}

#[test]
fn test_maps_longest_segment_prefix() {
    let mapping = FrictionlessMapping {
        pointers: vec![
            pointer("/properties", "/fields", false),
            pointer("/properties/id", "/fields/0", false),
            pointer("/properties/id/enum", "/fields/0/constraints/enum", true),
        ],
        ..Default::default()
    };
    assert_eq!(map_pointer("/properties/id", &mapping), "/fields/0");
    assert_eq!(map_pointer("/properties/id/type", &mapping), "/fields/0");
    assert_eq!(
        map_pointer("/properties/id/enum/2", &mapping),
        "/fields/0/constraints/enum/2"
    );
    assert_eq!(map_pointer("/properties/idx", &mapping), "/fields");
    assert_eq!(map_pointer("/primaryKey", &mapping), "/primaryKey");
}

#[test]
fn test_first_of_equal_prefixes_wins() {
    let mapping = FrictionlessMapping {
        pointers: vec![
            pointer("/a", "/first", false),
            pointer("/a", "/second", false),
        ],
        ..Default::default()
    };
    assert_eq!(map_pointer("/a/b", &mapping), "/first");
}

#[test]
fn test_maps_resource_names_and_foreign_key_references() {
    let mapping = FrictionlessMapping {
        resource_names: HashMap::from([("fruit_kinds".to_string(), "fruit-kinds".to_string())]),
        ..Default::default()
    };

    let error = map_error(
        json!({
            "type": "foreignKey",
            "foreignKey": {
                "columns": ["kind"],
                "reference": {"resource": "fruit_kinds", "columns": ["kind"]},
            },
            "cells": ["plum"],
            "resourceName": "fruit_kinds",
        }),
        &mapping,
    );
    assert_eq!(error["resourceName"], "fruit-kinds");
    assert_eq!(error["foreignKey"]["reference"]["resource"], "fruit-kinds");

    let error = map_error(
        json!({"type": "row/primaryKey", "columnNames": ["id"], "rowNumber": 2, "resourceName": "other"}),
        &mapping,
    );
    assert_eq!(error["resourceName"], "other");
}
