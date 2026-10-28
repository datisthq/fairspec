use polars::prelude::*;
use regex::Regex;

/// Python's `strptime(..., strict=False)`: unparsable text becomes null.
pub fn create_strptime_options(format: &str) -> StrptimeOptions {
    StrptimeOptions {
        format: Some(format.into()),
        strict: false,
        exact: true,
        cache: true,
    }
}

pub fn get_utc_datetime_type() -> DataType {
    DataType::Datetime(TimeUnit::Microseconds, Some(TimeZone::UTC))
}

const LEAP_SECOND_PATTERN: &str = "([0-9]{2}:[0-9]{2}:)60";
const SECOND_PATTERN: &str = r":[0-9]{2}(?:\.[0-9]+)?";
const PLAIN_SECOND_PATTERN: &str = ":[0-5][0-9]";

/// Reads an RFC 3339 default form: `T` and `Z` in any case, and a second `60` only at
/// 23:59 in UTC, read as 23:59:59.
pub fn parse_default_temporal(
    text_expr: Expr,
    pattern: &str,
    format: &str,
    data_type: DataType,
    prefix: &str,
) -> Expr {
    // Polars evaluates every branch on every cell, so the common form (uppercase, whole
    // seconds below 60) takes its fast parser alone and the full reading sees only nulls.
    // Each step is a struct field, read by name, so no step is evaluated twice
    let plain_pattern = pattern.replace(SECOND_PATTERN, PLAIN_SECOND_PATTERN);
    let plain_format = format.replace("%.f", "");
    let with_prefix = |expr: Expr| {
        if prefix.is_empty() {
            expr
        } else {
            concat_str([lit(prefix), expr], "", false)
        }
    };
    let strptime = |expr: Expr, format: &str| {
        expr.str().strptime(
            data_type.clone(),
            create_strptime_options(format),
            lit("raise"),
        )
    };

    let plain_text_expr = when(field("is_plain"))
        .then(field("text"))
        .otherwise(lit(NULL).cast(DataType::String));
    let matched_expr = when(field("is_plain").not())
        .then(field("text"))
        .otherwise(lit(NULL).cast(DataType::String))
        .str()
        .to_uppercase()
        .str()
        .extract(lit(pattern), 0);
    let value_expr = strptime(
        concat_str(
            [
                lit(prefix),
                field("matched")
                    .str()
                    .replace(lit(LEAP_SECOND_PATTERN), lit("${1}59"), false),
            ],
            "",
            false,
        ),
        format,
    );
    let is_kept_expr = field("matched")
        .str()
        .contains(lit(LEAP_SECOND_PATTERN), true)
        .not()
        .or(field("value")
            .dt()
            .hour()
            .eq(lit(23))
            .and(field("value").dt().minute().eq(lit(59))));

    as_struct(vec![text_expr.alias("text")])
        .struct_()
        .with_fields(vec![
            field("text")
                .str()
                .contains(lit(plain_pattern), true)
                .alias("is_plain"),
        ])
        .struct_()
        .with_fields(vec![
            strptime(with_prefix(plain_text_expr), &plain_format).alias("plain"),
            matched_expr.alias("matched"),
        ])
        .struct_()
        .with_fields(vec![value_expr.alias("value")])
        .struct_()
        .with_fields(vec![
            coalesce(&[
                field("plain"),
                when(is_kept_expr).then(field("value")).otherwise(lit(NULL)),
            ])
            .alias("value"),
        ])
        .struct_()
        .field_by_name("value")
}

fn field(name: &str) -> Expr {
    Expr::Field(Arc::from([PlSmallStr::from(name)]))
}

/// Nulls a value whose `%S` reads `60`, which Polars parses as a leap second.
pub fn reject_leap_second(text_expr: Expr, value_expr: Expr, format: &str) -> Expr {
    let Some(pattern) = create_seconds_pattern(format) else {
        return value_expr;
    };
    let seconds_expr = text_expr.str().extract(lit(pattern), 1);
    when(seconds_expr.eq_missing(lit("60")).not())
        .then(value_expr)
        .otherwise(lit(NULL))
}

fn create_seconds_pattern(format: &str) -> Option<String> {
    let token_regex = Regex::new(r"(?s)%[.:]?.|[^%]").expect("valid regex");
    let tokens: Vec<&str> = token_regex
        .find_iter(format)
        .map(|token| token.as_str())
        .collect();
    if !tokens.contains(&"%S") {
        return None;
    }

    let parts: String = tokens
        .iter()
        .map(|token| match *token {
            // Polars reads at most 4 digits of an unsigned year, so `%Y%m%d%H%M%S` splits like it
            "%Y" => r"(?:[+-]\d+|\d{1,4})".to_string(),
            "%j" => r"\d{1,3}".to_string(),
            "%S" => r"(\d{1,2})".to_string(),
            "%.f" => r"(?:\.\d+)?".to_string(),
            "%z" | "%:z" => r"[+-]\d{2}:?\d{2}".to_string(),
            "%b" | "%B" | "%p" => "[a-z]+".to_string(),
            "%%" => "%".to_string(),
            token if token.starts_with('%') => r"\d{1,2}".to_string(),
            token => regex::escape(token),
        })
        .collect();
    Some(format!("(?i)^{parts}$"))
}
