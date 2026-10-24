use crate::actions::path::general::get_file_extension;
use crate::actions::resource::data::get_data_first_path;
use crate::models::resource::Resource;

pub fn infer_file_dialect_format(resource: &Resource) -> Option<&'static str> {
    let path = get_data_first_path(resource)?;
    let extension = get_file_extension(&path)?;
    let format = match extension.as_str() {
        "csv" => "csv",
        "tsv" => "tsv",
        "json" => "json",
        "jsonl" | "ndjson" => "jsonl",
        "xlsx" => "xlsx",
        "ods" => "ods",
        "parquet" => "parquet",
        "arrow" | "feather" => "arrow",
        "sqlite" => "sqlite",
        _ => return None,
    };
    Some(format)
}
