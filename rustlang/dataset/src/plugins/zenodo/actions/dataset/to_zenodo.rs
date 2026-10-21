use fairspec_metadata::{Dataset, DateType, Descriptor};
use serde_json::{Value, json};

pub fn convert_dataset_to_zenodo(dataset: &Dataset) -> Descriptor {
    let datacite = &dataset.datacite;
    let mut metadata = Descriptor::new();
    metadata.insert("upload_type".to_string(), Value::from("dataset"));

    let title = datacite
        .titles
        .as_ref()
        .and_then(|titles| titles.first())
        .map(|title| title.title.as_str())
        .filter(|title| !title.is_empty());
    if let Some(title) = title {
        metadata.insert("title".to_string(), Value::from(title));
    }

    let description = datacite
        .descriptions
        .as_ref()
        .and_then(|descriptions| descriptions.first())
        .map(|description| description.description.as_str())
        .filter(|description| !description.is_empty());
    let description = description
        .or(title)
        .unwrap_or("Dataset created with fairspec");
    metadata.insert("description".to_string(), Value::from(description));

    if let Some(version) = datacite
        .version
        .as_ref()
        .filter(|version| !version.is_empty())
    {
        metadata.insert("version".to_string(), Value::from(version.as_str()));
    }

    let rights = datacite
        .rights_list
        .as_ref()
        .and_then(|rights_list| rights_list.first())
        .and_then(|rights| rights.rights.as_ref())
        .filter(|rights| !rights.is_empty());
    if let Some(rights) = rights {
        metadata.insert("license".to_string(), Value::from(rights.as_str()));
    }

    let creators = datacite.creators.as_deref().unwrap_or_default();
    let creators = match creators.is_empty() {
        true => vec![json!({"name": "Unknown Author", "affiliation": "Unknown Affiliation"})],
        false => creators
            .iter()
            .map(|creator| {
                let mut zenodo_creator = json!({"name": creator.name});
                let affiliation = creator
                    .affiliation
                    .as_ref()
                    .and_then(|affiliations| affiliations.first())
                    .map(|affiliation| affiliation.name.as_str())
                    .filter(|name| !name.is_empty());
                if let Some(affiliation) = affiliation {
                    zenodo_creator["affiliation"] = Value::from(affiliation);
                }
                zenodo_creator
            })
            .collect(),
    };
    metadata.insert("creators".to_string(), Value::Array(creators));

    let subjects = datacite.subjects.as_deref().unwrap_or_default();
    if !subjects.is_empty() {
        let keywords = subjects
            .iter()
            .map(|subject| Value::from(subject.subject.as_str()))
            .collect();
        metadata.insert("keywords".to_string(), Value::Array(keywords));
    }

    let issued = datacite
        .dates
        .as_deref()
        .unwrap_or_default()
        .iter()
        .find(|date| date.date_type == DateType::Issued);
    if let Some(issued) = issued {
        metadata.insert(
            "publication_date".to_string(),
            Value::from(issued.date.as_str()),
        );
    }

    if let Some(doi) = datacite.doi.as_ref().filter(|doi| !doi.is_empty()) {
        metadata.insert("doi".to_string(), Value::from(doi.as_str()));
    }

    let mut record = Descriptor::new();
    record.insert("metadata".to_string(), Value::Object(metadata));
    record
}
