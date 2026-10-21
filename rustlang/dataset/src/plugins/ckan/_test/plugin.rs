use crate::plugin::DatasetPlugin;
use crate::plugins::ckan::plugin::{CkanPlugin, get_is_ckan};

#[test]
fn test_loads_from_ckan_url() {
    assert!(get_is_ckan("https://demo.ckan.org/dataset/my-dataset"));
}

#[test]
fn test_returns_none_for_url_without_dataset_path() {
    assert_eq!(
        CkanPlugin
            .load_dataset("https://demo.ckan.org/about")
            .unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_local_paths() {
    assert_eq!(CkanPlugin.load_dataset("/tmp/data").unwrap(), None);
}

#[test]
fn test_returns_none_for_github_urls() {
    assert_eq!(
        CkanPlugin
            .load_dataset("https://github.com/owner/repo")
            .unwrap(),
        None
    );
}

#[test]
fn test_handles_additional_path_segments() {
    assert!(get_is_ckan(
        "https://demo.ckan.org/en_GB/dataset/my-dataset"
    ));
}

#[test]
fn test_handles_query_parameters() {
    assert!(get_is_ckan(
        "https://demo.ckan.org/dataset/my-dataset?page=1"
    ));
}

#[test]
fn test_handles_http_urls() {
    assert!(get_is_ckan("http://demo.ckan.org/dataset/my-dataset"));
}

#[test]
fn test_returns_none_for_zenodo_urls() {
    assert_eq!(
        CkanPlugin
            .load_dataset("https://zenodo.org/records/123456")
            .unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_dataset_in_query_params_only() {
    let result = CkanPlugin.load_dataset("https://example.com/search?name=dataset");
    assert_eq!(result.unwrap(), None);
}
