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

/// Reads an RFC 3339 default form: `T` and `Z` in any case, and a second `60` only at
/// 23:59 in UTC, read as 23:59:59.
pub fn parse_default_temporal(
    text_expr: Expr,
    pattern: &str,
    format: &str,
    data_type: DataType,
    prefix: &str,
) -> Expr {
    let matched_expr = text_expr
        .str()
        .to_uppercase()
        .str()
        .extract(lit(pattern), 0);
    let value_expr = concat_str(
        [
            lit(prefix),
            matched_expr
                .clone()
                .str()
                .replace(lit(LEAP_SECOND_PATTERN), lit("${1}59"), false),
        ],
        "",
        false,
    )
    .str()
    .strptime(data_type, create_strptime_options(format), lit("raise"));
    let is_leap_second_expr = matched_expr.str().contains(lit(LEAP_SECOND_PATTERN), true);
    let is_last_minute_expr = value_expr
        .clone()
        .dt()
        .hour()
        .eq(lit(23))
        .and(value_expr.clone().dt().minute().eq(lit(59)));
    when(is_leap_second_expr.not().or(is_last_minute_expr))
        .then(value_expr)
        .otherwise(lit(NULL))
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
