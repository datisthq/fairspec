use std::fs;
use std::io::Write;

use serde::Serialize;
use serde_json::Value;

use super::stringify::stringify_descriptor;
use crate::actions::profile::registry::get_profile_type_name;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::profile::ProfileType;
use crate::settings::FAIRSPEC_VERSION;

pub fn save_descriptor(descriptor: &Descriptor, path: &str, overwrite: bool) -> Result<()> {
    let text = stringify_descriptor(descriptor);
    if let Some(parent) = std::path::Path::new(path).parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(overwrite)
        .create_new(!overwrite)
        .open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

/// Serializes a model and stamps the versioned profile URL when it has no `$schema`.
pub(crate) fn save_profiled_model(
    model: &impl Serialize,
    profile_type: ProfileType,
    path: &str,
    overwrite: bool,
) -> Result<()> {
    let mut descriptor = match serde_json::to_value(model) {
        Ok(Value::Object(descriptor)) => descriptor,
        Ok(_) => return Err(Error::Parse("Model is not an object".to_string())),
        Err(error) => return Err(Error::Parse(error.to_string())),
    };

    if !descriptor.contains_key("$schema") {
        descriptor.insert(
            "$schema".to_string(),
            Value::String(format!(
                "https://fairspec.org/profiles/{FAIRSPEC_VERSION}/{}.json",
                get_profile_type_name(profile_type)
            )),
        );
    }

    save_descriptor(&descriptor, path, overwrite)
}
