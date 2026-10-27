use fairspec_metadata::Dataset;
use serde_json::json;

use super::from_ckan_unit::load_fixture;
use crate::plugins::ckan::actions::dataset::from_ckan::convert_dataset_from_ckan;
use crate::plugins::ckan::actions::dataset::to_ckan::convert_dataset_to_ckan;
use crate::plugins::ckan::models::dataset::NewCkanDataset;

#[test]
fn test_converts_fairspec_dataset_to_ckan_dataset() {
    let dataset: Dataset = serde_json::from_value(json!({
        "titles": [{"title": "Test Package"}],
        "descriptions": [{"description": "This is a test package", "descriptionType": "Abstract"}],
        "version": "1.0.0",
        "rightsList": [{
            "rights": "Creative Commons Attribution",
            "rightsUri": "http://www.opendefinition.org/licenses/cc-by",
            "rightsIdentifier": "cc-by",
        }],
        "creators": [{"name": "Test Author", "nameType": "Personal"}],
        "contributors": [{
            "name": "Test Maintainer",
            "nameType": "Personal",
            "contributorType": "ContactPerson",
        }],
        "subjects": [{"subject": "test"}, {"subject": "sample"}, {"subject": "data"}],
        "dates": [
            {"date": "2023-01-01T00:00:00Z", "dateType": "Created"},
            {"date": "2023-01-02T00:00:00Z", "dateType": "Updated"},
        ],
        "resources": [{
            "name": "test_resource",
            "data": "https://example.com/data.csv",
            "fileDialect": {"format": "csv"},
            "descriptions": [{"description": "Test resource", "descriptionType": "Abstract"}],
            "integrity": {"type": "md5", "hash": "1234567890abcdef"},
        }],
    }))
    .unwrap();

    let result = convert_dataset_to_ckan(&dataset).unwrap();

    assert_eq!(result.title.as_deref(), Some("Test Package"));
    assert_eq!(result.notes.as_deref(), Some("This is a test package"));
    assert_eq!(result.version.as_deref(), Some("1.0.0"));

    assert_eq!(result.license_id.as_deref(), Some("cc-by"));
    assert_eq!(
        result.license_title.as_deref(),
        Some("Creative Commons Attribution")
    );
    assert_eq!(
        result.license_url.as_deref(),
        Some("http://www.opendefinition.org/licenses/cc-by")
    );

    assert_eq!(result.author.as_deref(), Some("Test Author"));
    assert_eq!(result.maintainer.as_deref(), Some("Test Maintainer"));

    assert_eq!(result.tags.len(), 3);
    for (tag, subject) in result.tags.iter().zip(["test", "sample", "data"]) {
        assert_eq!(tag.name, subject);
        assert_eq!(tag.display_name.as_deref(), Some(subject));
    }

    assert_eq!(
        result.metadata_created.as_deref(),
        Some("2023-01-01T00:00:00Z")
    );
    assert_eq!(
        result.metadata_modified.as_deref(),
        Some("2023-01-02T00:00:00Z")
    );

    assert_eq!(result.resources.len(), 1);
    assert_eq!(result.resources[0].name.as_deref(), Some("test_resource"));
    assert_eq!(
        result.resources[0].description.as_deref(),
        Some("Test resource")
    );
    assert_eq!(
        result.resources[0].hash.as_deref(),
        Some("1234567890abcdef")
    );
}

#[test]
fn test_handles_empty_resources_array() {
    let dataset: Dataset = serde_json::from_value(json!({"resources": []})).unwrap();
    assert_eq!(convert_dataset_to_ckan(&dataset).unwrap().resources, vec![]);
}

#[test]
fn test_handles_undefined_optional_properties() {
    let dataset: Dataset = serde_json::from_value(json!({"resources": []})).unwrap();
    assert_eq!(
        convert_dataset_to_ckan(&dataset).unwrap(),
        NewCkanDataset::default()
    );
}

#[test]
fn test_round_trip_ckan_to_dataset_to_ckan() {
    let original = load_fixture();

    let dataset = convert_dataset_from_ckan(&original);
    let result = convert_dataset_to_ckan(&dataset).unwrap();

    assert_eq!(result.title, original.title);
    assert_eq!(result.notes, original.notes);
    assert_eq!(result.version, original.version);

    assert_eq!(result.license_id, original.license_id);
    assert_eq!(result.license_title, original.license_title);
    assert_eq!(result.license_url, original.license_url);

    assert_eq!(result.author, original.author);
    assert_eq!(result.maintainer, original.maintainer);

    assert_eq!(result.metadata_created, original.metadata_created);
    assert_eq!(result.metadata_modified, original.metadata_modified);

    assert!(!result.resources.is_empty());

    assert_eq!(result.tags.len(), original.tags.len());
    for original_tag in &original.tags {
        assert!(result.tags.iter().any(|tag| tag.name == original_tag.name));
    }
}
