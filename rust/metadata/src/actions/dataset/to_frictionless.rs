use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use serde_json::{Value, json};

use crate::actions::frictionless::value::{get_list, get_non_empty_list, get_text};
use crate::actions::resource::to_frictionless::{convert_resource_to_frictionless, get_first_text};
use crate::actions::rights::to_frictionless::convert_rights_to_frictionless;

/// Convert a denormalized Fairspec Dataset to a Frictionless Data Package descriptor
/// Its resources' `tableSchema` and `fileDialect` must already be objects, not paths
pub fn convert_dataset_to_frictionless(dataset: &Descriptor) -> Result<Descriptor> {
    let mut frictionless_package = Descriptor::new();
    frictionless_package.insert(
        "$schema".to_string(),
        "https://datapackage.org/profiles/2.0/datapackage.json".into(),
    );

    if let Some(doi) = get_text(dataset, "doi") {
        frictionless_package.insert("id".to_string(), format!("https://doi.org/{doi}").into());
    }

    if let Some(title) = get_first_text(dataset, "titles", "title") {
        frictionless_package.insert("title".to_string(), title.into());
    }

    if let Some(description) = get_first_text(dataset, "descriptions", "description") {
        frictionless_package.insert("description".to_string(), description.into());
    }

    let related_identifiers = get_list(dataset, "relatedIdentifiers")
        .cloned()
        .unwrap_or_default();
    if let Some(homepage) = related_identifiers
        .iter()
        .find(|identifier| identifier["relationType"] == "IsDocumentedBy")
    {
        frictionless_package.insert(
            "homepage".to_string(),
            homepage["relatedIdentifier"].clone(),
        );
    }

    if let Some(version) = get_text(dataset, "version") {
        frictionless_package.insert("version".to_string(), version.into());
    }

    if let Some(created) = get_list(dataset, "dates")
        .into_iter()
        .flatten()
        .find(|date| date["dateType"] == "Created")
    {
        frictionless_package.insert("created".to_string(), created["date"].clone());
    }

    if let Some(subjects) = get_non_empty_list(dataset, "subjects") {
        let keywords: Vec<Value> = subjects
            .iter()
            .map(|subject| subject["subject"].clone())
            .collect();
        frictionless_package.insert("keywords".to_string(), keywords.into());
    }

    if let Some(rights_list) = get_non_empty_list(dataset, "rightsList") {
        let licenses = convert_rights_to_frictionless(rights_list);
        frictionless_package.insert("licenses".to_string(), licenses.into());
    }

    let creators = get_list(dataset, "creators")
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
        .map(|creator| convert_person(creator, "author"));
    let contributors = get_list(dataset, "contributors")
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
        .map(|contributor| {
            let role = contributor
                .get("contributorType")
                .and_then(Value::as_str)
                .and_then(get_role)
                .unwrap_or("contributor");
            convert_person(contributor, role)
        });
    let contributors: Vec<Value> = creators.chain(contributors).collect();
    if !contributors.is_empty() {
        frictionless_package.insert("contributors".to_string(), contributors.into());
    }

    let sources: Vec<Value> = related_identifiers
        .iter()
        .filter(|identifier| identifier["relationType"] == "IsDerivedFrom")
        .map(|identifier| {
            let path = &identifier["relatedIdentifier"];
            json!({"title": path, "path": path})
        })
        .collect();
    if !sources.is_empty() {
        frictionless_package.insert("sources".to_string(), sources.into());
    }

    let resources = get_list(dataset, "resources")
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
        .enumerate()
        .map(|(index, resource)| {
            convert_resource_to_frictionless(resource, index + 1).map(Value::Object)
        })
        .collect::<Result<Vec<_>>>()?;
    frictionless_package.insert("resources".to_string(), resources.into());

    Ok(frictionless_package)
}

fn get_role(contributor_type: &str) -> Option<&'static str> {
    match contributor_type {
        "ContactPerson" => Some("maintainer"),
        "DataCurator" => Some("wrangler"),
        "Distributor" => Some("publisher"),
        _ => None,
    }
}

fn convert_person(person: &Descriptor, role: &str) -> Value {
    let mut contributor = Descriptor::new();
    if let Some(name) = person.get("name") {
        contributor.insert("title".to_string(), name.clone());
    }
    for key in ["givenName", "familyName"] {
        if let Some(text) = get_text(person, key) {
            contributor.insert(key.to_string(), text.into());
        }
    }
    let organization = get_list(person, "affiliation")
        .and_then(|affiliation| affiliation.first())
        .and_then(Value::as_object)
        .and_then(|affiliation| get_text(affiliation, "name"));
    if let Some(organization) = organization {
        contributor.insert("organization".to_string(), organization.into());
    }
    contributor.insert("roles".to_string(), json!([role]));
    Value::Object(contributor)
}
