use fairspec_metadata::Descriptor;
use serde_json::Value;

#[expect(dead_code, reason = "unused by the Python twin too, ported for parity")]
pub fn convert_resource_to_zenodo(resource: &Descriptor) -> Descriptor {
    let mut zenodo_file = Descriptor::new();

    if let Some(name) = resource.get("name").filter(|name| get_is_truthy(name)) {
        zenodo_file.insert("key".to_string(), name.clone());
    }

    if let Some(Value::Object(integrity)) = resource.get("integrity")
        && integrity.get("type").and_then(Value::as_str) == Some("md5")
    {
        let hash = match integrity.get("hash") {
            Some(Value::String(hash)) => hash.clone(),
            Some(hash) => hash.to_string(),
            None => String::new(),
        };
        zenodo_file.insert("checksum".to_string(), Value::String(format!("md5:{hash}")));
    }

    zenodo_file
}

fn get_is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64() != Some(0.0),
        Value::String(value) => !value.is_empty(),
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => !value.is_empty(),
    }
}
