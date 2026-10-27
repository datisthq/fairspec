/// Convert a Frictionless name to a Fairspec resource name (letters, digits and `_`)
pub fn convert_name_from_frictionless(name: &str) -> String {
    name.chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() || char == '_' {
                char
            } else {
                '_'
            }
        })
        .collect()
}
