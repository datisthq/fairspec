use fairspec_metadata::{
    Contributor, ContributorType, Creator, CreatorNameType, DataciteDate, DataciteDescription,
    Dataset, DateType, DescriptionType, Rights, Subject, Title,
};

use crate::plugins::github::actions::resource::from_github::convert_resource_from_github;
use crate::plugins::github::models::owner::GithubOwnerType;
use crate::plugins::github::models::repository::GithubRepository;

pub fn convert_dataset_from_github(repository: &GithubRepository) -> Dataset {
    let files = repository.files.as_deref().unwrap_or_default();
    let default_branch = Some(repository.default_branch.as_str())
        .filter(|branch| !branch.is_empty())
        .unwrap_or("main");
    let resources = files
        .iter()
        .filter(|file| !file.path.starts_with('.') && file.r#type == "blob")
        .map(|file| convert_resource_from_github(file, default_branch))
        .collect();

    let mut dataset = Dataset {
        resources: Some(resources),
        ..Dataset::default()
    };
    let datacite = &mut dataset.datacite;

    datacite.titles = (!repository.full_name.is_empty()).then(|| {
        vec![Title {
            title: repository.full_name.clone(),
            ..Title::default()
        }]
    });

    datacite.descriptions = repository
        .description
        .as_ref()
        .filter(|description| !description.is_empty())
        .map(|description| {
            vec![DataciteDescription {
                description: description.clone(),
                description_type: DescriptionType::Abstract,
                lang: None,
            }]
        });

    datacite.rights_list = repository.license.as_ref().map(|license| {
        vec![Rights {
            rights: Some(license.name.clone()),
            rights_uri: license.url.clone(),
            rights_identifier: Some(
                license
                    .spdx_id
                    .clone()
                    .filter(|spdx_id| !spdx_id.is_empty())
                    .unwrap_or_else(|| license.key.clone()),
            ),
            rights_identifier_scheme: Some("SPDX".to_string()),
            ..Rights::default()
        }]
    });

    let owner = &repository.owner;
    if owner.r#type == GithubOwnerType::Organization {
        datacite.contributors = Some(vec![Contributor {
            name: owner.login.clone(),
            name_type: Some(CreatorNameType::Organizational),
            given_name: None,
            family_name: None,
            name_identifiers: None,
            affiliation: None,
            lang: None,
            contributor_type: ContributorType::HostingInstitution,
        }]);
    } else {
        datacite.creators = Some(vec![Creator {
            name: owner.login.clone(),
            name_type: Some(CreatorNameType::Personal),
            ..Creator::default()
        }]);
    }

    datacite.subjects = (!repository.topics.is_empty()).then(|| {
        repository
            .topics
            .iter()
            .map(|topic| Subject {
                subject: topic.clone(),
                ..Subject::default()
            })
            .collect()
    });

    datacite.dates = (!repository.created_at.is_empty()).then(|| {
        vec![DataciteDate {
            date: repository.created_at.clone(),
            date_type: DateType::Created,
            date_information: None,
        }]
    });

    dataset
}
