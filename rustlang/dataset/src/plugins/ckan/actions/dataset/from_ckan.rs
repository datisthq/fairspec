use fairspec_metadata::{
    Contributor, ContributorType, Creator, CreatorNameType, DataciteDate, DataciteDescription,
    Dataset, DateType, DescriptionType, Rights, Subject, Title,
};

use crate::plugins::ckan::actions::resource::from_ckan::convert_resource_from_ckan;
use crate::plugins::ckan::models::dataset::CkanDataset;

pub fn convert_dataset_from_ckan(ckan_dataset: &CkanDataset) -> Dataset {
    let mut dataset = Dataset {
        resources: Some(
            ckan_dataset
                .resources
                .iter()
                .map(convert_resource_from_ckan)
                .collect(),
        ),
        ..Dataset::default()
    };
    let datacite = &mut dataset.datacite;

    datacite.titles = get_text(&ckan_dataset.title).map(|title| {
        vec![Title {
            title: title.to_string(),
            ..Title::default()
        }]
    });

    datacite.descriptions = get_text(&ckan_dataset.notes).map(|notes| {
        vec![DataciteDescription {
            extra: Default::default(),
            description: notes.to_string(),
            description_type: DescriptionType::Abstract,
            lang: None,
        }]
    });

    datacite.version = ckan_dataset.version.clone();

    let license_id = get_text(&ckan_dataset.license_id);
    let license_title = get_text(&ckan_dataset.license_title);
    if license_id.is_some() || license_title.is_some() {
        datacite.rights_list = Some(vec![Rights {
            rights: Some(license_title.or(license_id).unwrap_or_default().to_string()),
            rights_uri: ckan_dataset.license_url.clone(),
            rights_identifier: ckan_dataset.license_id.clone(),
            ..Rights::default()
        }]);
    }

    datacite.creators = get_text(&ckan_dataset.author).map(|author| {
        vec![Creator {
            name: author.to_string(),
            name_type: Some(CreatorNameType::Personal),
            ..Creator::default()
        }]
    });

    datacite.contributors = get_text(&ckan_dataset.maintainer).map(|maintainer| {
        vec![Contributor {
            extra: Default::default(),
            name: maintainer.to_string(),
            name_type: Some(CreatorNameType::Personal),
            given_name: None,
            family_name: None,
            name_identifiers: None,
            affiliation: None,
            lang: None,
            contributor_type: ContributorType::ContactPerson,
        }]
    });

    datacite.subjects = (!ckan_dataset.tags.is_empty()).then(|| {
        ckan_dataset
            .tags
            .iter()
            .map(|tag| Subject {
                subject: tag.name.clone(),
                ..Subject::default()
            })
            .collect()
    });

    let mut dates = Vec::new();
    if let Some(created) = get_text(&ckan_dataset.metadata_created) {
        dates.push(create_date(created, DateType::Created));
    }
    if let Some(modified) = get_text(&ckan_dataset.metadata_modified) {
        dates.push(create_date(modified, DateType::Updated));
    }
    datacite.dates = (!dates.is_empty()).then_some(dates);

    dataset
}

fn get_text(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}

fn create_date(date: &str, date_type: DateType) -> DataciteDate {
    DataciteDate {
        extra: Default::default(),
        date: date.to_string(),
        date_type,
        date_information: None,
    }
}
