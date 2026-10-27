use serde_json::{Value, json};

use crate::actions::file_dialect::validate::validate_file_dialect;
use crate::actions::frictionless::convert::{
    FrictionlessKind, convert_descriptor_from_frictionless, get_is_frictionless_descriptor,
};
use crate::actions::table_schema::validate::validate_table_schema;
use crate::models::descriptor::Descriptor;

fn descriptor(value: Value) -> Descriptor {
    serde_json::from_value(value).unwrap()
}

fn get_pointers(errors: &impl serde::Serialize) -> Vec<Value> {
    let errors = serde_json::to_value(errors).unwrap();
    errors
        .as_array()
        .unwrap()
        .iter()
        .map(|error| error["jsonPointer"].clone())
        .collect()
}

mod get_is_frictionless_descriptor {
    use super::*;

    fn check(value: Value, kind: FrictionlessKind) -> bool {
        get_is_frictionless_descriptor(&descriptor(value), kind)
    }

    #[test]
    fn test_table_schema_with_fields_and_no_properties() {
        let kind = FrictionlessKind::TableSchema;
        assert!(check(json!({"fields": []}), kind));
        assert!(!check(json!({"fields": [], "properties": {}}), kind));
        assert!(!check(json!({"fields": "id"}), kind));
        assert!(!check(json!({"properties": {}}), kind));
    }

    #[test]
    fn test_file_dialect_without_format() {
        let kind = FrictionlessKind::FileDialect;
        assert!(check(json!({"delimiter": ";"}), kind));
        assert!(!check(json!({"format": "csv"}), kind));
        assert!(!check(json!({"format": "csv", "delimiter": ";"}), kind));
        assert!(!check(json!({"title": "dialect"}), kind));
    }

    #[test]
    fn test_dataset_by_profile_schema_or_resource_keys() {
        let kind = FrictionlessKind::Dataset;
        assert!(check(json!({"profile": "data-package"}), kind));
        assert!(check(
            json!({"$schema": "https://datapackage.org/profiles/2.0/datapackage.json"}),
            kind
        ));
        assert!(check(
            json!({"$schema": "https://specs.frictionlessdata.io/schemas/data-package.json"}),
            kind
        ));
        for key in ["path", "schema", "dialect"] {
            assert!(check(json!({"resources": [{key: "x"}]}), kind));
        }
        assert!(!check(json!({"resources": [{"data": "x.csv"}]}), kind));
        assert!(!check(json!({"profile": 1}), kind));
    }

    #[test]
    fn test_fairspec_schema_always_wins() {
        let schema = "https://fairspec.org/profiles/latest/dataset.json";
        assert!(!check(
            json!({"$schema": schema, "profile": "data-package", "resources": [{"path": "x"}]}),
            FrictionlessKind::Dataset
        ));
        assert!(!check(
            json!({"$schema": schema, "fields": []}),
            FrictionlessKind::TableSchema
        ));
        assert!(!check(
            json!({"$schema": schema}),
            FrictionlessKind::FileDialect
        ));
    }
}

mod convert_descriptor_from_frictionless {
    use super::*;

    #[test]
    fn test_reports_model_mismatches_under_root() {
        let (converted, _, errors) = convert_descriptor_from_frictionless(
            &descriptor(json!({"fields": [{"name": "id", "constraints": {"required": "yes"}}]})),
            FrictionlessKind::TableSchema,
            Some("/resources/0"),
        );
        assert!(converted.is_none());
        assert_eq!(
            get_pointers(&errors),
            vec![json!("/resources/0/fields/0/constraints/required")]
        );
    }

    #[test]
    fn test_infers_dialect_format_from_keys() {
        for (dialect, format) in [
            (json!({"property": "items"}), "json"),
            (json!({"itemType": "object"}), "json"),
            (json!({"itemKeys": ["id"]}), "json"),
            (json!({"sheetName": "data"}), "xlsx"),
            (json!({"sheetNumber": 2}), "xlsx"),
            (json!({"table": "items"}), "sqlite"),
            (json!({"delimiter": ";"}), "csv"),
        ] {
            let (converted, _, _) = convert_descriptor_from_frictionless(
                &descriptor(dialect),
                FrictionlessKind::FileDialect,
                None,
            );
            assert_eq!(converted.unwrap()["format"], format);
        }
    }

    #[test]
    fn test_records_resource_names() {
        let (_, mapping, _) = convert_descriptor_from_frictionless(
            &descriptor(json!({"resources": [{"name": "fruit-list", "path": "a.csv"}]})),
            FrictionlessKind::Dataset,
            None,
        );
        assert_eq!(mapping.resource_names["fruit_list"], "fruit-list");
    }
}

mod validate {
    use super::*;

    #[test]
    fn test_validates_frictionless_table_schema() {
        let result = validate_table_schema(
            descriptor(json!({"fields": [{"name": "id", "type": "integer"}], "primaryKey": "id"})),
            None,
        )
        .unwrap();
        assert!(result.valid);
        let table_schema = serde_json::to_value(result.table_schema.unwrap()).unwrap();
        assert_eq!(table_schema["primaryKey"], json!(["id"]));
    }

    #[test]
    fn test_maps_table_schema_errors_to_fields() {
        let result = validate_table_schema(
            descriptor(json!({
                "fields": [{"name": "id"}],
                "primaryKey": ["code"],
                "foreignKeys": [{"fields": "code", "reference": {"fields": "id"}}],
            })),
            None,
        )
        .unwrap();
        assert!(!result.valid);
        assert_eq!(
            get_pointers(&result.errors),
            vec![json!("/primaryKey"), json!("/foreignKeys/0/fields")]
        );
    }

    #[test]
    fn test_maps_table_schema_errors_under_root() {
        let result = validate_table_schema(
            descriptor(json!({"fields": [{"name": "day", "type": "date", "format": "%Q"}]})),
            Some("/resources/0"),
        )
        .unwrap();
        assert!(!result.valid);
        let pointers = get_pointers(&result.errors);
        assert!(!pointers.is_empty());
        assert!(
            pointers
                .iter()
                .all(|pointer| pointer == "/resources/0/fields/0/format")
        );
    }

    #[test]
    fn test_validates_frictionless_file_dialect() {
        let result =
            validate_file_dialect(descriptor(json!({"delimiter": ";", "header": false})), None)
                .unwrap();
        assert!(result.valid);
        assert_eq!(
            serde_json::to_value(result.file_dialect.unwrap()).unwrap(),
            json!({"format": "csv", "delimiter": ";", "headerRows": false})
        );
    }

    #[test]
    fn test_dialect_without_frictionless_keys_needs_format() {
        let result = validate_file_dialect(descriptor(json!({"title": "dialect"})), None).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn test_reports_file_dialect_model_mismatches() {
        let result = validate_file_dialect(descriptor(json!({"header": "no"})), None).unwrap();
        assert!(!result.valid);
        assert_eq!(get_pointers(&result.errors), vec![json!("/header")]);
    }
}
