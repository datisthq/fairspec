use fairspec_metadata::{
    DataError, ExpectedResourceType, FairspecError, MetadataError, Report, Resource, ResourceError,
    ResourceTypeError, Result, create_report, get_data_paths, inspect_json, resolve_data_schema,
    validate_data_schema,
};

use super::format::DATA_SCHEMA_FORMATS;
use super::load::load_data;

pub fn validate_data(resource: &Resource) -> Result<Report> {
    let Some(data_schema) = resolve_data_schema(resource.data_schema.as_ref())? else {
        return Ok(create_report(Vec::new(), None));
    };

    if get_data_paths(resource).len() > 1 {
        let error = MetadataError {
            message: "a data schema needs single-part data".to_string(),
            json_pointer: "/dataSchema".to_string(),
            ..Default::default()
        };
        return Ok(create_report(vec![FairspecError::Metadata(error)], None));
    }

    let schema_report = validate_data_schema(data_schema.clone(), Some("/dataSchema"))?;
    if !schema_report.valid {
        return Ok(create_report(schema_report.errors, None));
    }

    let Some(data) = load_data(resource)? else {
        let error = ResourceTypeError {
            resource_name: None,
            expected_resource_type: ExpectedResourceType::Data,
            referencing_resource_name: None,
        };
        return Ok(create_report(
            vec![FairspecError::Resource(ResourceError::Type(error))],
            None,
        ));
    };

    let errors = inspect_json(&data, data_schema, None, Some(&DATA_SCHEMA_FORMATS))?
        .into_iter()
        .map(|note| {
            FairspecError::Data(DataError {
                r#type: Default::default(),
                resource_name: None,
                message: note.message,
                json_pointer: note.json_pointer,
            })
        })
        .collect();
    Ok(create_report(errors, None))
}
