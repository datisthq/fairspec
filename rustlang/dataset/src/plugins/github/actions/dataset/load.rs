use fairspec_metadata::{Descriptor, Error, Result};
use serde_json::Value;

use super::from_github::convert_dataset_from_github;
use crate::actions::dataset::merge::merge_datasets;
use crate::plugins::github::models::repository::GithubRepository;
use crate::plugins::github::models::tree::GithubTree;
use crate::plugins::github::services::github::make_github_api_request;

pub fn load_dataset_from_github(repo_url: &str, api_key: Option<&str>) -> Result<Descriptor> {
    let (owner, repo) = extract_repository_info(repo_url).ok_or_else(|| {
        Error::Load(format!(
            "Failed to extract repository info from URL: {repo_url}"
        ))
    })?;

    let mut repository: GithubRepository =
        make_github_api_request(&format!("/repos/{owner}/{repo}"), None, None, api_key)?;

    let reference = &repository.default_branch;
    let tree_response: GithubTree = make_github_api_request(
        &format!("/repos/{owner}/{repo}/git/trees/{reference}?recursive=1"),
        None,
        None,
        api_key,
    )?;
    repository.files = Some(tree_response.tree);

    let system_dataset = convert_dataset_from_github(&repository);
    let user_dataset_path = system_dataset
        .resources
        .iter()
        .flatten()
        .filter_map(|resource| resource.unstable_custom_metadata.as_ref())
        .find(|custom| custom.get("githubKey").and_then(Value::as_str) == Some("dataset.json"))
        .and_then(|custom| custom.get("githubUrl").and_then(Value::as_str))
        .map(String::from);

    let mut dataset = merge_datasets(&system_dataset, user_dataset_path.as_deref())?;

    for resource in dataset.resources.iter_mut().flatten() {
        resource.unstable_custom_metadata = None;
    }

    match serde_json::to_value(dataset) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        _ => Err(Error::Parse("Dataset is not an object".to_string())),
    }
}

fn extract_repository_info(repo_url: &str) -> Option<(String, String)> {
    let parsed = url::Url::parse(repo_url).ok()?;
    let parts: Vec<&str> = parsed
        .path()
        .split('/')
        .filter(|part| !part.is_empty())
        .collect();
    match parts.as_slice() {
        [owner, repo, ..] => Some((owner.to_string(), repo.to_string())),
        _ => None,
    }
}
