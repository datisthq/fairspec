use std::sync::Arc;

use fairspec_dataset::prefetch_files;
use fairspec_metadata::{
    Error, FileDialect, PathOr, Resource, Result, get_supported_file_dialect, resolve_table_schema,
};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::table::file_dialect::{join_header_rows, skip_comment_rows};
use crate::actions::table::normalize::normalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::file_dialect::{get_comment_rows, get_header_rows};
use crate::models::table::{LoadTableOptions, Table};
use crate::plugins::csv::actions::file_dialect::infer::infer_csv_file_dialect;

pub fn load_csv_table(resource: &Resource, options: &LoadTableOptions) -> Result<Table> {
    let Some(mut file_dialect) = get_supported_file_dialect(resource, &["csv", "tsv"])? else {
        return Err(Error::Load("Resource data is not compatible".to_string()));
    };

    let paths = prefetch_files(
        resource,
        options.preview_bytes,
        options.infer_table_schema_options.concurrency,
    )?;
    let Some(first_path) = paths.first() else {
        return Err(Error::Load("Resource path is not defined".to_string()));
    };

    if get_dialect_has_only_format(&file_dialect) {
        let resource = Resource {
            data: Some(Value::String(first_path.clone())),
            file_dialect: Some(PathOr::Value(file_dialect.clone())),
            ..Resource::default()
        };
        if let Some(inferred) = infer_csv_file_dialect(&resource, &Default::default())? {
            file_dialect = inferred;
        }
    }

    let scan = CsvScan::new(&file_dialect);
    let header_rows = get_header_rows(Some(&file_dialect));
    let column_names = get_column_names(&file_dialect);
    let is_multi_header = header_rows.len() > 1 && column_names.is_none();
    let mut schema = scan.schema.clone();
    if is_multi_header {
        schema = Some(get_headerless_schema(first_path, &scan)?);
    }

    let mut tables = Vec::new();
    for path in &paths {
        tables.push(
            scan.create_reader(path, schema.clone(), is_multi_header)
                .finish()
                .into_result()?,
        );
    }
    let mut result = concat(tables, UnionArgs::default()).into_result()?;

    let names: Vec<String> = result
        .collect_schema()
        .into_result()?
        .iter_names()
        .map(|name| name.to_string())
        .collect();
    if !scan.has_header && column_names.is_none() && !is_multi_header {
        // By position, not from polars' own names: polars numbers them from column_1
        let new: Vec<String> = (1..=names.len())
            .map(|index| format!("column{index}"))
            .collect();
        result = result.rename(names, new, true);
    } else if scan.has_header
        && let Some(column_names) = column_names
    {
        // Names that replace a header go in by position, as polars' new_columns does
        let count = column_names.len().min(names.len());
        result = result.rename(&names[..count], &column_names[..count], true);
    }

    if header_rows.len() > 1 {
        result = join_header_rows(result, &file_dialect)?;
    }
    if !get_comment_rows(&file_dialect).is_empty() {
        result = skip_comment_rows(result, &file_dialect);
    }

    if options.denormalized != Some(true) {
        let table_schema = match resolve_table_schema(resource.table_schema.as_ref())? {
            Some(table_schema) => table_schema,
            None => {
                infer_table_schema_from_table(result.clone(), &options.infer_table_schema_options)?
            }
        };
        result = normalize_table(result, &table_schema)?;
    }

    Ok(result)
}

struct CsvScan {
    skip_rows: usize,
    has_header: bool,
    eol_char: u8,
    separator: u8,
    quote_char: Option<u8>,
    null_values: Option<String>,
    comment_prefix: Option<String>,
    schema: Option<SchemaRef>,
}

impl CsvScan {
    fn new(file_dialect: &FileDialect) -> Self {
        let header_rows = get_header_rows(Some(file_dialect));
        let column_names = get_column_names(file_dialect);
        let has_header =
            header_rows.len() == 1 || (header_rows.len() > 1 && column_names.is_some());
        let (line_terminator, null_sequence, comment_prefix) = match file_dialect {
            FileDialect::Csv(dialect) => (
                dialect.line_terminator.clone(),
                dialect.null_sequence.clone(),
                dialect.comment_prefix.clone(),
            ),
            FileDialect::Tsv(dialect) => (
                dialect.line_terminator.clone(),
                dialect.null_sequence.clone(),
                dialect.comment_prefix.clone(),
            ),
            _ => (None, None, None),
        };
        // Polars takes a single eol character and drops the "\r" before a "\n" itself
        let eol_char = line_terminator
            .filter(|terminator| !terminator.is_empty())
            .and_then(|terminator| terminator.bytes().last())
            .unwrap_or(b'\n');
        let (separator, quote_char) = match file_dialect {
            FileDialect::Tsv(_) => (b'\t', None),
            FileDialect::Csv(dialect) => (
                get_first_byte(dialect.delimiter.as_deref()).unwrap_or(b','),
                Some(get_first_byte(dialect.quote_char.as_deref()).unwrap_or(b'"')),
            ),
            _ => (b',', Some(b'"')),
        };
        let schema = match column_names {
            Some(names) if !has_header => Some(Arc::new(
                names
                    .iter()
                    .map(|name| Field::new(name.as_str().into(), DataType::String))
                    .collect::<Schema>(),
            )),
            _ => None,
        };
        CsvScan {
            skip_rows: header_rows.first().map_or(0, |row| row - 1),
            has_header,
            eol_char,
            separator,
            quote_char,
            null_values: null_sequence,
            comment_prefix,
            schema,
        }
    }

    fn create_reader(
        &self,
        path: &str,
        schema: Option<SchemaRef>,
        is_multi_header: bool,
    ) -> LazyCsvReader {
        let mut reader = LazyCsvReader::new(PlRefPath::new(path))
            .with_infer_schema_length(Some(0))
            .with_truncate_ragged_lines(true)
            .with_skip_rows(self.skip_rows)
            .with_has_header(self.has_header)
            .with_eol_char(self.eol_char)
            .with_separator(self.separator)
            .with_quote_char(self.quote_char)
            .with_null_values(
                self.null_values
                    .as_ref()
                    .map(|value| NullValues::AllColumnsSingle(value.as_str().into())),
            )
            .with_comment_prefix(self.comment_prefix.as_deref().map(Into::into))
            .with_schema(schema);
        if is_multi_header {
            // A first header row may be shorter than the rows below it
            reader = reader
                .with_missing_columns_policy(Some(MissingColumnsPolicy::Insert))
                .with_missing_is_null(true);
        }
        reader
    }
}

// Header rows are joined from the data, so every column is read as text and the
// widest of the first rows sets the width: a first header row may be shorter
fn get_headerless_schema(path: &str, scan: &CsvScan) -> Result<SchemaRef> {
    let names = scan
        .create_reader(path, None, false)
        .with_infer_schema_length(Some(100))
        .finish()
        .into_result()?
        .collect_schema()
        .into_result()?;
    Ok(Arc::new(
        names
            .iter_names()
            .map(|name| Field::new(name.clone(), DataType::String))
            .collect::<Schema>(),
    ))
}

fn get_column_names(file_dialect: &FileDialect) -> Option<&Vec<String>> {
    match file_dialect {
        FileDialect::Csv(dialect) => dialect.column_names.as_ref(),
        FileDialect::Tsv(dialect) => dialect.column_names.as_ref(),
        _ => None,
    }
    .filter(|names| !names.is_empty())
}

fn get_first_byte(text: Option<&str>) -> Option<u8> {
    text.and_then(|text| text.bytes().next())
}

fn get_dialect_has_only_format(file_dialect: &FileDialect) -> bool {
    let Ok(Value::Object(descriptor)) = serde_json::to_value(file_dialect) else {
        return true;
    };
    descriptor
        .keys()
        .all(|key| matches!(key.as_str(), "format" | "title" | "description"))
}
