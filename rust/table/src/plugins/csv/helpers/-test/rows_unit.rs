use std::fs;

use fairspec_dataset::write_temp_file;
use fairspec_metadata::FileDialect;
use serde_json::{Value, json};

use crate::plugins::csv::helpers::rows::clean_csv_rows;

fn dialect(value: Value) -> FileDialect {
    serde_json::from_value(value).unwrap()
}

fn clean(text: &str, value: Value) -> Option<String> {
    let path = write_temp_file(text, false, None, None).unwrap();
    clean_csv_rows(&[path], &dialect(value), None)
        .unwrap()
        .map(|cleaned| fs::read_to_string(&cleaned.paths[0]).unwrap())
}

mod clean_csv_rows {
    use super::*;

    #[test]
    fn test_should_keep_files_without_a_blank_line_or_a_null_sequence() {
        assert!(clean("a,b\r\n1,2\r\n", json!({"format": "csv"})).is_none());
    }

    #[test]
    fn test_should_drop_blank_lines_but_not_one_inside_a_quoted_cell() {
        assert_eq!(
            clean("\na,b\n1,\"x\n\ny\"\n\r\n3,4\n\n", json!({"format": "csv"})).unwrap(),
            "a,b\n1,\"x\n\ny\"\n3,4\n"
        );
    }

    #[test]
    fn test_should_keep_a_row_of_delimiters() {
        assert_eq!(
            clean("a,b\n,\n\n", json!({"format": "csv"})).unwrap(),
            "a,b\n,\n"
        );
    }

    #[test]
    fn test_should_empty_only_unquoted_data_cells_equal_to_the_null_sequence() {
        let path =
            write_temp_file("NA,b\nNA,\"NA\"\n\"a\"\"NA\",NA\r\n", false, None, None).unwrap();
        let cleaned = clean_csv_rows(
            &[path],
            &dialect(json!({"format": "csv", "nullSequence": "NA"})),
            None,
        )
        .unwrap()
        .unwrap();
        let FileDialect::Csv(file_dialect) = &cleaned.file_dialect else {
            panic!("a csv dialect");
        };
        assert!(file_dialect.null_sequence.is_none());
        assert_eq!(
            fs::read_to_string(&cleaned.paths[0]).unwrap(),
            "NA,b\n,\"NA\"\n\"a\"\"NA\",\r\n"
        );
    }

    #[test]
    fn test_should_keep_the_quotes_of_tsv_cells() {
        assert_eq!(
            clean(
                "a\tb\n\"NA\tNA\"\n",
                json!({"format": "tsv", "nullSequence": "NA"})
            )
            .unwrap(),
            "a\tb\n\"NA\tNA\"\n"
        );
    }

    #[test]
    fn test_should_not_read_quotes_in_a_comment_row() {
        assert_eq!(
            clean(
                "a\n# it's\n\nx\n",
                json!({"format": "csv", "quoteChar": "'", "commentPrefix": "#"})
            )
            .unwrap(),
            "a\n# it's\nx\n"
        );
    }
}
