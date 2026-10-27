use fairspec_metadata::{Error, Result};
use serde_json::Value;

pub fn decode_json_buffer(data: &[u8], is_lines: bool) -> Result<Value> {
    let data = data.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(data);
    let text = std::str::from_utf8(data).map_err(|error| Error::Parse(error.to_string()))?;
    let parse = |text: &str| -> Result<Value> {
        serde_json::from_str(text).map_err(|error| Error::Parse(error.to_string()))
    };
    if !is_lines {
        return parse(text);
    }
    text.split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .filter(|line| !line.is_empty())
        .map(parse)
        .collect::<Result<Vec<_>>>()
        .map(Value::Array)
}
