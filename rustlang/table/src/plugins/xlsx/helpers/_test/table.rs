use chrono::NaiveDate;

use crate::plugins::xlsx::helpers::sheet::SheetCell;
use crate::plugins::xlsx::helpers::table::format_sheet_cell;

fn format(cell: SheetCell) -> String {
    format_sheet_cell(Some(&cell)).unwrap()
}

#[test]
fn test_should_format_numbers_as_javascript_does() {
    let cases = [
        (1.0, "1"),
        (1e21, "1e+21"),
        (1e20, "100000000000000000000"),
        (1e-7, "1e-7"),
        (0.000001, "0.000001"),
        (-0.0, "0"),
        (123.456, "123.456"),
        (-1.5e-10, "-1.5e-10"),
        (0.1 + 0.2, "0.30000000000000004"),
        (5e-324, "5e-324"),
        (1.7976931348623157e308, "1.7976931348623157e+308"),
    ];
    for (value, text) in cases {
        assert_eq!(format(SheetCell::Number(value)), text, "{value}");
    }
}

#[test]
fn test_should_format_dates_as_iso() {
    let date = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
    let midnight = date.and_hms_opt(0, 0, 0).unwrap();
    let time = date.and_hms_milli_opt(3, 4, 5, 678).unwrap();
    assert_eq!(format(SheetCell::DateTime(midnight)), "2024-01-02T00:00:00");
    assert_eq!(format(SheetCell::DateTime(time)), "2024-01-02T03:04:05.678");
}

#[test]
fn test_should_format_booleans_in_lower_case() {
    assert_eq!(format(SheetCell::Boolean(true)), "true");
    assert_eq!(format(SheetCell::Boolean(false)), "false");
}
