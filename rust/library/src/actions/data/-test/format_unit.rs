use crate::actions::data::format::DATA_SCHEMA_FORMATS;

fn check(name: &str, value: &str) -> bool {
    let (_, check) = DATA_SCHEMA_FORMATS
        .iter()
        .find(|(format, _)| *format == name)
        .unwrap();
    check(value)
}

#[test]
fn test_checks_a_value() {
    let cases = [
        ("date", "2024-02-29", true),
        ("date", "2023-02-29", false),
        ("date", "2024-13-01", false),
        ("time", "23:59:60Z", true),
        ("time", "15:59:60-08:00", true),
        ("time", "12:00:60Z", false),
        ("time", "12:00:00", false),
        ("date-time", "2024-01-31t10:00:00.5+01:00", true),
        ("date-time", "2024-01-31 10:00:00Z", false),
        ("date-time", "2024-01-3\u{e9}10:00:00Z", false),
        ("duration", "P1W", true),
        ("duration", "P", false),
        ("email", "joe@example.com", true),
        ("email", "joe@example.com\n", false),
        ("email", "joe", false),
        ("ipv4", "192.168.0.1", true),
        ("ipv4", "01.1.1.1", false),
        ("uuid", "123E4567-e89b-12d3-a456-426614174000", true),
        ("uuid", "123e4567e89b12d3a456426614174000", false),
    ];
    for (name, value, expected) in cases {
        assert_eq!(check(name, value), expected, "{name} {value:?}");
    }
}
