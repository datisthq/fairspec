use fairspec_metadata::{Error, Result};

pub fn parse_json_pointer(pointer: &str) -> Result<Vec<String>> {
    if pointer.is_empty() {
        return Ok(vec![]);
    }
    let Some(rest) = pointer.strip_prefix('/') else {
        return Err(Error::Parse(format!("Invalid JSON Pointer: {pointer}")));
    };
    Ok(rest
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect())
}

pub fn format_json_pointer(tokens: &[String]) -> String {
    tokens
        .iter()
        .map(|token| format!("/{}", token.replace('~', "~0").replace('/', "~1")))
        .collect()
}
