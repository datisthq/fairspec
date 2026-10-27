use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::models::table::LoadTableOptions;
use crate::plugins::_shared::collect_records;
use crate::plugins::csv::actions::table::load::load_csv_table;

fn write(body: &str) -> String {
    write_temp_file(body, false, None, None).unwrap()
}

fn load(data: Value, dialect: Value) -> Value {
    let resource: Resource =
        serde_json::from_value(json!({"data": data, "fileDialect": dialect})).unwrap();
    collect_records(load_csv_table(&resource, &Default::default()).unwrap())
}

fn csv(dialect: Value) -> Value {
    let mut dialect = dialect;
    dialect["format"] = json!("csv");
    dialect
}

/// Serves `body` on 127.0.0.1 to every connection, standing in for the recorded remote file.
fn serve(body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap() == 0 || line.trim().is_empty() {
                    break;
                }
            }
            let head = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(head.as_bytes()).unwrap();
            stream.write_all(body.as_bytes()).unwrap();
        }
    });
    url
}

mod csv {
    use super::*;

    #[test]
    fn test_should_load_local_file() {
        assert_eq!(
            load(json!(write("id,name\n1,english\n2,中文")), csv(json!({}))),
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }

    #[test]
    fn test_should_load_local_file_multipart() {
        let paths = json!([
            write("id,name\n1,english"),
            write("id,name\n2,中文\n3,german")
        ]);
        assert_eq!(
            load(paths, csv(json!({}))),
            json!([
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文"},
                {"id": 3, "name": "german"},
            ])
        );
    }

    #[test]
    fn test_should_load_remote_file_with_preview_bytes() {
        let url = serve("id,name\n1,english\n2,中文\n");
        let resource: Resource =
            serde_json::from_value(json!({"data": format!("{url}/table.csv")})).unwrap();
        let options = LoadTableOptions {
            preview_bytes: Some(18),
            ..Default::default()
        };
        assert_eq!(
            collect_records(load_csv_table(&resource, &options).unwrap()),
            json!([{"id": 1, "name": "english"}])
        );
    }

    #[test]
    fn test_should_handle_custom_delimiter() {
        assert_eq!(
            load(
                json!(write("id|name\n1|alice\n2|bob")),
                csv(json!({"delimiter": "|"}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_handle_files_without_header() {
        assert_eq!(
            load(
                json!(write("1,alice\n2,bob")),
                csv(json!({"headerRows": false}))
            ),
            json!([
                {"column1": 1, "column2": "alice"},
                {"column1": 2, "column2": "bob"},
            ])
        );
    }

    #[test]
    fn test_should_handle_files_without_header_using_column_names() {
        assert_eq!(
            load(
                json!(write("1,alice\n2,bob")),
                csv(json!({"headerRows": false, "columnNames": ["id", "name"]}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_take_default_header_rows_when_dialect_declares_only_format() {
        assert_eq!(
            load(json!(write("1,100\n2,200\n3,300")), csv(json!({}))),
            json!([{"1": 2, "100": 200}, {"1": 3, "100": 300}])
        );
    }

    #[test]
    fn test_should_handle_custom_line_terminator() {
        assert_eq!(
            load(
                json!(write("id,name|1,alice|2,bob")),
                csv(json!({"lineTerminator": "|"}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_handle_crlf_line_terminator() {
        assert_eq!(
            load(
                json!(write("id,name\r\n1,alice\r\n2,bob\r\n")),
                csv(json!({"lineTerminator": "\r\n"}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_handle_custom_quote_character() {
        assert_eq!(
            load(
                json!(write("id,name\n1,'alice smith'\n2,'bob jones'")),
                csv(json!({"quoteChar": "'"}))
            ),
            json!([{"id": 1, "name": "alice smith"}, {"id": 2, "name": "bob jones"}])
        );
    }

    #[test]
    fn test_should_handle_comment_character() {
        assert_eq!(
            load(
                json!(write(
                    "# This is a comment\nid,name\n1,alice\n# Another comment\n2,bob"
                )),
                csv(json!({"commentPrefix": "#"}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_support_header_rows() {
        assert_eq!(
            load(
                json!(write("#comment\nid,name\n1,alice\n2,bob")),
                csv(json!({"headerRows": [2]}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_support_header_join() {
        assert_eq!(
            load(
                json!(write("#comment\nid,name\nint,str\n1,alice\n2,bob")),
                csv(json!({"headerRows": [2, 3], "headerJoin": "_"}))
            ),
            json!([
                {"id_int": 1, "name_str": "alice"},
                {"id_int": 2, "name_str": "bob"},
            ])
        );
    }

    #[test]
    fn test_should_support_comment_rows() {
        assert_eq!(
            load(
                json!(write("id,name\n1,alice\ncomment\n2,bob")),
                csv(json!({"commentRows": [3]}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_support_header_rows_and_comment_rows() {
        assert_eq!(
            load(
                json!(write("#comment\nid,name\n1,alice\n#comment\n2,bob")),
                csv(json!({"headerRows": [2], "commentRows": [4]}))
            ),
            json!([{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}])
        );
    }

    #[test]
    fn test_should_number_comment_rows_before_comment_prefix_removes_rows() {
        assert_eq!(
            load(
                json!(write("id,name\n#x,y\na,b\nskip,me\nc,d\n")),
                csv(json!({"commentRows": [4], "commentPrefix": "#"}))
            ),
            json!([{"id": "a", "name": "b"}, {"id": "c", "name": "d"}])
        );
    }

    #[test]
    fn test_should_support_header_join_and_comment_rows() {
        assert_eq!(
            load(
                json!(write(
                    "#comment\nid,name\nint,str\n1,alice\n#comment\n2,bob"
                )),
                csv(json!({"headerRows": [2, 3], "headerJoin": "_", "commentRows": [5]}))
            ),
            json!([
                {"id_int": 1, "name_str": "alice"},
                {"id_int": 2, "name_str": "bob"},
            ])
        );
    }

    #[test]
    fn test_should_handle_null_sequence() {
        assert_eq!(
            load(
                json!(write("id,name,age\n1,alice,25\n2,N/A,30\n3,bob,N/A")),
                csv(json!({"nullSequence": "N/A"}))
            ),
            json!([
                {"id": 1, "name": "alice", "age": 25},
                {"id": 2, "name": null, "age": 30},
                {"id": 3, "name": "bob", "age": null},
            ])
        );
    }

    #[test]
    fn test_should_handle_multiple_format_options_together() {
        assert_eq!(
            load(
                json!(write(
                    "#comment\nid|'full name'|age\n1|'alice smith'|25\n2|'bob jones'|30"
                )),
                csv(json!({
                    "delimiter": "|",
                    "quoteChar": "'",
                    "commentPrefix": "#",
                    "headerRows": [1],
                }))
            ),
            json!([
                {"id": 1, "full name": "alice smith", "age": 25},
                {"id": 2, "full name": "bob jones", "age": 30},
            ])
        );
    }
}

mod tsv {
    use super::*;

    #[test]
    fn test_should_load_local_file() {
        assert_eq!(
            load(
                json!(write("id\tname\n1\tenglish\n2\t中文")),
                json!({"format": "tsv"})
            ),
            json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
        );
    }
}
