use std::fs::File;
use std::io::{Read, Take, Write};

use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::{Error, FileDialect, Result};
use polars::prelude::*;

use crate::helpers::error::PolarsResultExt;
use crate::models::table::Table;

// Control bytes a text file almost never holds; tab, LF and CR are left out
const PLACEHOLDER_BYTES: [u8; 28] = [
    31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 12, 11, 8, 7, 6, 5, 4,
    3, 2, 1,
];
pub const CHUNK_SIZE: usize = 65536;

#[derive(Debug, Clone, PartialEq)]
pub struct DelimiterSwap {
    pub delimiter: String,
    pub placeholder: String,
}

pub struct SwappedDelimiter {
    pub swap: DelimiterSwap,
    pub paths: Vec<String>,
    pub file_dialect: FileDialect,
}

/// Polars splits cells on one byte, so a multi-byte delimiter is swapped in copies of the
/// files for a control byte none of them holds; `restore_delimiter` swaps it back.
pub fn swap_delimiter(
    paths: &[String],
    file_dialect: &FileDialect,
    max_bytes: Option<usize>,
) -> Result<Option<SwappedDelimiter>> {
    let FileDialect::Csv(dialect) = file_dialect else {
        return Ok(None);
    };
    let Some(delimiter) = dialect.delimiter.clone().filter(|text| text.len() > 1) else {
        return Ok(None);
    };

    let reserved: Vec<u8> = [
        &dialect.quote_char,
        &dialect.line_terminator,
        &dialect.null_sequence,
        &dialect.comment_prefix,
    ]
    .into_iter()
    .flatten()
    .flat_map(|text| text.bytes())
    .collect();
    let candidates: Vec<u8> = PLACEHOLDER_BYTES
        .into_iter()
        .filter(|byte| !reserved.contains(byte))
        .collect();
    let Some(placeholder) = find_absent_byte(paths, &candidates, max_bytes)? else {
        return Err(Error::Load(format!(
            "Delimiter \"{delimiter}\" cannot be read: the file holds every control byte"
        )));
    };

    let swap = DelimiterSwap {
        delimiter: delimiter.clone(),
        placeholder: char::from(placeholder).to_string(),
    };
    let swap_text = |text: &Option<String>| {
        text.as_ref()
            .map(|text| text.replace(&delimiter, &swap.placeholder))
    };
    let mut swapped = dialect.clone();
    swapped.delimiter = Some(swap.placeholder.clone());
    swapped.null_sequence = swap_text(&dialect.null_sequence);
    swapped.comment_prefix = swap_text(&dialect.comment_prefix);
    let paths = paths
        .iter()
        .map(|path| swap_file_delimiter(path, delimiter.as_bytes(), placeholder, max_bytes))
        .collect::<Result<Vec<_>>>()?;

    Ok(Some(SwappedDelimiter {
        swap,
        paths,
        file_dialect: FileDialect::Csv(swapped),
    }))
}

/// Puts the multi-byte delimiter back in every cell and column name.
pub fn restore_delimiter(mut table: Table, swap: &DelimiterSwap) -> Result<Table> {
    let names: Vec<String> = table
        .collect_schema()
        .into_result()?
        .iter_names()
        .map(|name| name.to_string())
        .collect();
    let exprs: Vec<Expr> = names
        .iter()
        .map(|name| {
            col(name.as_str())
                .str()
                .replace_all(
                    lit(swap.placeholder.as_str()),
                    lit(swap.delimiter.as_str()),
                    true,
                )
                .alias(name.replace(&swap.placeholder, &swap.delimiter))
        })
        .collect();
    Ok(table.select(exprs))
}

fn find_absent_byte(
    paths: &[String],
    candidates: &[u8],
    max_bytes: Option<usize>,
) -> Result<Option<u8>> {
    for &byte in candidates {
        let mut is_present = false;
        for path in paths {
            if get_is_byte_present(path, byte, max_bytes)? {
                is_present = true;
                break;
            }
        }
        if !is_present {
            return Ok(Some(byte));
        }
    }
    Ok(None)
}

fn get_is_byte_present(path: &str, byte: u8, max_bytes: Option<usize>) -> Result<bool> {
    let mut file = open_file(path, max_bytes)?;
    let mut chunk = vec![0; CHUNK_SIZE];
    loop {
        let size = file.read(&mut chunk)?;
        if size == 0 {
            return Ok(false);
        }
        if chunk[..size].contains(&byte) {
            return Ok(true);
        }
    }
}

fn swap_file_delimiter(
    path: &str,
    needle: &[u8],
    placeholder: u8,
    max_bytes: Option<usize>,
) -> Result<String> {
    let target = get_temp_file_path(false, None, Some("csv"))?;
    let mut source = open_file(path, max_bytes)?;
    let mut output = File::create(&target)?;
    let mut chunk = vec![0; CHUNK_SIZE];
    let mut data: Vec<u8> = Vec::new();
    let mut swapped: Vec<u8> = Vec::new();
    loop {
        let size = source.read(&mut chunk)?;
        if size == 0 {
            break;
        }
        data.extend_from_slice(&chunk[..size]);
        // A delimiter cut by the chunk boundary waits for the next chunk
        let cut = (1..needle.len())
            .rev()
            .find(|size| data.ends_with(&needle[..*size]))
            .map_or(data.len(), |size| data.len() - size);
        swapped.clear();
        replace_bytes(&data[..cut], needle, placeholder, &mut swapped);
        output.write_all(&swapped)?;
        data.drain(..cut);
    }
    output.write_all(&data)?;
    Ok(target)
}

fn replace_bytes(data: &[u8], needle: &[u8], placeholder: u8, output: &mut Vec<u8>) {
    let mut start = 0;
    let mut index = 0;
    while index + needle.len() <= data.len() {
        if data[index..].starts_with(needle) {
            output.extend_from_slice(&data[start..index]);
            output.push(placeholder);
            index += needle.len();
            start = index;
        } else {
            index += 1;
        }
    }
    output.extend_from_slice(&data[start..]);
}

// A preview reads only its first bytes, so neither pass reads past them
pub(crate) fn open_file(path: &str, max_bytes: Option<usize>) -> Result<Take<File>> {
    let limit = max_bytes.filter(|max_bytes| *max_bytes > 0);
    Ok(File::open(path)?.take(limit.map_or(u64::MAX, |limit| limit as u64)))
}
