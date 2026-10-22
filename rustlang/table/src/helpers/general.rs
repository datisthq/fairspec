use serde_json::Value;

pub fn get_is_object(value: &Value) -> bool {
    value.is_object()
}

/// A bound validates as a float, so 10 would otherwise render as 10.0.
pub fn stringify_number(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 {
        return format!("{:.0}", value + 0.0);
    }
    format_python_float(value)
}

/// Python's `repr(float)`: shortest round-trip digits, scientific below 1e-4 and from 1e16.
pub fn format_python_float(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "inf" } else { "-inf" }.to_string();
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
