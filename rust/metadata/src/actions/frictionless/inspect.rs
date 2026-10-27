use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use serde_json::Value;

use crate::models::frictionless::shape::Shape;

/// Parse a Frictionless descriptor, reporting every mismatch at its pointer, as zod does
pub fn inspect_frictionless_model(
    descriptor: &Descriptor,
    shape: &Shape,
    root_json_pointer: Option<&str>,
) -> (Option<Descriptor>, Vec<FairspecError>) {
    let root = match root_json_pointer {
        None | Some("/") => "",
        Some(root) => root,
    };
    let mut errors = Vec::new();
    inspect_shape(&Value::Object(descriptor.clone()), shape, root, &mut errors);

    match errors.is_empty() {
        true => (Some(descriptor.clone()), errors),
        false => (None, errors),
    }
}

fn inspect_shape(value: &Value, shape: &Shape, pointer: &str, errors: &mut Vec<FairspecError>) {
    match shape {
        Shape::Unknown => {}
        Shape::String if !value.is_string() => {
            push_error(errors, pointer, create_type_message("string", value))
        }
        Shape::Boolean if !value.is_boolean() => {
            push_error(errors, pointer, create_type_message("boolean", value))
        }
        Shape::Integer { minimum } => match get_integer(value) {
            None => push_error(errors, pointer, create_type_message("int", value)),
            Some(number) => {
                if let Some(minimum) = minimum
                    && number < *minimum as f64
                {
                    push_error(
                        errors,
                        pointer,
                        format!("Too small: expected number to be >={minimum}"),
                    );
                }
            }
        },
        Shape::Enum(options) => {
            if !value.as_str().is_some_and(|value| options.contains(&value)) {
                let options = options
                    .iter()
                    .map(|option| format!("\"{option}\""))
                    .collect::<Vec<_>>()
                    .join("|");
                push_error(
                    errors,
                    pointer,
                    format!("Invalid option: expected one of {options}"),
                );
            }
        }
        Shape::Array(item) => match value {
            Value::Array(items) => {
                for (index, value) in items.iter().enumerate() {
                    inspect_shape(value, item, &format!("{pointer}/{index}"), errors);
                }
            }
            _ => push_error(errors, pointer, create_type_message("array", value)),
        },
        Shape::Record if !value.is_object() => {
            push_error(errors, pointer, create_type_message("record", value))
        }
        Shape::Object(properties) => match value {
            Value::Object(object) => {
                for property in properties.iter() {
                    let pointer = format!("{pointer}/{}", escape_pointer(property.name));
                    match object.get(property.name) {
                        Some(value) => inspect_shape(value, &property.shape, &pointer, errors),
                        None if property.required => push_error(
                            errors,
                            &pointer,
                            "Invalid input: expected a value, received undefined".to_string(),
                        ),
                        None => {}
                    }
                }
            }
            _ => push_error(errors, pointer, create_type_message("object", value)),
        },
        Shape::Union(options) => {
            let matches = options.iter().any(|option| {
                let mut option_errors = Vec::new();
                inspect_shape(value, option, pointer, &mut option_errors);
                option_errors.is_empty()
            });
            if !matches {
                push_error(errors, pointer, "Invalid input".to_string());
            }
        }
        _ => {}
    }
}

fn push_error(errors: &mut Vec<FairspecError>, pointer: &str, message: String) {
    errors.push(FairspecError::Metadata(MetadataError {
        message,
        json_pointer: pointer.to_string(),
        ..MetadataError::default()
    }));
}

// zod's `z.int()` takes any safe integer, including a JSON `1.0`
fn get_integer(value: &Value) -> Option<f64> {
    const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
    let number = value.as_f64()?;
    (number.fract() == 0.0 && number.abs() <= MAX_SAFE_INTEGER).then_some(number)
}

fn create_type_message(expected: &str, value: &Value) -> String {
    let received = match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    };
    format!("Invalid input: expected {expected}, received {received}")
}

fn escape_pointer(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
