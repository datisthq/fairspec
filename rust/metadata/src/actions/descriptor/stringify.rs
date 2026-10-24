use crate::models::descriptor::Descriptor;

pub fn stringify_descriptor(descriptor: &Descriptor) -> String {
    serde_json::to_string_pretty(descriptor).unwrap_or_default()
}
