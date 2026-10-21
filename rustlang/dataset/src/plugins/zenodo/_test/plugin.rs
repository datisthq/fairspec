use crate::plugin::DatasetPlugin;
use crate::plugins::zenodo::plugin::{ZenodoPlugin, get_is_zenodo};

#[test]
fn test_loads_from_zenodo_url() {
    assert!(get_is_zenodo("https://zenodo.org/records/123456"));
}

#[test]
fn test_returns_none_for_non_zenodo_urls() {
    assert_eq!(
        ZenodoPlugin
            .load_dataset("https://example.com/data")
            .unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_local_paths() {
    assert_eq!(ZenodoPlugin.load_dataset("/tmp/data").unwrap(), None);
}

#[test]
fn test_returns_none_for_github_urls() {
    assert_eq!(
        ZenodoPlugin
            .load_dataset("https://github.com/owner/repo")
            .unwrap(),
        None
    );
}

#[test]
fn test_handles_sandbox_zenodo() {
    assert!(get_is_zenodo("https://sandbox.zenodo.org/records/123456"));
}

#[test]
fn test_handles_zenodo_urls_with_paths() {
    assert!(get_is_zenodo("https://zenodo.org/records/123456/files"));
}

#[test]
fn test_handles_zenodo_urls_with_query_params() {
    assert!(get_is_zenodo("https://zenodo.org/records/123456?preview=1"));
}

#[test]
fn test_returns_none_for_http_non_zenodo_urls() {
    assert_eq!(
        ZenodoPlugin
            .load_dataset("http://example.com/data")
            .unwrap(),
        None
    );
}
