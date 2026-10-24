use crate::actions::descriptor::parse::parse_descriptor;

#[test]
fn test_parses_object() {
    let descriptor = parse_descriptor(r#"{"name": "test"}"#).unwrap();
    assert_eq!(descriptor["name"], "test");
}

#[test]
fn test_rejects_non_object_without_echoing_input() {
    for (text, kind) in [
        (r#"["secret", "token"]"#, "array"),
        (r#""secret token""#, "string"),
        ("42", "number"),
        ("1.5", "number"),
        ("true", "boolean"),
        ("null", "null"),
    ] {
        assert_eq!(
            parse_descriptor(text).unwrap_err().to_string(),
            format!("Invalid descriptor: expected a JSON object, got {kind}")
        );
    }
}
