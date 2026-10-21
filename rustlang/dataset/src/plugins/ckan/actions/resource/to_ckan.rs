use fairspec_metadata::{DateType, PathOr, Resource, Result};

use crate::plugins::ckan::actions::table_schema::to_ckan::convert_table_schema_to_ckan;
use crate::plugins::ckan::models::resource::NewCkanResource;

pub fn convert_resource_to_ckan(resource: &Resource) -> Result<NewCkanResource> {
    let name = resource.name.clone().filter(|name| !name.is_empty());

    let format = match &resource.file_dialect {
        Some(PathOr::Value(file_dialect)) => file_dialect
            .format()
            .filter(|format| !format.is_empty())
            .map(str::to_uppercase),
        _ => None,
    };

    let description = resource
        .datacite
        .descriptions
        .as_ref()
        .and_then(|descriptions| descriptions.first())
        .map(|description| description.description.clone())
        .filter(|description| !description.is_empty());

    let hash = resource
        .integrity
        .as_ref()
        .map(|integrity| integrity.hash.clone())
        .filter(|hash| !hash.is_empty());

    let dates = resource.datacite.dates.as_deref().unwrap_or_default();
    let find_date = |date_type: DateType| {
        dates
            .iter()
            .find(|date| date.date_type == date_type)
            .map(|date| date.date.clone())
    };

    let schema = match &resource.table_schema {
        Some(PathOr::Value(table_schema)) => Some(convert_table_schema_to_ckan(table_schema)?),
        _ => None,
    };

    Ok(NewCkanResource {
        name,
        format,
        description,
        hash,
        created: find_date(DateType::Created),
        last_modified: find_date(DateType::Updated),
        schema,
    })
}
