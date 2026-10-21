use fairspec_metadata::{Descriptor, Integrity, IntegrityType, Resource, get_file_name_slug};
use serde_json::Value;

use crate::plugins::zenodo::models::file::ZenodoFile;

pub fn convert_resource_from_zenodo(zenodo_file: &ZenodoFile) -> Resource {
    let path = convert_path(&zenodo_file.links.self_);

    let name = get_file_name_slug(&zenodo_file.key)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| zenodo_file.id.clone());

    let mut custom_metadata = Descriptor::new();
    custom_metadata.insert(
        "zenodoKey".to_string(),
        Value::from(zenodo_file.key.as_str()),
    );
    custom_metadata.insert("zenodoUrl".to_string(), Value::from(path.as_str()));

    Resource {
        data: Some(Value::String(path)),
        name: Some(name),
        integrity: Some(Integrity {
            r#type: IntegrityType::Md5,
            hash: zenodo_file.checksum.replace("md5:", ""),
        }),
        unstable_custom_metadata: Some(custom_metadata),
        ..Resource::default()
    }
}

fn convert_path(link: &str) -> String {
    let result = link.replace("/api/", "/");
    match result.strip_suffix("/content") {
        Some(stripped) => stripped.to_string(),
        None => result,
    }
}
