use fairspec_metadata::Descriptor;
use serde_json::Value;

/// A non-empty string, as JavaScript's truthiness reads it
pub(crate) fn get_text<'a>(descriptor: &'a Descriptor, key: &str) -> Option<&'a str> {
    descriptor
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
}

pub(crate) fn get_object<'a>(descriptor: &'a Descriptor, key: &str) -> Option<&'a Descriptor> {
    descriptor.get(key).and_then(Value::as_object)
}

pub(crate) fn get_list<'a>(descriptor: &'a Descriptor, key: &str) -> Option<&'a Vec<Value>> {
    descriptor.get(key).and_then(Value::as_array)
}

pub(crate) fn get_non_empty_list<'a>(
    descriptor: &'a Descriptor,
    key: &str,
) -> Option<&'a Vec<Value>> {
    get_list(descriptor, key).filter(|list| !list.is_empty())
}

pub(crate) fn get_is_truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(number)) => number.as_f64().is_some_and(|number| number != 0.0),
        Some(Value::String(text)) => !text.is_empty(),
        Some(_) => true,
    }
}

pub(crate) fn copy_value(target: &mut Descriptor, key: &str, value: Option<&Value>) {
    if let Some(value) = value {
        target.insert(key.to_string(), value.clone());
    }
}
