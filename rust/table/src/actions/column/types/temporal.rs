use polars::prelude::*;

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
