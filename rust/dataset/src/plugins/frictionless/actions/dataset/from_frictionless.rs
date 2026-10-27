use fairspec_metadata::Descriptor;
use regex::Regex;
use serde_json::{Value, json};
use std::sync::LazyLock;

use crate::plugins::frictionless::actions::resource::from_frictionless::convert_resource_from_frictionless;
use crate::plugins::frictionless::actions::rights::from_frictionless::convert_rights_from_frictionless;
use crate::plugins::frictionless::helpers::value::{get_list, get_non_empty_list, get_text};

static DOI_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^https?://(dx\.)?doi\.org/").expect("valid regex"));
static DOI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^10[.][0-9]{4,9}[/][^\s]+$").expect("valid regex"));

/// Convert a Frictionless Data Package to a Fairspec Dataset descriptor
/// Its resources' `schema` and `dialect` must already be objects, not paths
pub fn convert_dataset_from_frictionless(frictionless_package: &Descriptor) -> Descriptor {
    let mut dataset = Descriptor::new();

    if let Some(doi) = frictionless_package
        .get("id")
        .and_then(Value::as_str)
        .and_then(convert_doi)
    {
        dataset.insert("doi".to_string(), doi.into());
    }

    let mut creators = Vec::new();
    let mut contributors = Vec::new();
    for contributor in get_list(frictionless_package, "contributors")
        .into_iter()
        .flatten()
    {
        let Some(contributor) = contributor.as_object() else {
            continue;
        };
        let Some(mut person) = convert_person(contributor) else {
            continue;
        };

        let roles: Vec<&str> = match contributor.get("roles").and_then(Value::as_array) {
            Some(roles) => roles.iter().filter_map(Value::as_str).collect(),
            None => get_text(contributor, "role").into_iter().collect(),
        };
        if roles.contains(&"author") || roles.contains(&"creator") {
            creators.push(Value::Object(person));
        } else {
            let contributor_type = roles
                .iter()
                .find_map(|role| get_contributor_type(role))
                .unwrap_or("Other");
            person.insert("contributorType".to_string(), contributor_type.into());
            contributors.push(Value::Object(person));
        }
    }

    if !creators.is_empty() {
        dataset.insert("creators".to_string(), creators.into());
    }

    if let Some(title) = get_text(frictionless_package, "title") {
        dataset.insert("titles".to_string(), json!([{"title": title}]));
    }

    if let Some(keywords) = get_non_empty_list(frictionless_package, "keywords") {
        let subjects: Vec<Value> = keywords
            .iter()
            .map(|subject| json!({"subject": subject}))
            .collect();
        dataset.insert("subjects".to_string(), subjects.into());
    }

    if !contributors.is_empty() {
        dataset.insert("contributors".to_string(), contributors.into());
    }

    if let Some(created) = get_text(frictionless_package, "created") {
        dataset.insert(
            "dates".to_string(),
            json!([{"date": created, "dateType": "Created"}]),
        );
    }

    let mut related_identifiers = Vec::new();
    if let Some(homepage) = get_text(frictionless_package, "homepage") {
        related_identifiers.push(create_related_identifier(homepage, "IsDocumentedBy"));
    }
    for source in get_list(frictionless_package, "sources")
        .into_iter()
        .flatten()
    {
        let path = source
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if path.starts_with("http://") || path.starts_with("https://") {
            related_identifiers.push(create_related_identifier(path, "IsDerivedFrom"));
        }
    }
    if !related_identifiers.is_empty() {
        dataset.insert("relatedIdentifiers".to_string(), related_identifiers.into());
    }

    if let Some(version) = get_text(frictionless_package, "version") {
        dataset.insert("version".to_string(), version.into());
    }

    if let Some(licenses) = get_non_empty_list(frictionless_package, "licenses") {
        let rights_list = convert_rights_from_frictionless(licenses);
        dataset.insert("rightsList".to_string(), rights_list.into());
    }

    if let Some(description) = get_text(frictionless_package, "description") {
        dataset.insert(
            "descriptions".to_string(),
            json!([{"description": description, "descriptionType": "Abstract"}]),
        );
    }

    let resources: Vec<Value> = get_list(frictionless_package, "resources")
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
        .map(|resource| Value::Object(convert_resource_from_frictionless(resource)))
        .collect();
    dataset.insert("resources".to_string(), resources.into());

    dataset
}

fn get_contributor_type(role: &str) -> Option<&'static str> {
    match role {
        "maintainer" => Some("ContactPerson"),
        "wrangler" => Some("DataCurator"),
        "publisher" => Some("Distributor"),
        _ => None,
    }
}

fn convert_person(contributor: &Descriptor) -> Option<Descriptor> {
    let given_name = get_text(contributor, "givenName");
    let family_name = get_text(contributor, "familyName");
    let name = match contributor.get("title").and_then(Value::as_str) {
        Some(title) => title.to_string(),
        None => [given_name, family_name]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" "),
    };
    if name.is_empty() {
        return None;
    }

    let mut person = Descriptor::new();
    person.insert("name".to_string(), name.into());
    if given_name.is_some() || family_name.is_some() {
        person.insert("nameType".to_string(), "Personal".into());
    }
    if let Some(given_name) = given_name {
        person.insert("givenName".to_string(), given_name.into());
    }
    if let Some(family_name) = family_name {
        person.insert("familyName".to_string(), family_name.into());
    }
    if let Some(organization) = get_text(contributor, "organization") {
        person.insert("affiliation".to_string(), json!([{"name": organization}]));
    }
    Some(person)
}

fn create_related_identifier(identifier: &str, relation_type: &str) -> Value {
    json!({
        "relatedIdentifier": identifier,
        "relatedIdentifierType": "URL",
        "relationType": relation_type,
    })
}

fn convert_doi(id: &str) -> Option<String> {
    let doi = DOI_PREFIX.replace(id, "");
    DOI.is_match(&doi).then(|| doi.into_owned())
}
