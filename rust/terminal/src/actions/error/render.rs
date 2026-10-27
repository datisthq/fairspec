use fairspec_library::fairspec_metadata::FairspecError;
use serde_json::Value;

use crate::helpers::json::format_python_float;

/// A validation error as one human line in console markup.
pub fn render_error(error: &FairspecError) -> String {
    let value = serde_json::to_value(error).unwrap_or_default();
    let field = |name: &str| value.get(name).cloned().unwrap_or(Value::Null);
    let b = |name: &str| bold(&field(name));
    let in_resource = in_resource(&field("resourceName"));
    let check_message = match field("message") {
        Value::Null => String::new(),
        Value::String(message) if message.is_empty() => String::new(),
        message => format!(" ({})", stringify_python(&message)),
    };
    let cell = || {
        format!(
            "the cell {} in column {} of row {}",
            b("cell"),
            b("columnName"),
            b("rowNumber")
        )
    };
    let list = |name: &str| match field(name) {
        Value::Array(items) => items.iter().map(bold).collect::<Vec<_>>().join(", "),
        _ => String::new(),
    };
    let message = || stringify_python(&field("message"));

    match field("type").as_str().unwrap_or_default() {
        "cell/type" => format!(
            "Value of {} is not {}{in_resource}",
            cell(),
            b("columnType")
        ),
        "cell/missing" => format!(
            "A cell in column {} of row {} is missing{in_resource}",
            b("columnName"),
            b("rowNumber")
        ),
        "cell/minimum" => format!(
            "Value of {} is less than {}{in_resource}",
            cell(),
            b("minimum")
        ),
        "cell/maximum" => format!(
            "Value of {} is more than {}{in_resource}",
            cell(),
            b("maximum")
        ),
        "cell/exclusiveMinimum" => format!(
            "Value of {} is less or equal to {}{in_resource}",
            cell(),
            b("minimum")
        ),
        "cell/exclusiveMaximum" => format!(
            "Value of {} is greater or equal to {}{in_resource}",
            cell(),
            b("maximum")
        ),
        "cell/multipleOf" => format!(
            "Value of {} is not a multiple of {}{in_resource}",
            cell(),
            b("multipleOf")
        ),
        "cell/minLength" => format!(
            "Length of {} is less than {}{in_resource}",
            cell(),
            b("minLength")
        ),
        "cell/maxLength" => format!(
            "Length of {} is more than {}{in_resource}",
            cell(),
            b("maxLength")
        ),
        "cell/pattern" => format!(
            "Value of {} does not match the {}{in_resource}",
            cell(),
            b("pattern")
        ),
        "cell/unique" => format!("Value of {} is not unique{in_resource}", cell()),
        "cell/const" => format!(
            "Value of {} is not allowed value {}{in_resource}",
            cell(),
            b("const")
        ),
        "cell/enum" => format!(
            "Value of {} is not in the allowed values {}{in_resource}",
            cell(),
            list("enum")
        ),
        "cell/json" => format!(
            "Value of {} violates JSON schema at {}: {}{in_resource}",
            cell(),
            b("jsonPointer"),
            message()
        ),
        "cell/minItems" => format!(
            "Value of {} has less than {} items{in_resource}",
            cell(),
            b("minItems")
        ),
        "cell/maxItems" => format!(
            "Value of {} has more than {} items{in_resource}",
            cell(),
            b("maxItems")
        ),
        "cell/check" => format!(
            "Value of {} fails check {}{check_message}{in_resource}",
            cell(),
            b("checkName")
        ),
        "check" => format!(
            "Check {} cannot be evaluated{in_resource}: {}",
            b("checkName"),
            message()
        ),
        "column/missing" => format!(
            "Required column {} is missing{in_resource}",
            b("columnName")
        ),
        "column/extra" => format!(
            "Column {} is not in the schema{in_resource}",
            b("columnName")
        ),
        "column/type" => format!(
            "Column {} is expected to be {}, but it is {}{in_resource}",
            b("columnName"),
            b("expectedColumnType"),
            b("actualColumnType")
        ),
        "data" => format!(
            "Data error at {}: {}{in_resource}",
            b("jsonPointer"),
            message()
        ),
        "file/textual" => {
            let encoding = match field("actualEncoding") {
                Value::Null => "binary".to_string(),
                Value::String(encoding) if encoding.is_empty() => "binary".to_string(),
                encoding => bold(&encoding),
            };
            format!(
                "File is expected to be textual with utf-8 encoding but it is {encoding}{in_resource}"
            )
        }
        "file/integrity" => format!(
            "File hash {} is expected to be {}, but it is {}){in_resource}",
            b("hashType"),
            b("expectedHash"),
            b("actualHash")
        ),
        "file/header" => match field("columnNames") {
            Value::Array(names) if !names.is_empty() => format!(
                "File part {} has header {}, which differs from the header of part 1{in_resource}",
                b("partNumber"),
                list("columnNames")
            ),
            _ => format!("File part {} has no header{in_resource}", b("partNumber")),
        },
        "foreignKey" => format!(
            "Foreign key constraint violated as cells {} do not reference existing values{in_resource}",
            list("cells")
        ),
        "metadata" => format!("{} at {}{in_resource}", message(), b("jsonPointer")),
        "row/primaryKey" => format!(
            "Row {} violates primary key constraint on columns {}{in_resource}",
            b("rowNumber"),
            list("columnNames")
        ),
        "row/uniqueKey" => format!(
            "Row {} violates unique key constraint on columns {}{in_resource}",
            b("rowNumber"),
            list("columnNames")
        ),
        "row/check" => format!(
            "Row {} fails check {}{check_message}{in_resource}",
            b("rowNumber"),
            b("checkName")
        ),
        "table/check" => format!(
            "Table fails check {}{check_message}{in_resource}",
            b("checkName")
        ),
        "resource/missing" => format!(
            "Resource {} is missing, but expected{}",
            b("resourceName"),
            self::in_resource(&field("referencingResourceName"))
        ),
        "resource/type" => format!(
            "Resource {} is expected to be {}{}",
            b("resourceName"),
            b("expectedResourceType"),
            self::in_resource(&field("referencingResourceName"))
        ),
        _ => stringify_python(&value),
    }
}

fn bold(value: &Value) -> String {
    format!("[bold]{}[/bold]", stringify_python(value))
}

fn in_resource(resource_name: &Value) -> String {
    match resource_name {
        Value::Null => String::new(),
        Value::String(name) if name.is_empty() => String::new(),
        name => format!(" in resource {}", bold(name)),
    }
}

// Python's `str()` of a field value
fn stringify_python(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::String(text) => text.clone(),
        Value::Number(number) if number.is_f64() => {
            format_python_float(number.as_f64().unwrap_or_default())
        }
        Value::Number(number) => number.to_string(),
        other => other.to_string(),
    }
}
