use serde_json::Value;

/// Python's `json.dumps(value, indent=2, ensure_ascii=False)`, which `--json` output and
/// `--llms` manifests are written with; floats keep Python's `repr` (`1e-05`, `1e+16`).
pub fn dump_json(value: &Value) -> String {
    let mut text = String::new();
    write_value(&mut text, value, 0);
    text
}

fn write_value(text: &mut String, value: &Value, depth: usize) {
    match value {
        Value::Null => text.push_str("null"),
        Value::Bool(value) => text.push_str(if *value { "true" } else { "false" }),
        Value::Number(number) => match (number.as_i64(), number.as_u64(), number.as_f64()) {
            (Some(integer), _, _) => text.push_str(&integer.to_string()),
            (_, Some(integer), _) => text.push_str(&integer.to_string()),
            (_, _, Some(float)) => text.push_str(&format_python_float(float)),
            _ => text.push_str(&number.to_string()),
        },
        Value::String(string) => write_string(text, string),
        Value::Array(items) if items.is_empty() => text.push_str("[]"),
        Value::Array(items) => {
            text.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    text.push(',');
                }
                write_newline(text, depth + 1);
                write_value(text, item, depth + 1);
            }
            write_newline(text, depth);
            text.push(']');
        }
        Value::Object(object) if object.is_empty() => text.push_str("{}"),
        Value::Object(object) => {
            text.push('{');
            for (index, (key, item)) in object.iter().enumerate() {
                if index > 0 {
                    text.push(',');
                }
                write_newline(text, depth + 1);
                write_string(text, key);
                text.push_str(": ");
                write_value(text, item, depth + 1);
            }
            write_newline(text, depth);
            text.push('}');
        }
    }
}

fn write_newline(text: &mut String, depth: usize) {
    text.push('\n');
    text.push_str(&"  ".repeat(depth));
}

fn write_string(text: &mut String, string: &str) {
    text.push('"');
    for char in string.chars() {
        match char {
            '"' => text.push_str("\\\""),
            '\\' => text.push_str("\\\\"),
            '\n' => text.push_str("\\n"),
            '\r' => text.push_str("\\r"),
            '\t' => text.push_str("\\t"),
            '\u{08}' => text.push_str("\\b"),
            '\u{0c}' => text.push_str("\\f"),
            char if (char as u32) < 0x20 => text.push_str(&format!("\\u{:04x}", char as u32)),
            char => text.push(char),
        }
    }
    text.push('"');
}

/// Python's `repr(float)`: shortest round-trip digits, scientific below 1e-4 and from 1e16.
pub fn format_python_float(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let abs = value.abs();
    if abs != 0.0 && !(1e-4..1e16).contains(&abs) {
        let text = format!("{value:e}");
        let (mantissa, exponent) = text.split_once('e').unwrap_or((&text, "0"));
        let exponent: i32 = exponent.parse().unwrap_or(0);
        let sign = if exponent < 0 { '-' } else { '+' };
        return format!("{mantissa}e{sign}{:02}", exponent.abs());
    }
    let text = format!("{value}");
    if text.contains('.') {
        text
    } else {
        format!("{text}.0")
    }
}
