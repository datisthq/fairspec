use fairspec_dataset::prefetch_files;
use fairspec_metadata::{
    Error, FileDialect, PathOr, Resource, Result, get_supported_file_dialect, resolve_table_schema,
};
use serde_json::Value;

use crate::actions::table::file_dialect::concat_part_tables;
use crate::actions::table::normalize::normalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::models::table::{LoadTableOptions, Table};
use crate::plugins::xlsx::actions::file_dialect::infer::infer_xlsx_file_dialect;
use crate::plugins::xlsx::helpers::sheet::read_sheet_rows;
use crate::plugins::xlsx::helpers::table::{create_table_from_sheet_rows, get_sheet_header};

pub fn load_xlsx_table(resource: &Resource, options: &LoadTableOptions) -> Result<Table> {
    let Some(mut file_dialect) = get_supported_file_dialect(resource, &["xlsx", "ods"])? else {
        return Err(Error::Load("Resource data is not compatible".to_string()));
    };

    let paths = prefetch_files(
        resource,
        None,
        options.infer_table_schema_options.concurrency,
    )?;
    let Some(first_path) = paths.first() else {
        return Err(Error::Load("Resource path is not defined".to_string()));
    };

    if resource.file_dialect.is_none() {
        let resource = Resource {
            data: Some(Value::String(first_path.clone())),
            file_dialect: Some(PathOr::Value(file_dialect.clone())),
            ..resource.clone()
        };
        if let Some(inferred) = infer_xlsx_file_dialect(&resource, None)? {
            file_dialect = inferred;
        }
    }
    if !matches!(file_dialect, FileDialect::Xlsx(_) | FileDialect::Ods(_)) {
        return Err(Error::Load("Resource data is not compatible".to_string()));
    }

    // With columnNames the tables carry those names, so each part's own header is compared
    let is_own_header = paths.len() > 1 && get_has_column_names(&file_dialect);
    let mut tables = Vec::new();
    let mut part_headers = Vec::new();
    for path in &paths {
        if let Some(rows) = read_sheet_rows(path, &file_dialect, None)? {
            tables.push(create_table_from_sheet_rows(&rows, &file_dialect)?);
            if is_own_header {
                part_headers.push(get_sheet_header(&rows, &file_dialect)?);
            }
        }
    }
    let part_headers = is_own_header.then_some(part_headers);
    let mut result = match tables.len() {
        0 => return Err(Error::Load("Resource data has no such sheet".to_string())),
        1 => tables.into_iter().next().unwrap_or_default(),
        _ => concat_part_tables(tables, &file_dialect, part_headers)?,
    };

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

fn get_has_column_names(file_dialect: &FileDialect) -> bool {
    match file_dialect {
        FileDialect::Xlsx(dialect) => dialect.column_names.is_some(),
        FileDialect::Ods(dialect) => dialect.column_names.is_some(),
        _ => false,
    }
}
