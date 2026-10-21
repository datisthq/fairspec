use fairspec_metadata::{ContributorType, CreatorNameType, DateType, DescriptionType};
use regex::Regex;
use serde_json::{Value, json};

use crate::plugins::ckan::actions::dataset::from_ckan::convert_dataset_from_ckan;
use crate::plugins::ckan::models::dataset::CkanDataset;

pub(super) fn load_fixture() -> CkanDataset {
    serde_json::from_str(include_str!("fixtures/ckan-dataset.json")).unwrap()
}

#[test]
fn test_converts_ckan_dataset_to_fairspec_dataset() {
    let ckan_dataset = load_fixture();

    let result = convert_dataset_from_ckan(&ckan_dataset);
    let datacite = &result.datacite;

    let titles = datacite.titles.as_ref().unwrap();
    assert_eq!(titles.len(), 1);
    assert_eq!(Some(&titles[0].title), ckan_dataset.title.as_ref());

    let descriptions = datacite.descriptions.as_ref().unwrap();
    assert_eq!(descriptions.len(), 1);
    assert_eq!(
        Some(&descriptions[0].description),
        ckan_dataset.notes.as_ref()
    );
    assert_eq!(descriptions[0].description_type, DescriptionType::Abstract);

    assert_eq!(datacite.version, ckan_dataset.version);

    let dates = datacite.dates.as_ref().unwrap();
    assert_eq!(dates.len(), 2);
    let find_date = |date_type| {
        dates
            .iter()
            .find(|date| date.date_type == date_type)
            .unwrap()
    };
    assert_eq!(
        Some(&find_date(DateType::Created).date),
        ckan_dataset.metadata_created.as_ref()
    );
    assert_eq!(
        Some(&find_date(DateType::Updated).date),
        ckan_dataset.metadata_modified.as_ref()
    );

    let rights_list = datacite.rights_list.as_ref().unwrap();
    assert_eq!(rights_list.len(), 1);
    assert_eq!(rights_list[0].rights, ckan_dataset.license_title);
    assert_eq!(rights_list[0].rights_uri, ckan_dataset.license_url);
    assert_eq!(rights_list[0].rights_identifier, ckan_dataset.license_id);

    let creators = datacite.creators.as_ref().unwrap();
    assert_eq!(creators.len(), 1);
    assert_eq!(Some(&creators[0].name), ckan_dataset.author.as_ref());
    assert_eq!(creators[0].name_type, Some(CreatorNameType::Personal));

    let contributors = datacite.contributors.as_ref().unwrap();
    assert_eq!(contributors.len(), 1);
    assert_eq!(
        Some(&contributors[0].name),
        ckan_dataset.maintainer.as_ref()
    );
    assert_eq!(contributors[0].name_type, Some(CreatorNameType::Personal));
    assert_eq!(
        contributors[0].contributor_type,
        ContributorType::ContactPerson
    );

    let subjects = datacite.subjects.as_ref().unwrap();
    let subject_names: Vec<_> = subjects.iter().map(|subject| &subject.subject).collect();
    let tag_names: Vec<_> = ckan_dataset.tags.iter().map(|tag| &tag.name).collect();
    assert_eq!(subject_names, tag_names);

    let resources = result.resources.as_ref().unwrap();
    assert_eq!(resources.len(), ckan_dataset.resources.len());

    let first_ckan_resource = &ckan_dataset.resources[0];
    let first_resource = &resources[0];
    assert_eq!(
        first_resource.data,
        Some(Value::from(first_ckan_resource.url.as_str()))
    );
    let name = first_resource.name.as_deref().unwrap();
    assert!(
        Regex::new(r"^sample[-_]linked[-_]csv$")
            .unwrap()
            .is_match(name)
    );
    let descriptions = first_resource.datacite.descriptions.as_ref().unwrap();
    assert_eq!(
        Some(&descriptions[0].description),
        first_ckan_resource.description.as_ref()
    );
}

#[test]
fn test_handles_empty_resources_array() {
    let mut ckan_dataset = load_fixture();
    ckan_dataset.resources = vec![];

    let result = convert_dataset_from_ckan(&ckan_dataset);

    assert_eq!(result.resources, Some(vec![]));
}

#[test]
fn test_handles_undefined_optional_properties() {
    let ckan_dataset: CkanDataset =
        serde_json::from_value(json!({"resources": [], "tags": [], "id": "test", "name": "test"}))
            .unwrap();

    let result = convert_dataset_from_ckan(&ckan_dataset);

    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({"resources": []})
    );
}
