use serde_json::Value;

use crate::models::descriptor::Descriptor;
use crate::models::path::PathOr;

pub fn get_is_descriptor(value: &Value) -> bool {
    value.is_object()
}

impl From<Descriptor> for PathOr<Descriptor> {
    fn from(descriptor: Descriptor) -> Self {
        PathOr::Value(descriptor)
    }
}

impl From<&Descriptor> for PathOr<Descriptor> {
    fn from(descriptor: &Descriptor) -> Self {
        PathOr::Value(descriptor.clone())
    }
}

impl From<String> for PathOr<Descriptor> {
    fn from(path: String) -> Self {
        PathOr::Path(path)
    }
}

impl From<&str> for PathOr<Descriptor> {
    fn from(path: &str) -> Self {
        PathOr::Path(path.to_string())
    }
}
