use fairspec_metadata::{IntegrityType, Resource, Result};
use serde_json::Value;

use super::infer::{infer_bytes, infer_integrity, infer_textual};
use super::prefetch::prefetch_file;
use crate::models::file::FileDescription;

pub fn describe_file(path: &str, hash_type: Option<IntegrityType>) -> Result<FileDescription> {
    let local_path = prefetch_file(path, None)?;
    let resource = Resource {
        data: Some(Value::String(local_path)),
        ..Resource::default()
    };

    Ok(FileDescription {
        bytes: infer_bytes(&resource, None)?,
        textual: infer_textual(&resource, None)?,
        integrity: infer_integrity(&resource, hash_type, None)?,
    })
}
