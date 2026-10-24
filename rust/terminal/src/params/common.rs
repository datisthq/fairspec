use serde::Serialize;

#[fireargs::input]
#[derive(Debug, Clone, Default)]
pub struct SelectParams {
    /// dataset to select resource from
    #[fireargs(short = 'd')]
    pub dataset: Option<String>,
    /// resource in provided dataset
    #[fireargs(short = 'r')]
    pub resource: Option<String>,
}

/// A choice's wire text, as the library's string options take it.
pub fn get_choice_text(choice: &impl Serialize) -> String {
    match serde_json::to_value(choice) {
        Ok(serde_json::Value::String(text)) => text,
        _ => String::new(),
    }
}

/// A comma-separated CLI list.
pub fn parse_list(value: &str) -> Result<Vec<String>, String> {
    Ok(value.split(',').map(str::to_string).collect())
}
