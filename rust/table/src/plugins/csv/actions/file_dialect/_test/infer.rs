use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::plugins::csv::actions::file_dialect::infer::infer_csv_file_dialect;

fn infer(body: &str, with_dialect: bool) -> Value {
    let path = write_temp_file(body, false, None, None).unwrap();
    let mut resource = json!({"data": path});
    if with_dialect {
        resource["fileDialect"] = json!({"format": "csv"});
    }
    let resource: Resource = serde_json::from_value(resource).unwrap();
    let dialect = infer_csv_file_dialect(&resource, &Default::default()).unwrap();
    serde_json::to_value(dialect).unwrap()
}

fn csv(delimiter: &str, quote_char: Option<&str>, header_rows: Value) -> Value {
    let mut dialect = json!({"format": "csv", "delimiter": delimiter});
    if let Some(quote_char) = quote_char {
        dialect["quoteChar"] = json!(quote_char);
    }
    dialect["headerRows"] = header_rows;
    dialect["lineTerminator"] = json!("\n");
    dialect
}

mod basic {
    use super::*;

    #[test]
    fn test_should_infer_simple_csv_file() {
        assert_eq!(
            infer("id,name\n1,english\n2,中文", false),
            csv(",", None, json!([1]))
        );
    }

    #[test]
    fn test_should_infer_quote_char() {
        assert_eq!(
            infer("id,name\n1,\"John Doe\"\n2,\"Jane Smith\"", false),
            csv(",", Some("\""), json!([1]))
        );
    }

    #[test]
    fn test_should_infer_quote_char_with_single_quotes() {
        assert_eq!(
            infer("id,name\n1,'John Doe'\n2,'Jane Smith'", false),
            csv(",", Some("'"), json!([1]))
        );
    }

    #[test]
    fn test_should_infer_header_false_when_no_header() {
        assert_eq!(
            infer("1,english\n2,中文\n3,español", false),
            csv(",", None, json!(false))
        );
    }

    #[test]
    fn test_should_detect_header_when_present() {
        assert_eq!(
            infer("id,name\n1,english\n2,中文", false),
            csv(",", None, json!([1]))
        );
    }

    #[test]
    fn test_should_infer_complex_csv_with_quotes_and_header() {
        let body = "name,description\n\"Product A\",\"A great product with, commas\"\n\"Product B\",\"Another product\"";
        assert_eq!(infer(body, false), csv(",", Some("\""), json!([1])));
    }
}

mod delimiters {
    use super::*;

    #[test]
    fn test_should_infer_comma_delimiter() {
        assert_eq!(
            infer("id,name,age\n1,alice,25\n2,bob,30", true),
            csv(",", None, json!([1]))
        );
    }

    #[test]
    fn test_should_infer_pipe_delimiter() {
        assert_eq!(
            infer("id|name|age\n1|alice|25\n2|bob|30", true),
            csv("|", None, json!([1]))
        );
    }

    #[test]
    fn test_should_infer_semicolon_delimiter() {
        assert_eq!(
            infer("id;name;age\n1;alice;25\n2;bob;30", true),
            csv(";", None, json!([1]))
        );
    }

    #[test]
    fn test_should_infer_tab_delimiter_as_tsv() {
        assert_eq!(
            infer("id\tname\tage\n1\talice\t25\n2\tbob\t30", true),
            json!({"format": "tsv", "headerRows": [1], "lineTerminator": "\n"})
        );
    }
}

mod quotes {
    use super::*;

    #[test]
    fn test_should_handle_quoted_fields() {
        let body = "id,name,description\n1,\"alice\",\"Description with, comma\"\n2,\"bob\",\"Normal text\"";
        assert_eq!(infer(body, true), csv(",", Some("\""), json!([1])));
    }

    #[test]
    fn test_should_handle_single_quote_character() {
        let body = "id,name,description\n1,'alice','Description text'\n2,'bob','Normal text'";
        assert_eq!(infer(body, true), csv(",", Some("'"), json!([1])));
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn test_should_return_none_for_resources_without_path() {
        let resource: Resource = serde_json::from_value(json!({
            "data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}],
        }))
        .unwrap();
        assert_eq!(
            infer_csv_file_dialect(&resource, &Default::default()).unwrap(),
            None
        );
    }

    #[test]
    fn test_should_handle_custom_line_terminator() {
        let mut expected = csv(",", None, json!([1]));
        expected["lineTerminator"] = json!("\r\n");
        assert_eq!(infer("id,name\r\n1,alice\r\n2,bob\r\n", true), expected);
    }

    #[test]
    fn test_should_handle_header_row_only() {
        assert_eq!(infer("id,name,age", true), csv(",", None, json!(false)));
    }

    #[test]
    fn test_should_handle_empty_file() {
        assert_eq!(
            infer("", true),
            json!({"format": "csv", "delimiter": ",", "lineTerminator": "\n"})
        );
    }
}

mod header_detection {
    use super::*;

    #[test]
    fn test_should_detect_header_with_mixed_types() {
        assert_eq!(
            infer("id,name,age\n1,Alice,25\n2,Bob,30\n3,Charlie,35", false),
            csv(",", None, json!([1]))
        );
    }

    #[test]
    fn test_should_detect_header_after_preamble_rows() {
        let body = "# Comment line 1\n# Comment line 2\nid,name,age\n1,Alice,25\n2,Bob,30";
        assert_eq!(infer(body, false), csv(",", None, json!([3])));
    }

    #[test]
    fn test_should_not_detect_header_when_first_row_is_numeric() {
        assert_eq!(
            infer("1,2,3\n4,5,6\n7,8,9", false),
            csv(",", None, json!(false))
        );
    }

    #[test]
    fn test_should_detect_header_with_underscores_and_mixed_case() {
        let body =
            "user_id,User_Name,EmailAddress\n1,alice,alice@example.com\n2,bob,bob@example.com";
        assert_eq!(infer(body, false), csv(",", None, json!([1])));
    }

    #[test]
    fn test_should_not_detect_header_when_first_row_has_data_like_values() {
        let body = "blsrpxedd,37257,695.80,false,1927-11-07T01:03:54Z\n\
                    zmvpq03o4,68694,337.73,false,1927-04-02T12:37:52Z\n\
                    iw1fm3k9n,52019,988.74,false,2009-02-22T05:50:15Z";
        assert_eq!(infer(body, false), csv(",", None, json!(false)));
    }
}
