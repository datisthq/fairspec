use fairspec_library::fairspec_metadata::{Error, Result};
use fairspec_library::fairspec_table::Frame;
use fairspec_library::fairspec_table::polars::prelude::*;
use serde_json::{Map, Number, Value};

/// The rows as pydantic's JSON mode writes `frame.to_dicts()`: temporal values as ISO
/// text (microseconds only when set, `Z` for a zero offset), NaN and infinity as null.
pub fn get_frame_records(frame: &Frame) -> Result<Vec<Value>> {
    let exprs: Vec<Expr> = frame
        .schema()
        .iter()
        .map(|(name, dtype)| create_record_expr(col(name.clone()), dtype))
        .collect();
    let rendered = frame
        .clone()
        .lazy()
        .select(exprs)
        .collect()
        .map_err(to_error)?;

    let names: Vec<String> = rendered
        .get_column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();
    let mut records = Vec::with_capacity(rendered.height());
    for index in 0..rendered.height() {
        let row = rendered.get_row(index).map_err(to_error)?;
        let record: Map<String, Value> = names
            .iter()
            .zip(&row.0)
            .map(|(name, value)| (name.clone(), get_record_value(value)))
            .collect();
        records.push(Value::Object(record));
    }
    Ok(records)
}

/// Python polars' `frame.describe()`, with `statistic` renamed `#` as the CLI shows it.
pub fn describe_frame(frame: &Frame) -> Result<Frame> {
    const METRICS: [&str; 9] = [
        "count",
        "null_count",
        "mean",
        "std",
        "min",
        "25%",
        "50%",
        "75%",
        "max",
    ];
    const QUANTILES: [f64; 3] = [0.25, 0.5, 0.75];

    let schema = frame.schema().clone();
    let mut exprs = Vec::new();
    for (name, dtype) in schema.iter() {
        let column = col(name.clone());
        let is_numeric = dtype.is_primitive_numeric() || matches!(dtype, DataType::Decimal(_, _));
        let is_temporal = !is_numeric && dtype.is_temporal();
        let skip_minmax = dtype.is_nested()
            || matches!(
                dtype,
                DataType::Categorical(_, _) | DataType::Enum(_, _) | DataType::Null
            );
        let null = || lit(NULL);

        exprs.push(column.clone().count().cast(DataType::Float64));
        exprs.push(column.clone().null_count().cast(DataType::Float64));
        exprs.push(
            if is_numeric || is_temporal || *dtype == DataType::Boolean {
                column.clone().mean()
            } else {
                null()
            },
        );
        exprs.push(if is_numeric {
            column.clone().std(1)
        } else {
            null()
        });
        exprs.push(if skip_minmax {
            null()
        } else {
            column.clone().min()
        });
        for quantile in QUANTILES {
            exprs.push(if is_temporal {
                column
                    .clone()
                    .sort(Default::default())
                    .to_physical()
                    .quantile(lit(quantile), QuantileMethod::Nearest)
                    .cast(dtype.clone())
            } else if is_numeric {
                column
                    .clone()
                    .sort(Default::default())
                    .quantile(lit(quantile), QuantileMethod::Nearest)
            } else {
                null()
            });
        }
        exprs.push(if skip_minmax { null() } else { column.max() });
    }
    let exprs: Vec<Expr> = exprs
        .into_iter()
        .enumerate()
        .map(|(index, expr)| expr.alias(format!("fairspec:metric:{index}")))
        .collect();
    let metrics = frame
        .clone()
        .lazy()
        .select(exprs)
        .collect()
        .map_err(to_error)?;
    let row = metrics.get_row(0).map_err(to_error)?;

    let mut columns = vec![Column::new("#".into(), METRICS.to_vec())];
    for (index, (name, dtype)) in schema.iter().enumerate() {
        let values = &row.0[index * METRICS.len()..(index + 1) * METRICS.len()];
        let has_numeric_result = dtype.is_primitive_numeric()
            || matches!(
                dtype,
                DataType::Decimal(_, _) | DataType::Null | DataType::Boolean
            )
            || dtype.is_nested();
        if has_numeric_result {
            let numbers: Vec<Option<f64>> = values.iter().map(get_float).collect();
            columns.push(Column::new(name.clone(), numbers));
        } else {
            let texts: Vec<Option<String>> = values
                .iter()
                .enumerate()
                .map(|(metric, value)| match (metric, value) {
                    (_, AnyValue::Null) => None,
                    (0 | 1, value) => get_float(value).map(|number| format!("{}", number as i64)),
                    (_, value) => Some(stringify_python_value(value)),
                })
                .collect();
            columns.push(Column::new(name.clone(), texts));
        }
    }
    DataFrame::new_infer_height(columns).map_err(to_error)
}

fn create_record_expr(column: Expr, dtype: &DataType) -> Expr {
    let strip = |expr: Expr, pattern: &str, replacement: &str| {
        expr.str().replace(
            lit(pattern.to_string()),
            lit(replacement.to_string()),
            false,
        )
    };
    match dtype {
        DataType::Date => column.dt().strftime("%Y-%m-%d"),
        DataType::Time => strip(column.dt().strftime("%H:%M:%S%.6f"), r"\.000000$", ""),
        DataType::Datetime(_, None) => strip(
            column.dt().strftime("%Y-%m-%dT%H:%M:%S%.6f"),
            r"\.000000$",
            "",
        ),
        DataType::Datetime(_, Some(_)) => {
            let text = strip(
                column.dt().strftime("%Y-%m-%dT%H:%M:%S%.6f%:z"),
                r"\.000000([+-]\d\d:\d\d)$",
                "$1",
            );
            strip(text, r"\+00:00$", "Z")
        }
        DataType::Decimal(_, _) | DataType::Categorical(_, _) | DataType::Enum(_, _) => {
            column.cast(DataType::String)
        }
        _ => column,
    }
}

fn get_record_value(value: &AnyValue) -> Value {
    match value {
        AnyValue::Null => Value::Null,
        AnyValue::Boolean(value) => Value::Bool(*value),
        AnyValue::String(value) => Value::String(value.to_string()),
        AnyValue::StringOwned(value) => Value::String(value.to_string()),
        AnyValue::Float32(value) => get_float_value(f64::from(*value)),
        AnyValue::Float64(value) => get_float_value(*value),
        AnyValue::Binary(value) => Value::String(String::from_utf8_lossy(value).into_owned()),
        AnyValue::BinaryOwned(value) => Value::String(String::from_utf8_lossy(value).into_owned()),
        AnyValue::Duration(value, unit) => Value::String(format_duration(*value, *unit)),
        AnyValue::List(series) => {
            Value::Array(series.iter().map(|item| get_record_value(&item)).collect())
        }
        AnyValue::StructOwned(payload) => {
            let (values, fields) = payload.as_ref();
            Value::Object(
                fields
                    .iter()
                    .zip(values)
                    .map(|(field, value)| (field.name().to_string(), get_record_value(value)))
                    .collect(),
            )
        }
        value if value.is_integer() => match value.extract::<i64>() {
            Some(integer) => Value::from(integer),
            None => value.extract::<u64>().map_or(Value::Null, Value::from),
        },
        value => Value::String(value.to_string()),
    }
}

fn get_float_value(value: f64) -> Value {
    Number::from_f64(value).map_or(Value::Null, Value::Number)
}

fn get_float(value: &AnyValue) -> Option<f64> {
    match value {
        AnyValue::Null => None,
        AnyValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
        value => value.extract::<f64>(),
    }
}

// pydantic's ISO 8601 duration: `PT1M30S`, `P1DT2H`, `PT0S`
fn format_duration(value: i64, unit: TimeUnit) -> String {
    let per_second: i64 = match unit {
        TimeUnit::Nanoseconds => 1_000_000_000,
        TimeUnit::Microseconds => 1_000_000,
        TimeUnit::Milliseconds => 1_000,
    };
    let sign = if value < 0 { "-" } else { "" };
    let value = value.unsigned_abs() as i128;
    let total_seconds = value / per_second as i128;
    let fraction = value % per_second as i128;
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    let mut text = format!("{sign}P");
    if days > 0 {
        text.push_str(&format!("{days}D"));
    }
    let mut time = String::new();
    if hours > 0 {
        time.push_str(&format!("{hours}H"));
    }
    if minutes > 0 {
        time.push_str(&format!("{minutes}M"));
    }
    if seconds > 0 || fraction > 0 || (days == 0 && time.is_empty()) {
        if fraction > 0 {
            let digits = format!("{:0width$}", fraction, width = per_second.ilog10() as usize);
            time.push_str(&format!("{seconds}.{}S", digits.trim_end_matches('0')));
        } else {
            time.push_str(&format!("{seconds}S"));
        }
    }
    if !time.is_empty() {
        text.push('T');
        text.push_str(&time);
    }
    text
}

// Python's `str()` of a describe value: `2024-01-16 12:00:00`, microseconds when set
fn stringify_python_value(value: &AnyValue) -> String {
    match value {
        AnyValue::String(text) => text.to_string(),
        AnyValue::StringOwned(text) => text.to_string(),
        AnyValue::Boolean(value) => if *value { "True" } else { "False" }.to_string(),
        value => {
            let text = value.to_string();
            match text.split_once('.') {
                Some((head, fraction)) if head.contains(':') => {
                    let fraction = format!("{fraction:0<6}");
                    let fraction = &fraction[..6];
                    if fraction == "000000" {
                        head.to_string()
                    } else {
                        format!("{head}.{fraction}")
                    }
                }
                _ => text,
            }
        }
    }
}

fn to_error(error: PolarsError) -> Error {
    Error::Load(error.to_string())
}
