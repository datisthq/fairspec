use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_path_to_error::Segment;

use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;

/// Parses a descriptor with a built-in model, reporting a mismatch as a metadata error.
pub fn inspect_model<T: DeserializeOwned>(
    descriptor: &Descriptor,
    root_json_pointer: Option<&str>,
) -> (Option<T>, Vec<FairspecError>) {
    let value = Value::Object(descriptor.clone());
    match serde_path_to_error::deserialize::<_, T>(value) {
        Ok(model) => (Some(model), Vec::new()),
        Err(error) => {
            let root = match root_json_pointer {
                None | Some("/") => "",
                Some(root) => root,
            };
            let pointer: String = error
                .path()
                .iter()
                .filter_map(|segment| match segment {
                    Segment::Seq { index } => Some(format!("/{index}")),
                    Segment::Map { key } => {
                        Some(format!("/{}", key.replace('~', "~0").replace('/', "~1")))
                    }
                    _ => None,
                })
                .collect();
            let error = MetadataError {
                message: error.inner().to_string(),
                json_pointer: format!("{root}{pointer}"),
                ..Default::default()
            };
            (None, vec![FairspecError::Metadata(error)])
        }
    }
}
