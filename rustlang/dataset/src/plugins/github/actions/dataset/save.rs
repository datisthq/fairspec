use std::io::Read;

use base64::Engine;

use fairspec_metadata::{
    Dataset, Descriptor, Error, ResourceDataPath, Result, denormalize_dataset, stringify_descriptor,
};
use serde_json::{Value, json};

use crate::actions::dataset::basepath::get_dataset_basepath;
use crate::actions::resource::save::{SaveFileProps, save_resource_files};
use crate::actions::stream::load::load_file_stream;
use crate::plugins::github::models::content::GithubContent;
use crate::plugins::github::models::repository::GithubRepository;
use crate::plugins::github::services::github::make_github_api_request;

pub fn save_dataset_to_github(
    dataset: &Dataset,
    api_key: &str,
    repo: &str,
    org: Option<&str>,
) -> Result<Descriptor> {
    let basepath = get_dataset_basepath(dataset)?;

    let endpoint = match org.filter(|org| !org.is_empty()) {
        Some(org) => format!("/orgs/{org}/repos"),
        None => "/user/repos".to_string(),
    };
    let github_repository: GithubRepository = make_github_api_request(
        &endpoint,
        Some("POST"),
        Some(&create_payload(json!({"name": repo, "auto_init": true}))),
        Some(api_key),
    )?;

    let owner_login = &github_repository.owner.login;

    let mut resource_descriptors = Vec::new();
    for resource in dataset.resources.iter().flatten() {
        let mut save_file = |props: &SaveFileProps| {
            let content = encode_base64(&read_file(&props.normalized_path)?);
            let path = &props.denormalized_path;

            let payload = create_payload(json!({
                "path": path,
                "content": content,
                "message": format!("Added file \"{path}\""),
            }));
            let _: GithubContent = make_github_api_request(
                &format!("/repos/{owner_login}/{repo}/contents/{path}"),
                Some("PUT"),
                Some(&payload),
                Some(api_key),
            )?;

            Ok(path.clone())
        };

        resource_descriptors.push(Value::Object(save_resource_files(
            resource,
            &mut save_file,
            basepath.as_deref(),
            false,
            false,
        )?));
    }

    let denormalized = denormalize_dataset(dataset, basepath.as_deref())?;
    let mut descriptor = match serde_json::to_value(&denormalized) {
        Ok(Value::Object(descriptor)) => descriptor,
        _ => return Err(Error::Parse("Dataset is not an object".to_string())),
    };
    descriptor.insert("resources".to_string(), Value::Array(resource_descriptors));

    let content = encode_base64(stringify_descriptor(&descriptor).as_bytes());
    let _: GithubContent = make_github_api_request(
        &format!("/repos/{owner_login}/{repo}/contents/dataset.json"),
        Some("PUT"),
        Some(&create_payload(json!({
            "path": "dataset.json",
            "message": "Added file \"dataset.json\"",
            "content": content,
        }))),
        Some(api_key),
    )?;

    Ok(create_payload(json!({
        "path": format!(
            "https://raw.githubusercontent.com/{owner_login}/{repo}/refs/heads/main/dataset.json"
        ),
        "repo_url": github_repository.html_url,
    })))
}

fn create_payload(value: Value) -> Descriptor {
    match value {
        Value::Object(payload) => payload,
        _ => Descriptor::new(),
    }
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    load_file_stream(ResourceDataPath::Path(path.to_string()), None, None)?
        .read_to_end(&mut data)
        .map_err(Error::Io)?;
    Ok(data)
}

fn encode_base64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}
