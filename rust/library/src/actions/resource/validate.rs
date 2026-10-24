use fairspec_dataset::validate_file;
use fairspec_metadata::{Report, Resource, Result, create_report};

use crate::actions::data::validate::validate_data;
use crate::actions::table::validate::validate_table;
use crate::models::table::ValidateTableOptions;

pub fn validate_resource(resource: &Resource, options: &ValidateTableOptions) -> Result<Report> {
    let mut errors = Vec::new();
    let concurrency = options
        .load_table_options
        .infer_table_schema_options
        .concurrency;

    let file_report = validate_file(resource, concurrency)?;
    errors.extend(file_report.errors);
    if !file_report.valid {
        return Ok(create_report(errors, None));
    }

    let data_report = validate_data(resource)?;
    errors.extend(data_report.errors);
    if !data_report.valid {
        return Ok(create_report(errors, None));
    }

    let table_report = validate_table(resource, options)?;
    errors.extend(table_report.errors);
    Ok(create_report(errors, None))
}
