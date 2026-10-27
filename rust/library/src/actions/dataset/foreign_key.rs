use fairspec_metadata::{
    Dataset, FairspecError, ForeignKey, ForeignKeyError, MetadataError, Report, Resource,
    ResourceError, ResourceMissingError, Result, TableError, create_report, resolve_table_schema,
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

    for (index, resource) in dataset.resources.iter().flatten().enumerate() {
        let Some(table_schema) = resolve_table_schema(resource.table_schema.as_ref())? else {
            continue;
        };

        for (fk_index, foreign_key) in table_schema.foreign_keys.iter().flatten().enumerate() {
            errors.extend(validate_foreign_key(
                resource,
                foreign_key,
                dataset,
                &format!("/resources/{index}/tableSchema/foreignKeys/{fk_index}"),
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
    json_pointer: &str,
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

    let Some(key_types) = get_key_types(&table, &ref_table, columns, ref_columns)? else {
        return Ok(vec![FairspecError::Metadata(MetadataError {
            message: "foreign key columns and reference columns differ in type".to_string(),
            json_pointer: json_pointer.to_string(),
            ..Default::default()
        })]);
    };

    let key_names: Vec<String> = (0..columns.len())
        .map(|index| format!("fairspec:key:{index}"))
        .collect();
    let create_keys = |names: &[String]| -> Vec<Expr> {
        names
            .iter()
            .zip(&key_types)
            .zip(&key_names)
            .map(|((name, key_type), key_name)| {
                cast_key(col(name.as_str()), key_type.as_ref()).alias(key_name.as_str())
            })
            .collect()
    };
    let ref_selected = ref_table
        .select(create_keys(ref_columns))
        .unique_stable(None, UniqueKeepStrategy::Any);

    let on: Vec<Expr> = key_names.iter().map(|name| col(name.as_str())).collect();
    let selected: Vec<Expr> = columns.iter().map(|name| col(name.as_str())).collect();
    let violations = table
        .select(selected.clone())
        .drop_nulls(None)
        .with_columns(create_keys(columns))
        .join(ref_selected, on.clone(), on, JoinArgs::new(JoinType::Anti))
        .map_err(|error| fairspec_metadata::Error::Load(error.to_string()))?
        .select(selected)
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

/// The type each key pair joins on: numeric columns of differing types compare by
/// value, any other differing pair is `None`, which never joins.
fn get_key_types(
    table: &LazyFrame,
    ref_table: &LazyFrame,
    columns: &[String],
    ref_columns: &[String],
) -> Result<Option<Vec<Option<DataType>>>> {
    let to_error = |error: PolarsError| fairspec_metadata::Error::Load(error.to_string());
    let schema = table.clone().collect_schema().map_err(to_error)?;
    let ref_schema = ref_table.clone().collect_schema().map_err(to_error)?;

    let mut key_types = Vec::new();
    for (name, ref_name) in columns.iter().zip(ref_columns) {
        let (Some(dtype), Some(ref_dtype)) = (schema.get(name), ref_schema.get(ref_name)) else {
            return Ok(None);
        };
        let key_type = if dtype.is_categorical() || dtype.is_enum() {
            if !(ref_dtype.is_categorical() || ref_dtype.is_enum()) {
                return Ok(None);
            }
            Some(DataType::String)
        } else if dtype.is_numeric() && ref_dtype.is_numeric() {
            if dtype == ref_dtype {
                None
            } else if dtype.is_float() || ref_dtype.is_float() {
                Some(DataType::Float64)
            } else {
                Some(DataType::Decimal(DECIMAL_PRECISION, DECIMAL_SCALE))
            }
        } else if dtype == ref_dtype {
            None
        } else {
            return Ok(None);
        };
        key_types.push(key_type);
    }
    Ok(Some(key_types))
}

fn cast_key(column_expr: Expr, key_type: Option<&DataType>) -> Expr {
    match key_type {
        Some(key_type) => column_expr.cast(key_type.clone()),
        None => column_expr,
    }
}

const DECIMAL_PRECISION: usize = 38;
const DECIMAL_SCALE: usize = 18;

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
