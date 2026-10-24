use crate::plugins::sqlite::actions::database::quote::quote_identifier;

#[test]
fn test_should_wrap_a_name_in_double_quotes() {
    assert_eq!(quote_identifier("name"), "\"name\"");
}

#[test]
fn test_should_double_embedded_double_quotes() {
    assert_eq!(quote_identifier("a\"b"), "\"a\"\"b\"");
}
