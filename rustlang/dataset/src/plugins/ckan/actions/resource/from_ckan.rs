use std::sync::LazyLock;

use fairspec_metadata::{
    DataciteDate, DataciteDescription, DateType, DescriptionType, Descriptor, Integrity,
    IntegrityType, PathOr, Resource, get_file_name,
};
use regex::Regex;
use serde_json::Value;

use crate::plugins::ckan::actions::table_schema::from_ckan::convert_table_schema_from_ckan;
use crate::plugins::ckan::models::resource::CkanResource;

pub fn convert_resource_from_ckan(ckan_resource: &CkanResource) -> Resource {
    let name = get_text(&ckan_resource.name).map(convert_name);

    let descriptions = get_text(&ckan_resource.description).map(|description| {
        vec![DataciteDescription {
            description: description.to_string(),
            description_type: DescriptionType::Abstract,
            lang: None,
        }]
    });

    let sizes = ckan_resource
        .size
        .filter(|size| *size != 0)
        .map(|size| vec![format!("{size} bytes")]);

    let integrity = get_text(&ckan_resource.hash).map(|hash| Integrity {
        r#type: IntegrityType::Md5,
        hash: hash.to_string(),
    });

    let mut dates = Vec::new();
    if let Some(created) = get_text(&ckan_resource.created) {
        dates.push(create_date(created, DateType::Created));
    }
    if let Some(last_modified) = get_text(&ckan_resource.last_modified) {
        dates.push(create_date(last_modified, DateType::Updated));
    }

    let table_schema = ckan_resource
        .schema
        .as_ref()
        .map(|schema| PathOr::Value(convert_table_schema_from_ckan(schema)));

    let mut custom_metadata = Descriptor::new();
    custom_metadata.insert(
        "ckanKey".to_string(),
        get_file_name(&ckan_resource.url).map_or(Value::Null, Value::String),
    );
    custom_metadata.insert(
        "ckanUrl".to_string(),
        Value::String(ckan_resource.url.clone()),
    );
    custom_metadata.insert(
        "ckanId".to_string(),
        Value::String(ckan_resource.id.clone()),
    );

    let mut resource = Resource {
        data: Some(Value::String(ckan_resource.url.clone())),
        name,
        integrity,
        table_schema,
        unstable_custom_metadata: Some(custom_metadata),
        ..Resource::default()
    };
    resource.datacite.descriptions = descriptions;
    resource.datacite.sizes = sizes;
    resource.datacite.dates = (!dates.is_empty()).then_some(dates);
    resource
}

fn get_text(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}

fn create_date(date: &str, date_type: DateType) -> DataciteDate {
    DataciteDate {
        date: date.to_string(),
        date_type,
        date_information: None,
    }
}

fn convert_name(name: &str) -> String {
    static SEPARATORS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s.()/\\,\-]+").unwrap());
    static INVALID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9_]").unwrap());
    static LEADING_DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d)").unwrap());

    let result = SEPARATORS.replace_all(name, "_").to_lowercase();
    let result = INVALID.replace_all(&result, "");
    let result = LEADING_DIGIT.replace(&result, "_$1");
    result.chars().take(100).collect()
}
