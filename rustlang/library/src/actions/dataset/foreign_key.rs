use fairspec_metadata::{
    Dataset, FairspecError, ForeignKey, ForeignKeyError, Report, Resource, ResourceError,
    ResourceMissingError, Result, TableError, create_report, resolve_table_schema,
};
use fairspec_table::{LoadTableOptions, QUERY_ENGINE};
use polars::prelude::*;

use crate::actions::table::load::load_table;
use crate::models::table::ValidateTableOptions;

pub fn validate_dataset_foreign_keys(
    dataset: &Dataset,
    options: &ValidateTableOptions,
) -> Result<Report> {
    let mut errors = Vec::new();

    for resource in dataset.resources.iter().flatten() {
        let Some(table_schema) = resolve_table_schema(resource.table_schema.as_ref())? else {
            continue;
        };

        for foreign_key in table_schema.foreign_keys.iter().flatten() {
            errors.extend(validate_foreign_key(
                resource,
                foreign_key,
                dataset,
                &options.load_table_options,
            )?);
        }
    }

    Ok(create_report(errors, None))
}

fn validate_foreign_key(
    resource: &Resource,
    foreign_key: &ForeignKey,
    dataset: &Dataset,
    options: &LoadTableOptions,
) -> Result<Vec<FairspecError>> {
    let reference = &foreign_key.reference;
    let columns = &foreign_key.columns;
    let ref_columns = &reference.columns;

    if columns.is_empty() || ref_columns.is_empty() || columns.len() != ref_columns.len() {
        return Ok(Vec::new());
    }

    let ref_resource = match &reference.resource {
        Some(name) if !name.is_empty() => find_resource(dataset, name),
        _ => Some(resource),
    };
    let Some(ref_resource) = ref_resource else {
        let error = ResourceMissingError {
            resource_name: reference.resource.clone(),
            referencing_resource_name: resource.name.clone().unwrap_or_default(),
        };
        return Ok(vec![FairspecError::Resource(ResourceError::Missing(error))]);
    };

    let Some(table) = load_table(resource, options)? else {
        return Ok(Vec::new());
    };
    let Some(ref_table) = load_table(ref_resource, options)? else {
        return Ok(Vec::new());
    };

    let ref_selected = ref_table
        .select(
            ref_columns
                .iter()
                .zip(columns)
                .map(|(ref_name, name)| col(ref_name.as_str()).alias(name.as_str()))
                .collect::<Vec<_>>(),
        )
        .unique_stable(None, UniqueKeepStrategy::Any);

    let on: Vec<Expr> = columns.iter().map(|name| col(name.as_str())).collect();
    let violations = table
        .select(on.clone())
        .drop_nulls(None)
        .join(ref_selected, on.clone(), on, JoinArgs::new(JoinType::Anti))
        .map_err(|error| fairspec_metadata::Error::Load(error.to_string()))?
        .unique_stable(None, UniqueKeepStrategy::Any)
        .collect_with_engine(QUERY_ENGINE)
        .map(|result| result.unwrap_single())
        .map_err(|error| fairspec_metadata::Error::Load(error.to_string()))?;

    let mut errors = Vec::new();
    for index in 0..violations.height() {
        let row = violations
            .get_row(index)
            .map_err(|error| fairspec_metadata::Error::Load(error.to_string()))?;
        let cells = row.0.iter().map(stringify_cell).collect();
        errors.push(FairspecError::Table(TableError::ForeignKey(
            ForeignKeyError {
                r#type: Default::default(),
                resource_name: resource.name.clone(),
                foreign_key: foreign_key.clone(),
                cells,
            },
        )));
    }
    Ok(errors)
}

fn find_resource<'a>(dataset: &'a Dataset, name: &str) -> Option<&'a Resource> {
    dataset
        .resources
        .iter()
        .flatten()
        .find(|resource| resource.name.as_deref() == Some(name))
}

// Python's `str()` of a polars cell
fn stringify_cell(value: &AnyValue) -> String {
    match value {
        AnyValue::Null => "None".to_string(),
        AnyValue::Boolean(true) => "True".to_string(),
        AnyValue::Boolean(false) => "False".to_string(),
        AnyValue::Float32(value) => stringify_float(f64::from(*value)),
        AnyValue::Float64(value) => stringify_float(*value),
        other => match other.get_str() {
            Some(text) => text.to_string(),
            None => other.to_string(),
        },
    }
}

// Python's float repr: `1.0`, `1e+16`, `1e-05`, `inf`, `nan`
fn stringify_float(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    let magnitude = value.abs();
    if magnitude >= 1e16 || (magnitude < 1e-4 && magnitude > 0.0) {
        let text = format!("{value:e}");
        let (mantissa, exponent) = text.split_once('e').unwrap_or((&text, "0"));
        let (sign, digits) = match exponent.strip_prefix('-') {
            Some(digits) => ("-", digits),
            None => ("+", exponent),
        };
        return format!("{mantissa}e{sign}{digits:0>2}");
    }
    let text = value.to_string();
    if text.contains('.') {
        text
    } else {
        format!("{text}.0")
    }
}
