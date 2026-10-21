use fairspec_metadata::{
    Creator, CreatorAffiliation, CreatorNameType, DataciteDate, DataciteDescription, Dataset,
    DateType, DescriptionType, Rights, Subject, Title,
};

use crate::plugins::zenodo::actions::resource::from_zenodo::convert_resource_from_zenodo;
use crate::plugins::zenodo::models::record::ZenodoRecord;

pub fn convert_dataset_from_zenodo(zenodo_record: &ZenodoRecord) -> Dataset {
    let metadata = &zenodo_record.metadata;

    let mut dataset = Dataset {
        resources: Some(
            zenodo_record
                .files
                .as_deref()
                .unwrap_or_default()
                .iter()
                .map(convert_resource_from_zenodo)
                .collect(),
        ),
        ..Dataset::default()
    };
    let datacite = &mut dataset.datacite;

    datacite.titles = (!metadata.title.is_empty()).then(|| {
        vec![Title {
            title: metadata.title.clone(),
            ..Title::default()
        }]
    });

    datacite.descriptions = get_text(&metadata.description).map(|description| {
        vec![DataciteDescription {
            description: description.to_string(),
            description_type: DescriptionType::Abstract,
            lang: None,
        }]
    });

    datacite.creators = (!metadata.creators.is_empty()).then(|| {
        metadata
            .creators
            .iter()
            .map(|creator| Creator {
                name: creator.name.clone(),
                name_type: Some(CreatorNameType::Personal),
                affiliation: get_text(&creator.affiliation).map(|affiliation| {
                    vec![CreatorAffiliation {
                        name: affiliation.to_string(),
                        ..CreatorAffiliation::default()
                    }]
                }),
                ..Creator::default()
            })
            .collect()
    });

    datacite.subjects = metadata
        .keywords
        .as_ref()
        .filter(|keywords| !keywords.is_empty())
        .map(|keywords| {
            keywords
                .iter()
                .map(|keyword| Subject {
                    subject: keyword.clone(),
                    ..Subject::default()
                })
                .collect()
        });

    datacite.dates = get_text(&metadata.publication_date).map(|date| {
        vec![DataciteDate {
            date: date.to_string(),
            date_type: DateType::Issued,
            date_information: None,
        }]
    });

    datacite.rights_list = metadata.license.as_ref().map(|license| {
        vec![Rights {
            rights: Some(license.id.clone()),
            ..Rights::default()
        }]
    });

    datacite.doi = metadata.doi.clone();
    datacite.version = metadata.version.clone();

    dataset
}

fn get_text(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|value| !value.is_empty())
}
