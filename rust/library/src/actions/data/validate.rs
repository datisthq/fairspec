use fairspec_metadata::{
    DataError, ExpectedResourceType, FairspecError, Report, Resource, ResourceError,
    ResourceTypeError, Result, create_report, inspect_json, resolve_data_schema,
};

use super::load::load_data;

pub fn validate_data(resource: &Resource) -> Result<Report> {
    let Some(data_schema) = resolve_data_schema(resource.data_schema.as_ref())? else {
        return Ok(create_report(Vec::new(), None));
    };

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

    let errors = inspect_json(&data, data_schema, None)?
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
