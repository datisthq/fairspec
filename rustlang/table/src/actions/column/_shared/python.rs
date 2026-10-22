use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone as _, Timelike};
use polars::prelude::{AnyValue, TimeUnit, TimeZone};
use serde_json::Value;

use crate::helpers::general::format_python_float;

/// Python's `str(value)` of a JSON value: `None`, `True`, `1.0`, a dict as its repr.
pub fn stringify_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => represent_value(other),
    }
}

/// Python's `json.dumps(value)`: `", "` and `": "` separators, non-ASCII escaped.
pub fn dump_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(number) => stringify_json_number(number),
        Value::String(text) => quote_json_text(text),
        Value::Array(items) => {
            let items: Vec<String> = items.iter().map(dump_json).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(object) => {
            let entries: Vec<String> = object
                .iter()
                .map(|(key, value)| format!("{}: {}", quote_json_text(key), dump_json(value)))
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
    }
}

/// Python's `str(cell)` of a polars cell as `to_list` hands it over; null is empty.
pub fn stringify_cell(value: &AnyValue) -> String {
    match value {
        AnyValue::Null => String::new(),
        AnyValue::String(text) => text.to_string(),
        AnyValue::StringOwned(text) => text.to_string(),
        AnyValue::Boolean(value) => stringify_bool(*value).to_string(),
        AnyValue::Float32(value) => format_python_float(f64::from(*value)),
        AnyValue::Float64(value) => format_python_float(*value),
        AnyValue::Date(days) => NaiveDate::default()
            .checked_add_signed(chrono::Duration::days(i64::from(*days)))
            .map_or_else(String::new, |date| date.format("%Y-%m-%d").to_string()),
        AnyValue::Time(nanoseconds) => stringify_time(*nanoseconds),
        AnyValue::Datetime(value, unit, time_zone) => stringify_datetime(*value, *unit, *time_zone),
        AnyValue::DatetimeOwned(value, unit, time_zone) => {
            stringify_datetime(*value, *unit, time_zone.as_deref())
        }
        AnyValue::Duration(value, unit) => stringify_duration(*value, *unit),
        AnyValue::List(series) => {
            let items: Vec<String> = series.iter().map(|item| represent_cell(&item)).collect();
            format!("[{}]", items.join(", "))
        }
        other => match other.get_str() {
            Some(text) => text.to_string(),
            None => other.to_string(),
        },
    }
}

fn represent_value(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(value) => stringify_bool(*value).to_string(),
        Value::Number(number) => stringify_json_number(number),
        Value::String(text) => quote_python_text(text),
        Value::Array(items) => {
            let items: Vec<String> = items.iter().map(represent_value).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(object) => {
            let entries: Vec<String> = object
                .iter()
                .map(|(key, value)| {
                    format!("{}: {}", quote_python_text(key), represent_value(value))
                })
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
    }
}

fn represent_cell(value: &AnyValue) -> String {
    match value {
        AnyValue::Null => "None".to_string(),
        AnyValue::String(text) => quote_python_text(text),
        AnyValue::StringOwned(text) => quote_python_text(text),
        other => stringify_cell(other),
    }
}

fn stringify_bool(value: bool) -> &'static str {
    if value { "True" } else { "False" }
}

fn stringify_json_number(number: &serde_json::Number) -> String {
    if number.is_f64() {
        return format_python_float(number.as_f64().unwrap_or_default());
    }
    number.to_string()
}

fn stringify_time(nanoseconds: i64) -> String {
    let seconds = nanoseconds.div_euclid(1_000_000_000);
    let nanos = nanoseconds.rem_euclid(1_000_000_000);
    NaiveTime::from_num_seconds_from_midnight_opt(seconds as u32, nanos as u32)
        .map_or_else(String::new, format_python_time)
}

fn stringify_datetime(value: i64, unit: TimeUnit, time_zone: Option<&TimeZone>) -> String {
    let datetime = match unit {
        TimeUnit::Nanoseconds => DateTime::from_timestamp_nanos(value).naive_utc(),
        TimeUnit::Microseconds => match DateTime::from_timestamp_micros(value) {
            Some(datetime) => datetime.naive_utc(),
            None => return String::new(),
        },
        TimeUnit::Milliseconds => match DateTime::from_timestamp_millis(value) {
            Some(datetime) => datetime.naive_utc(),
            None => return String::new(),
        },
    };
    match time_zone.map(TimeZone::to_chrono) {
        Some(Ok(zone)) => {
            let local = zone.from_utc_datetime(&datetime);
            format!(
                "{}{}",
                format_python_datetime(local.naive_local()),
                local.format("%:z")
            )
        }
        _ => format_python_datetime(datetime),
    }
}

fn format_python_datetime(datetime: NaiveDateTime) -> String {
    format!(
        "{} {}",
        datetime.date().format("%Y-%m-%d"),
        format_python_time(datetime.time())
    )
}

fn format_python_time(time: NaiveTime) -> String {
    let microseconds = time.nanosecond() / 1000;
    if microseconds == 0 {
        return time.format("%H:%M:%S").to_string();
    }
    format!("{}.{microseconds:06}", time.format("%H:%M:%S"))
}

fn stringify_duration(value: i64, unit: TimeUnit) -> String {
    let microseconds = match unit {
        TimeUnit::Nanoseconds => value.div_euclid(1000),
        TimeUnit::Microseconds => value,
        TimeUnit::Milliseconds => value * 1000,
    };
    let days = microseconds.div_euclid(86_400_000_000);
    let rest = microseconds.rem_euclid(86_400_000_000);
    let seconds = rest / 1_000_000;
    let fraction = rest % 1_000_000;
    let mut text = format!(
        "{}:{:02}:{:02}",
        seconds / 3600,
        (seconds % 3600) / 60,
        seconds % 60
    );
    if fraction != 0 {
        text.push_str(&format!(".{fraction:06}"));
    }
    if days != 0 {
        let plural = if days.abs() == 1 { "" } else { "s" };
        text = format!("{days} day{plural}, {text}");
    }
    text
}

fn quote_python_text(text: &str) -> String {
    let quote = if text.contains('\'') && !text.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut quoted = String::from(quote);
    for char in text.chars() {
        match char {
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            char if char == quote => {
                quoted.push('\\');
                quoted.push(char);
            }
            char if (char as u32) < 0x20 || (0x7f..=0xa0).contains(&(char as u32)) => {
                quoted.push_str(&format!("\\x{:02x}", char as u32));
            }
            char => quoted.push(char),
        }
    }
    quoted.push(quote);
    quoted
}

fn quote_json_text(text: &str) -> String {
    let mut quoted = String::from('"');
    for char in text.chars() {
        match char {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            '\u{8}' => quoted.push_str("\\b"),
            '\u{c}' => quoted.push_str("\\f"),
            char if char.is_ascii() && (char as u32) >= 0x20 => quoted.push(char),
            char => {
                let mut units = [0u16; 2];
                for unit in char.encode_utf16(&mut units) {
                    quoted.push_str(&format!("\\u{unit:04x}"));
                }
            }
        }
    }
    quoted.push('"');
    quoted
}
