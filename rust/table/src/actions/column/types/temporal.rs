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
