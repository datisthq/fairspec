use std::io::Read;

use fairspec_dataset::{InferFileDialectOptions, load_file_stream};
use fairspec_metadata::{
    CsvFileDialect, FileDialect, HeaderRows, Resource, Result, TsvFileDialect, get_data_first_path,
};

use crate::utils::sniffer::sniffer::Sniffer;

/// The csv or tsv dialect the sniffer finds in the first `sample_bytes` (default 10,000).
pub fn infer_csv_file_dialect(
    resource: &Resource,
    options: &InferFileDialectOptions,
) -> Result<Option<FileDialect>> {
    let sample_bytes = options.sample_bytes.unwrap_or(10_000);
    let Some(data_path) = get_data_first_path(resource) else {
        return Ok(None);
    };

    let mut raw_bytes = Vec::new();
    load_file_stream(data_path, None, Some(sample_bytes))?.read_to_end(&mut raw_bytes)?;

    let Ok(result) = Sniffer::new().sniff_bytes(&raw_bytes) else {
        return Ok(Some(FileDialect::Csv(CsvFileDialect::default())));
    };

    let line_terminator = result.dialect.line_terminator.as_str().to_string();
    let header_rows = if result.dialect.header.has_header_row {
        Some(HeaderRows::Rows(vec![
            result.dialect.header.num_preamble_rows + 1,
        ]))
    } else if result.num_fields > 0 {
        Some(HeaderRows::Disabled)
    } else {
        None
    };

    if result.dialect.delimiter == 9 {
        return Ok(Some(FileDialect::Tsv(TsvFileDialect {
            line_terminator: Some(line_terminator),
            header_rows,
            ..TsvFileDialect::default()
        })));
    }

    Ok(Some(FileDialect::Csv(CsvFileDialect {
        delimiter: char::from_u32(result.dialect.delimiter).map(String::from),
        line_terminator: Some(line_terminator),
        quote_char: result
            .dialect
            .quote
            .char
            .and_then(char::from_u32)
            .map(String::from),
        header_rows,
        ..CsvFileDialect::default()
    })))
}
