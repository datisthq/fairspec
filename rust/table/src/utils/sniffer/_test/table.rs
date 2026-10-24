use crate::utils::sniffer::metadata::{LineTerminator, Quote};
use crate::utils::sniffer::potential_dialects::PotentialDialect;
use crate::utils::sniffer::table::Table;

fn create_dialect(quote: Option<u32>, line_terminator: LineTerminator) -> PotentialDialect {
    PotentialDialect {
        delimiter: 44,
        quote: Quote { char: quote },
        line_terminator,
    }
}

fn create_rows(rows: &[&[&str]]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| row.iter().map(|field| field.to_string()).collect())
        .collect()
}

#[test]
fn test_parse_comma_delimited() {
    let table = Table::parse(
        b"a,b,c\n1,2,3\n4,5,6",
        &create_dialect(None, LineTerminator::Lf),
    );
    assert_eq!(
        table.rows,
        create_rows(&[&["a", "b", "c"], &["1", "2", "3"], &["4", "5", "6"]])
    );
    assert_eq!(table.field_counts, vec![3, 3, 3]);
}

#[test]
fn test_parse_quoted_fields() {
    let dialect = create_dialect(Some(34), LineTerminator::Lf);
    let table = Table::parse(b"a,\"b,c\",d\n1,\"2,3\",4", &dialect);
    assert_eq!(
        table.rows,
        create_rows(&[&["a", "b,c", "d"], &["1", "2,3", "4"]])
    );
}

#[test]
fn test_escaped_quotes() {
    let dialect = create_dialect(Some(34), LineTerminator::Lf);
    let table = Table::parse(b"a,\"b\"\"c\",d", &dialect);
    assert_eq!(table.rows, create_rows(&[&["a", "b\"c", "d"]]));
}

#[test]
fn test_modal_field_count() {
    let rows = create_rows(&[
        &["a", "b", "c"],
        &["1", "2", "3"],
        &["4", "5"],
        &["7", "8", "9"],
    ]);
    let table = Table::new(rows, vec![3, 3, 2, 3]);
    assert_eq!(table.get_modal_field_count(), 3);
}

#[test]
fn test_cache_modal_field_count() {
    let table = Table::new(create_rows(&[&["a", "b"], &["1", "2"]]), vec![2, 2]);
    let first = table.get_modal_field_count();
    let second = table.get_modal_field_count();
    assert_eq!(first, second);
    assert_eq!(first, 2);
}

#[test]
fn test_detect_uniform() {
    let table = Table::new(
        create_rows(&[&["a", "b"], &["1", "2"], &["3", "4"]]),
        vec![2, 2, 2],
    );
    assert!(table.is_uniform());
}

#[test]
fn test_detect_non_uniform() {
    let rows = create_rows(&[&["a", "b"], &["1", "2", "3"], &["4", "5"]]);
    assert!(!Table::new(rows, vec![2, 3, 2]).is_uniform());
}

#[test]
fn test_num_rows() {
    let table = Table::new(create_rows(&[&["a"], &["b"], &["c"]]), vec![1, 1, 1]);
    assert_eq!(table.num_rows(), 3);
}

#[test]
fn test_empty_table() {
    let table = Table::new(vec![], vec![]);
    assert_eq!(table.num_rows(), 0);
    assert_eq!(table.get_modal_field_count(), 0);
    assert!(table.is_uniform());
}

#[test]
fn test_parse_crlf() {
    let table = Table::parse(
        b"a,b\r\n1,2\r\n3,4",
        &create_dialect(None, LineTerminator::Crlf),
    );
    assert_eq!(table.rows.len(), 3);
}

#[test]
fn test_parse_cr() {
    let table = Table::parse(b"a,b\r1,2\r3,4", &create_dialect(None, LineTerminator::Cr));
    assert_eq!(table.rows.len(), 3);
}

#[test]
fn test_modal_field_count_wide_tables() {
    let rows: Vec<Vec<String>> = (0..300)
        .map(|i| vec!["x".to_string(); if i % 2 == 0 { 260 } else { 250 }])
        .collect();
    let field_counts = rows.iter().map(Vec::len).collect();
    assert_eq!(Table::new(rows, field_counts).get_modal_field_count(), 260);
}
