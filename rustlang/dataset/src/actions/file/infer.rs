use std::fs;
use std::io::Read;

use fairspec_metadata::{Integrity, IntegrityType, Resource, Result, get_data_paths};
use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

use super::load::load_file;
use super::prefetch::prefetch_files;
use crate::actions::stream::concat::concat_file_streams;
use crate::actions::stream::load::load_file_stream;
use crate::helpers::concurrency::run_concurrent_tasks;

/// Whether every data file is UTF-8 text, judged on its first `sample_bytes` (default 10,000).
pub fn infer_textual(resource: &Resource, sample_bytes: Option<usize>) -> Result<bool> {
    let paths = get_data_paths(resource);
    if paths.is_empty() {
        return Ok(false);
    }
    for path in paths {
        if !get_is_textual(&path, sample_bytes.unwrap_or(10_000))? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn infer_integrity(
    resource: &Resource,
    hash_type: Option<IntegrityType>,
    concurrency: Option<usize>,
) -> Result<Option<Integrity>> {
    let hash_type = hash_type.unwrap_or(IntegrityType::Sha256);
    let hash = infer_hash(resource, Some(hash_type), concurrency)?;
    if hash.is_empty() {
        return Ok(None);
    }
    Ok(Some(Integrity {
        extra: Default::default(),
        r#type: hash_type,
        hash,
    }))
}

/// The lowercase hex digest of all data files concatenated; empty without data files.
pub fn infer_hash(
    resource: &Resource,
    hash_type: Option<IntegrityType>,
    concurrency: Option<usize>,
) -> Result<String> {
    let local_paths = prefetch_files(resource, None, concurrency)?;
    if local_paths.is_empty() {
        return Ok(String::new());
    }

    let streams = run_concurrent_tasks(
        |path: String| load_file_stream(path, None, None),
        local_paths,
        concurrency,
    )
    .into_iter()
    .collect::<Result<Vec<_>>>()?;
    let mut data = Vec::new();
    concat_file_streams(streams)?.read_to_end(&mut data)?;

    Ok(match hash_type.unwrap_or(IntegrityType::Sha256) {
        IntegrityType::Md5 => format_digest(&Md5::digest(&data)),
        IntegrityType::Sha1 => format_digest(&Sha1::digest(&data)),
        IntegrityType::Sha256 => format_digest(&Sha256::digest(&data)),
        IntegrityType::Sha512 => format_digest(&Sha512::digest(&data)),
    })
}

pub fn infer_bytes(resource: &Resource, concurrency: Option<usize>) -> Result<u64> {
    let mut total = 0;
    for local_path in prefetch_files(resource, None, concurrency)? {
        total += fs::metadata(local_path)?.len();
    }
    Ok(total)
}

// Python asks charset-normalizer when the sample is not UTF-8, but accepts only the
// hyphenated "utf-8"/"ascii" while the detector names codecs "utf_8"/"ascii", which
// non-UTF-8 bytes can never be: so the answer is exactly "valid UTF-8"
fn get_is_textual(path: &str, sample_bytes: usize) -> Result<bool> {
    let buffer = load_file(path, Some(sample_bytes))?;
    if buffer.is_empty() {
        return Ok(true);
    }
    if get_is_binary(&buffer) {
        return Ok(false);
    }
    Ok(std::str::from_utf8(&buffer).is_ok())
}

pub(crate) fn get_is_binary(data: &[u8]) -> bool {
    data.iter().any(|byte| matches!(byte, 0..=7 | 14..=31))
}

fn format_digest(digest: &[u8]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}
