use fairspec_metadata::Descriptor;
use serde_json::Value;

use crate::plugins::frictionless::helpers::value::get_text;

pub fn convert_rights_from_frictionless(licenses: &[Value]) -> Vec<Value> {
    licenses
        .iter()
        .filter_map(Value::as_object)
        .map(|license| {
            let mut rights = Descriptor::new();
            let title = license
                .get("title")
                .or_else(|| license.get("name"))
                .and_then(Value::as_str)
                .filter(|title| !title.is_empty());
            if let Some(title) = title {
                rights.insert("rights".to_string(), title.into());
            }
            if let Some(path) = get_text(license, "path") {
                rights.insert("rightsUri".to_string(), path.into());
            }
            if let Some(name) = get_text(license, "name") {
                rights.insert("rightsIdentifier".to_string(), name.into());
            }
            Value::Object(rights)
        })
        .collect()
}
