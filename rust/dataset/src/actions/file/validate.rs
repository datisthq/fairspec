use fairspec_metadata::{
    FairspecError, FileError, IntegrityError, IntegrityType, MetadataError, MetadataErrorType,
    Report, Resource, Result, TextualError, create_report,
};

use super::infer::{infer_hash, infer_textual};

pub fn validate_file(resource: &Resource, concurrency: Option<usize>) -> Result<Report> {
    let mut errors = Vec::new();

    if resource.textual == Some(true) && !infer_textual(resource, None)? {
        errors.push(FairspecError::File(FileError::Textual(
            TextualError::default(),
        )));
    }

    if let Some(integrity) = &resource.integrity {
        let length = get_hash_length(integrity.r#type);
        let is_hex = |char: char| char.is_ascii_digit() || ('a'..='f').contains(&char);
        if integrity.hash.len() != length || !integrity.hash.chars().all(is_hex) {
            errors.push(FairspecError::Metadata(MetadataError {
                r#type: MetadataErrorType::Metadata,
                resource_name: None,
                message: format!("must be {length} lowercase hexadecimal digits"),
                json_pointer: "/integrity/hash".to_string(),
            }));
            return Ok(create_report(errors, None));
        }
        let actual_hash = infer_hash(resource, Some(integrity.r#type), concurrency)?;
        if actual_hash != integrity.hash {
            errors.push(FairspecError::File(FileError::Integrity(IntegrityError {
                hash_type: get_hash_type_name(integrity.r#type),
                expected_hash: integrity.hash.clone(),
                actual_hash,
                ..IntegrityError::default()
            })));
        }
    }

    Ok(create_report(errors, None))
}

fn get_hash_length(hash_type: IntegrityType) -> usize {
    match hash_type {
        IntegrityType::Md5 => 32,
        IntegrityType::Sha1 => 40,
        IntegrityType::Sha256 => 64,
        IntegrityType::Sha512 => 128,
    }
}

fn get_hash_type_name(hash_type: fairspec_metadata::IntegrityType) -> String {
    serde_json::to_value(hash_type)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_default()
}
