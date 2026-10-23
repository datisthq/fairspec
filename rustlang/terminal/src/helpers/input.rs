use fairspec_library::fairspec_metadata::IntegrityType;

/// A required input's value; the program checks presence before the command runs.
pub fn require_value(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or_default()
}

pub fn parse_hash_type(value: Option<&str>) -> IntegrityType {
    match value {
        Some("md5") => IntegrityType::Md5,
        Some("sha1") => IntegrityType::Sha1,
        Some("sha512") => IntegrityType::Sha512,
        _ => IntegrityType::Sha256,
    }
}
