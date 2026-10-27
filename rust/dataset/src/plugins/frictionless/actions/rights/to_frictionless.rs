use fairspec_metadata::Descriptor;
use serde_json::Value;

use crate::plugins::frictionless::helpers::value::get_text;

pub fn convert_rights_to_frictionless(rights_list: &[Value]) -> Vec<Value> {
    rights_list
        .iter()
        .filter_map(Value::as_object)
        .filter_map(|rights| {
            let mut license = Descriptor::new();
            if let Some(identifier) = get_text(rights, "rightsIdentifier") {
                license.insert("name".to_string(), identifier.into());
            }
            if let Some(uri) = get_text(rights, "rightsUri") {
                license.insert("path".to_string(), uri.into());
            }
            if let Some(title) = get_text(rights, "rights") {
                license.insert("title".to_string(), title.into());
            }
            (license.contains_key("name") || license.contains_key("path"))
                .then_some(Value::Object(license))
        })
        .collect()
}
