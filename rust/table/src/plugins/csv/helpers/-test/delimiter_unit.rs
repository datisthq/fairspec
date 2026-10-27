use std::fs;

use fairspec_dataset::write_temp_file;
use fairspec_metadata::FileDialect;
use serde_json::json;

use crate::plugins::csv::helpers::delimiter::{CHUNK_SIZE, DelimiterSwap, swap_delimiter};

fn csv(delimiter: &str) -> FileDialect {
    serde_json::from_value(json!({"format": "csv", "delimiter": delimiter})).unwrap()
}

mod swap_delimiter {
    use super::*;

    #[test]
    fn test_should_keep_a_single_byte_delimiter() {
        let path = write_temp_file("a;b\n", false, None, None).unwrap();
        assert!(swap_delimiter(&[path], &csv(";"), None).unwrap().is_none());
    }

    #[test]
    fn test_should_swap_a_delimiter_cut_by_a_chunk_boundary() {
        let text = format!("{}¦b\n", "a".repeat(CHUNK_SIZE - 1));
        let path = write_temp_file(&text, false, None, None).unwrap();
        let swapped = swap_delimiter(&[path], &csv("¦"), None).unwrap().unwrap();

        assert_eq!(
            swapped.swap,
            DelimiterSwap {
                delimiter: "¦".to_string(),
                placeholder: "\x1f".to_string()
            }
        );
        assert_eq!(
            fs::read_to_string(&swapped.paths[0]).unwrap(),
            text.replace('¦', "\x1f")
        );
    }

    #[test]
    fn test_should_take_a_control_byte_the_file_does_not_hold() {
        let path = write_temp_file("a¦\x1f\n", false, None, None).unwrap();
        let swapped = swap_delimiter(&[path], &csv("¦"), None).unwrap().unwrap();

        assert_eq!(swapped.swap.placeholder, "\x1e");
    }

    #[test]
    fn test_should_read_no_further_than_max_bytes() {
        let path = write_temp_file("a¦b\n\x1f¦c\n", false, None, None).unwrap();
        let swapped = swap_delimiter(&[path], &csv("¦"), Some(5))
            .unwrap()
            .unwrap();

        assert_eq!(swapped.swap.placeholder, "\x1f");
        assert_eq!(fs::read_to_string(&swapped.paths[0]).unwrap(), "a\x1fb\n");
    }
}
