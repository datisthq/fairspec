use polars::prelude::*;
use serde_json::Value;

/// Python's `frame.to_dicts()` for test assertions, by way of polars' JSON writer.
pub fn collect_records(table: LazyFrame) -> Value {
    let mut frame = table.collect().unwrap();
    let mut text = Vec::new();
    JsonWriter::new(&mut text)
        .with_json_format(JsonFormat::Json)
        .finish(&mut frame)
        .unwrap();
    serde_json::from_slice(&text).unwrap()
}
