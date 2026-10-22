use chrono::{NaiveDate, NaiveTime};
use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::Resource;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::models::table::{LoadTableOptions, SaveTableOptions};
use crate::plugins::_shared::collect_records;
use crate::plugins::csv::actions::table::load::load_csv_table;
use crate::plugins::csv::actions::table::save::save_csv_table;

fn create_table() -> LazyFrame {
    df!("id" => [1.0, 2.0, 3.0], "name" => ["Alice", "Bob", "Charlie"])
        .unwrap()
        .lazy()
}

fn save(table: LazyFrame, path: &str, dialect: Option<Value>) -> String {
    let options = SaveTableOptions {
        path: path.to_string(),
        file_dialect: dialect.map(|dialect| serde_json::from_value(dialect).unwrap()),
        ..Default::default()
    };
    save_csv_table(table, &options).unwrap();
    std::fs::read_to_string(path).unwrap()
}

fn save_temp(table: LazyFrame, dialect: Value) -> String {
    let path = get_temp_file_path(false, None, None).unwrap();
    save(table, &path, Some(dialect))
}

mod csv {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        assert_eq!(
            save_temp(create_table(), json!({"format": "csv"})),
            "id,name\n1.0,Alice\n2.0,Bob\n3.0,Charlie\n"
        );
    }

    #[test]
    fn test_should_save_with_custom_delimiter() {
        assert_eq!(
            save_temp(create_table(), json!({"format": "csv", "delimiter": ";"})),
            "id;name\n1.0;Alice\n2.0;Bob\n3.0;Charlie\n"
        );
    }

    #[test]
    fn test_should_save_without_header() {
        assert_eq!(
            save_temp(
                create_table(),
                json!({"format": "csv", "headerRows": false})
            ),
            "1.0,Alice\n2.0,Bob\n3.0,Charlie\n"
        );
    }

    #[test]
    fn test_should_save_with_custom_quote_char() {
        let table = df!(
            "id" => [1.0, 2.0, 3.0],
            "name" => ["Alice,Smith", "Bob,Jones", "Charlie,Brown"],
        )
        .unwrap()
        .lazy();
        assert_eq!(
            save_temp(table, json!({"format": "csv", "quoteChar": "'"})),
            "id,name\n1.0,'Alice,Smith'\n2.0,'Bob,Jones'\n3.0,'Charlie,Brown'\n"
        );
    }

    #[test]
    fn test_should_save_temporal_values_in_their_default_forms() {
        let path = get_temp_file_path(false, None, Some("csv")).unwrap();
        let value = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_milli_opt(10, 30, 0, 500)
            .unwrap();
        let naive = Column::new("naive".into(), [value])
            .cast(&DataType::Datetime(TimeUnit::Microseconds, None))
            .unwrap();
        let source = DataFrame::new_infer_height(vec![
            naive.clone().with_name("zoned".into()),
            naive,
            Column::new("time".into(), [NaiveTime::from_hms_opt(10, 30, 0).unwrap()]),
        ])
        .unwrap()
        .lazy()
        .with_column(col("zoned").dt().replace_time_zone(
            Some(TimeZone::opt_try_new(Some("Europe/Rome")).unwrap().unwrap()),
            lit("raise"),
            NonExistent::Raise,
        ));
        assert_eq!(
            save(source, &path, None),
            "zoned,naive,time\n2025-01-01T09:30:00.500Z,2025-01-01T10:30:00.500,10:30:00\n"
        );
    }

    #[test]
    fn test_should_save_and_load_various_data_types() {
        let path = get_temp_file_path(false, None, Some("csv")).unwrap();
        let source = df!(
            "boolean" => [true],
            "date" => ["2025-01-01"],
            "datetime" => ["2025-01-01T00:00:00"],
            "integer" => [1i32],
            "number" => [1.1],
            "string" => ["string"],
        )
        .unwrap()
        .lazy();
        save(source, &path, None);

        let resource: Resource = serde_json::from_value(json!({"data": path})).unwrap();
        let options = LoadTableOptions {
            denormalized: Some(true),
            ..Default::default()
        };
        assert_eq!(
            collect_records(load_csv_table(&resource, &options).unwrap()),
            json!([{
                "boolean": "true",
                "date": "2025-01-01",
                "datetime": "2025-01-01T00:00:00",
                "integer": "1",
                "number": "1.1",
                "string": "string",
            }])
        );
    }
}

mod tsv {
    use super::*;

    #[test]
    fn test_should_save_table_to_file() {
        assert_eq!(
            save_temp(create_table(), json!({"format": "tsv"})),
            "id\tname\n1.0\tAlice\n2.0\tBob\n3.0\tCharlie\n"
        );
    }
}
