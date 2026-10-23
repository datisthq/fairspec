use fairspec_metadata::{Descriptor, Integrity, IntegrityType, Resource, get_file_name_slug};
use serde_json::Value;

use crate::plugins::github::models::file::GithubFile;

pub fn convert_resource_from_github(github_file: &GithubFile, default_branch: &str) -> Resource {
    let path = convert_path(&github_file.url, default_branch, &github_file.path);

    let name = get_file_name_slug(&path)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| github_file.sha.clone());

    let mut custom_metadata = Descriptor::new();
    custom_metadata.insert(
        "githubKey".to_string(),
        Value::from(github_file.path.as_str()),
    );
    custom_metadata.insert("githubUrl".to_string(), Value::from(path.as_str()));

    Resource {
        data: Some(Value::String(path)),
        name: Some(name),
        integrity: Some(Integrity {
            extra: Default::default(),
            r#type: IntegrityType::Sha1,
            hash: github_file.sha.clone(),
        }),
        unstable_custom_metadata: Some(custom_metadata),
        ..Resource::default()
    }
}

fn convert_path(url: &str, reference: &str, file_path: &str) -> String {
    let pathname = url::Url::parse(url)
        .map(|url| url.path().to_string())
        .unwrap_or_default();
    let parts: Vec<&str> = pathname.split('/').skip(2).collect();
    let (owner, repo) = match parts.as_slice() {
        [owner, repo, ..] => (*owner, *repo),
        _ => ("", ""),
    };
    format!("https://raw.githubusercontent.com/{owner}/{repo}/refs/heads/{reference}/{file_path}")
}
