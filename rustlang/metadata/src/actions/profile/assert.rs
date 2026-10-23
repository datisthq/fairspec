use regex::Regex;
use serde_json::Value;

use super::registry::get_profile_type_name;
use crate::models::exception::{Error, Result};
use crate::models::json_schema::JsonSchema;
use crate::models::profile::{Profile, ProfileType};

pub fn assert_profile(
    json_schema: JsonSchema,
    path: &str,
    profile_type: ProfileType,
) -> Result<Profile> {
    let type_name = get_profile_type_name(profile_type);
    let regex = Regex::new(&format!(
        r"^https://fairspec\.org/profiles/(\d+\.\d+\.\d+|latest)/{}\.json$",
        regex::escape(type_name)
    ))
    .expect("profile URL pattern is valid");

    let mut paths = vec![path];
    if let Some(Value::Array(refs)) = json_schema.get("allOf") {
        paths.extend(refs.iter().filter_map(|item| match item {
            Value::String(path) => Some(path.as_str()),
            Value::Object(object) => object.get("$ref").and_then(Value::as_str),
            _ => None,
        }));
    }

    if profile_type == ProfileType::DataSchema
        && path == "https://json-schema.org/draft/2020-12/schema"
    {
        return Ok(json_schema);
    }

    if paths.iter().any(|path| regex.is_match(path)) {
        return Ok(json_schema);
    }

    Err(Error::Profile(format!(
        "Profile at path {path} is not a valid {type_name} profile"
    )))
}
