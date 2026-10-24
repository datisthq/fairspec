use fairspec_metadata::{ContributorType, Dataset, DateType, Result};

use crate::plugins::ckan::actions::resource::to_ckan::convert_resource_to_ckan;
use crate::plugins::ckan::models::dataset::NewCkanDataset;
use crate::plugins::ckan::models::tag::CkanTag;

pub fn convert_dataset_to_ckan(dataset: &Dataset) -> Result<NewCkanDataset> {
    let datacite = &dataset.datacite;

    let title = datacite
        .titles
        .as_ref()
        .and_then(|titles| titles.first())
        .map(|title| title.title.clone())
        .filter(|title| !title.is_empty());

    let notes = datacite
        .descriptions
        .as_ref()
        .and_then(|descriptions| descriptions.first())
        .map(|description| description.description.clone())
        .filter(|notes| !notes.is_empty());

    let rights = datacite.rights_list.as_ref().and_then(|list| list.first());

    let author = datacite
        .creators
        .as_ref()
        .and_then(|creators| creators.first())
        .map(|creator| creator.name.clone())
        .filter(|name| !name.is_empty());

    let maintainer = datacite
        .contributors
        .as_deref()
        .unwrap_or_default()
        .iter()
        .find(|contributor| contributor.contributor_type == ContributorType::ContactPerson)
        .map(|contributor| contributor.name.clone())
        .filter(|name| !name.is_empty());

    let resources = dataset
        .resources
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(convert_resource_to_ckan)
        .collect::<Result<_>>()?;

    let tags = datacite
        .subjects
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(|subject| CkanTag {
            id: None,
            name: subject.subject.clone(),
            display_name: Some(subject.subject.clone()),
        })
        .collect();

    let dates = datacite.dates.as_deref().unwrap_or_default();
    let find_date = |date_type: DateType| {
        dates
            .iter()
            .find(|date| date.date_type == date_type)
            .map(|date| date.date.clone())
    };

    Ok(NewCkanDataset {
        resources,
        tags,
        title,
        notes,
        version: datacite.version.clone(),
        license_id: rights.and_then(|rights| rights.rights_identifier.clone()),
        license_title: rights.and_then(|rights| rights.rights.clone()),
        license_url: rights.and_then(|rights| rights.rights_uri.clone()),
        author,
        maintainer,
        metadata_created: find_date(DateType::Created),
        metadata_modified: find_date(DateType::Updated),
    })
}
