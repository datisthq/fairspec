use crate::plugin::DatasetPlugin;
use crate::plugins::github::plugin::{GithubPlugin, get_is_github};

#[test]
fn test_loads_from_github_url() {
    assert!(get_is_github("https://github.com/owner/repo"));
}

#[test]
fn test_returns_none_for_non_github_urls() {
    assert_eq!(
        GithubPlugin
            .load_dataset("https://example.com/data")
            .unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_local_paths() {
    assert_eq!(GithubPlugin.load_dataset("/tmp/data").unwrap(), None);
}

#[test]
fn test_returns_none_for_zenodo_urls() {
    assert_eq!(
        GithubPlugin
            .load_dataset("https://zenodo.org/records/123456")
            .unwrap(),
        None
    );
}

#[test]
fn test_handles_github_urls_with_paths() {
    assert!(get_is_github("https://github.com/owner/repo/tree/main"));
}

#[test]
fn test_handles_github_urls_with_query_params() {
    assert!(get_is_github("https://github.com/owner/repo?tab=code"));
}

#[test]
fn test_returns_none_for_http_non_github_urls() {
    assert_eq!(
        GithubPlugin
            .load_dataset("http://example.com/data")
            .unwrap(),
        None
    );
}

#[test]
fn test_returns_none_for_gitlab_urls() {
    assert_eq!(
        GithubPlugin
            .load_dataset("https://gitlab.com/owner/repo")
            .unwrap(),
        None
    );
}
