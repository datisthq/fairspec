use serde_json::json;

use super::{parse, round_trip};
use crate::models::error::FairspecError;
use crate::models::error::cell::CellError;
use crate::models::error::table::TableError;
use crate::models::report::Report;

#[test]
fn test_dispatch_by_type() {
    let cases = [
        (
            json!({"type": "metadata", "message": "m", "jsonPointer": "/"}),
            "Metadata(MetadataError",
        ),
        (
            json!({"type": "data", "message": "m", "jsonPointer": "/"}),
            "Data(DataError",
        ),
        (
            json!({"type": "resource/missing", "referencingResourceName": "a"}),
            "Resource(Missing(",
        ),
        (
            json!({"type": "resource/type", "expectedResourceType": "table"}),
            "Resource(Type(",
        ),
        (json!({"type": "file/textual"}), "File(Textual("),
        (
            json!({"type": "file/integrity", "hashType": "md5", "expectedHash": "a", "actualHash": "b"}),
            "File(Integrity(",
        ),
        (
            json!({"type": "column/missing", "columnName": "a"}),
            "Table(Column(Missing(",
        ),
        (
            json!({"type": "column/type", "columnName": "a", "expectedColumnType": "local-date-time", "actualColumnType": "string"}),
            "Table(Column(Type(",
        ),
        (
            json!({"type": "row/primaryKey", "rowNumber": 1, "columnNames": ["a"]}),
            "Table(Row(PrimaryKey(",
        ),
        (
            json!({"type": "row/uniqueKey", "rowNumber": 1, "columnNames": ["a"]}),
            "Table(Row(UniqueKey(",
        ),
        (
            json!({"type": "row/check", "rowNumber": 1, "checkName": "c"}),
            "Table(Row(Check(",
        ),
        (
            json!({"type": "foreignKey", "foreignKey": {"columns": ["a"], "reference": {"columns": ["b"]}}, "cells": ["1"]}),
            "Table(ForeignKey(",
        ),
        (
            json!({"type": "table/check", "checkName": "c"}),
            "Table(TableCheck(",
        ),
        (
            json!({"type": "check", "checkName": "c", "message": "m"}),
            "Table(Check(",
        ),
        (
            json!({"type": "cell/type", "columnName": "a", "rowNumber": 1, "cell": "x", "columnType": "integer"}),
            "Table(Cell(Type(",
        ),
        (
            json!({"type": "cell/exclusiveMinimum", "columnName": "a", "rowNumber": 1, "cell": "x", "minimum": "1"}),
            "Table(Cell(ExclusiveMinimum(",
        ),
        (
            json!({"type": "cell/json", "columnName": "a", "rowNumber": 1, "cell": "x", "message": "m", "jsonPointer": "/"}),
            "Table(Cell(Json(",
        ),
    ];
    for (input, prefix) in cases {
        let error: FairspecError = parse(input.clone());
        let debug = format!("{error:?}");
        assert!(debug.starts_with(prefix), "{input} parsed as {debug}");
        assert_eq!(serde_json::to_value(&error).unwrap(), input);
    }
}

#[test]
fn test_multiple_of_keeps_number_kind() {
    for input in [
        json!({"type": "cell/multipleOf", "columnName": "a", "rowNumber": 1, "cell": "3", "multipleOf": 2}),
        json!({"type": "cell/multipleOf", "columnName": "a", "rowNumber": 1, "cell": "3", "multipleOf": 2.5}),
    ] {
        let error: FairspecError = parse(input.clone());
        assert!(matches!(
            error,
            FairspecError::Table(TableError::Cell(CellError::MultipleOf(_)))
        ));
        assert_eq!(serde_json::to_value(&error).unwrap(), input);
    }
}

#[test]
fn test_rejects_unknown_type() {
    for input in [
        json!({"type": "nope"}),
        json!({"type": "data", "jsonPointer": "/"}),
        json!({"type": "metadata", "checkName": "c", "message": "m"}),
    ] {
        assert!(
            serde_json::from_value::<FairspecError>(input.clone()).is_err(),
            "{input}"
        );
    }
}

#[test]
fn test_report_round_trip() {
    let input = json!({"valid": false, "errors": [
        {"type": "cell/missing", "resourceName": "r", "columnName": "a", "rowNumber": 2, "cell": ""}
    ]});
    assert_eq!(round_trip::<Report>(&input), input);
}
