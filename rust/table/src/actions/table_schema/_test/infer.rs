use chrono::{NaiveDate, NaiveTime};
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::models::schema::InferTableSchemaOptions;

fn infer(table: LazyFrame, options: Value) -> Value {
    let options: InferTableSchemaOptions = serde_json::from_value(options).unwrap();
    let schema = infer_table_schema_from_table(table, &options).unwrap();
    serde_json::to_value(schema).unwrap()
}

fn strings(columns: &[(&str, &[&str])]) -> LazyFrame {
    let columns: Vec<Column> = columns
        .iter()
        .map(|(name, values)| Column::new((*name).into(), *values))
        .collect();
    DataFrame::new_infer_height(columns).unwrap().lazy()
}

fn datetimes(time_zone: Option<TimeZone>) -> Column {
    let value = NaiveDate::from_ymd_opt(2024, 1, 15)
        .unwrap()
        .and_hms_opt(10, 30, 0)
        .unwrap();
    Column::new("".into(), [value])
        .cast(&DataType::Datetime(TimeUnit::Microseconds, time_zone))
        .unwrap()
}

mod from_table {
    use super::*;

    #[test]
    fn test_should_infer_from_native_types() {
        let table = df!(
            "integer" => Series::new("integer".into(), [1i32, 2]),
            "number" => [1.1, 2.2],
        )
        .unwrap()
        .lazy();
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "integer": {"type": "integer"},
                "number": {"type": "number"},
            }})
        );
    }

    fn create_temporal_table(with_time: bool) -> LazyFrame {
        let mut columns = vec![
            datetimes(Some(TimeZone::UTC)).with_name("zoned".into()),
            datetimes(None).with_name("naive".into()),
        ];
        if with_time {
            let time = NaiveTime::from_hms_opt(10, 30, 0).unwrap();
            columns.push(Column::new("time".into(), [time]));
        }
        DataFrame::new_infer_height(columns).unwrap().lazy()
    }

    #[test]
    fn test_should_infer_temporal_formats_from_native_types() {
        assert_eq!(
            infer(create_temporal_table(true), json!({})),
            json!({"properties": {
                "zoned": {"type": "string", "format": "date-time"},
                "naive": {"type": "string", "format": "local-date-time"},
                "time": {"type": "string", "format": "local-time"},
            }})
        );
    }

    #[test]
    fn test_should_apply_a_temporal_format_option_only_to_columns_of_its_kind() {
        assert_eq!(
            infer(
                create_temporal_table(false),
                json!({"datetimeFormat": "%Y-%m-%d %H:%M"})
            ),
            json!({"properties": {
                "zoned": {"type": "string", "format": "date-time"},
                "naive": {
                    "type": "string",
                    "format": "local-date-time",
                    "temporalFormat": "%Y-%m-%d %H:%M",
                },
            }})
        );
    }

    #[test]
    fn test_should_infer_integers_from_floats() {
        let table = df!("id" => [1.0, 2.0, 3.0], "count" => [10.0, 20.0, 30.0])
            .unwrap()
            .lazy();
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "id": {"type": "integer"},
                "count": {"type": "integer"},
            }})
        );
    }

    #[test]
    fn test_should_infer_numeric() {
        let table = strings(&[
            ("name1", &["1", "2", "3"]),
            ("name2", &["1,000", "2,000", "3,000"]),
            ("name3", &["1.1", "2.2", "3.3"]),
            ("name4", &["1,000.1", "2,000.2", "3,000.3"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "name1": {"type": "integer"},
                "name2": {"type": "integer", "groupChar": ","},
                "name3": {"type": "number"},
                "name4": {"type": "number", "groupChar": ","},
            }})
        );
    }

    #[test]
    fn test_should_infer_numeric_comma_decimal() {
        let table = strings(&[
            ("name1", &["1.000", "2.000", "3.000"]),
            ("name2", &["1.000,5", "2.000,5", "3.000,5"]),
        ]);
        assert_eq!(
            infer(table, json!({"commaDecimal": true})),
            json!({"properties": {
                "name1": {"type": "integer", "groupChar": "."},
                "name2": {"type": "number", "decimalChar": ",", "groupChar": "."},
            }})
        );
    }

    #[test]
    fn test_should_infer_numeric_with_text() {
        let table = strings(&[
            ("integer", &["$10", "$20", "$30"]),
            ("percent", &["10%", "20%", "30%"]),
            ("number", &["$10.50", "$20.75", "$30.99"]),
            ("percentNumber", &["10.5%", "20.75%", "30.99%"]),
            ("integerGroupChar", &["$1,000", "$2,000", "$3,000"]),
            ("numberGroupChar", &["$1,000.50", "$2,000.75", "$3,000.99"]),
            ("european", &["€1.000,50", "€2.000,75", "€3.000,99"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "integer": {"type": "integer", "withText": true},
                "percent": {"type": "integer", "withText": true},
                "number": {"type": "number", "withText": true},
                "percentNumber": {"type": "number", "withText": true},
                "integerGroupChar": {"type": "integer", "groupChar": ",", "withText": true},
                "numberGroupChar": {"type": "number", "groupChar": ",", "withText": true},
                "european": {
                    "type": "number",
                    "groupChar": ".",
                    "decimalChar": ",",
                    "withText": true,
                },
            }})
        );
    }

    #[test]
    fn test_should_not_infer_numeric_with_text_for_non_currency_text() {
        let table = strings(&[
            ("ordinal", &["1st", "2nd", "3rd"]),
            ("unit", &["2d", "5h", "10m"]),
            ("label", &["Level 5", "Level 10", "Level 15"]),
            ("hash", &["#10", "#20", "#30"]),
            ("mixed", &["5x", "10x", "15x"]),
            ("word", &["abc", "def", "ghi"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "ordinal": {"type": "string"},
                "unit": {"type": "string"},
                "label": {"type": "string"},
                "hash": {"type": "string"},
                "mixed": {"type": "string"},
                "word": {"type": "string"},
            }})
        );
    }

    #[test]
    fn test_should_infer_booleans() {
        let table = strings(&[
            ("name1", &["true", "True", "TRUE"]),
            ("name2", &["false", "False", "FALSE"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "name1": {"type": "boolean"},
                "name2": {"type": "boolean"},
            }})
        );
    }

    #[test]
    fn test_should_infer_objects() {
        let table = strings(&[("name1", &[r#"{"a": 1}"#]), ("name2", &["{}"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "name1": {"type": "object"},
                "name2": {"type": "object"},
            }})
        );
    }

    #[test]
    fn test_should_infer_arrays() {
        let table = strings(&[("name1", &["[1,2,3]"]), ("name2", &["[]"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "name1": {"type": "array"},
                "name2": {"type": "array"},
            }})
        );
    }

    #[test]
    fn test_should_infer_dates_with_iso_format() {
        let table = strings(&[("name1", &["2023-01-15", "2023-02-20", "2023-03-25"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"name1": {"type": "string", "format": "date"}}})
        );
    }

    fn date(format: &str) -> Value {
        json!({"type": "string", "format": "date", "temporalFormat": format})
    }

    fn local_date_time(format: &str) -> Value {
        json!({"type": "string", "format": "local-date-time", "temporalFormat": format})
    }

    #[test]
    fn test_should_infer_dates_with_slash_format() {
        let table = strings(&[
            ("yearFirst", &["2023/01/15", "2023/02/20", "2023/03/25"]),
            ("dayMonth", &["15/01/2023", "20/02/2023", "25/03/2023"]),
            ("monthDay", &["01/15/2023", "02/20/2023", "03/25/2023"]),
        ]);
        assert_eq!(
            infer(table.clone(), json!({})),
            json!({"properties": {
                "yearFirst": date("%Y/%m/%d"),
                "dayMonth": date("%d/%m/%Y"),
                "monthDay": date("%d/%m/%Y"),
            }})
        );
        assert_eq!(
            infer(table, json!({"monthFirst": true})),
            json!({"properties": {
                "yearFirst": date("%Y/%m/%d"),
                "dayMonth": date("%m/%d/%Y"),
                "monthDay": date("%m/%d/%Y"),
            }})
        );
    }

    #[test]
    fn test_should_infer_dates_with_hyphen_format() {
        let table = strings(&[("dayMonth", &["15-01-2023", "20-02-2023", "25-03-2023"])]);
        assert_eq!(
            infer(table.clone(), json!({})),
            json!({"properties": {"dayMonth": date("%d-%m-%Y")}})
        );
        assert_eq!(
            infer(table, json!({"monthFirst": true})),
            json!({"properties": {"dayMonth": date("%m-%d-%Y")}})
        );
    }

    #[test]
    fn test_should_infer_times_with_standard_format() {
        let table = strings(&[
            ("fullTime", &["14:30:45", "08:15:30", "23:59:59"]),
            ("shortTime", &["14:30", "08:15", "23:59"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "fullTime": {"type": "string", "format": "local-time"},
                "shortTime": {"type": "string", "format": "local-time", "temporalFormat": "%H:%M"},
            }})
        );
    }

    #[test]
    fn test_should_infer_times_with_12_hour_format() {
        let table = strings(&[
            ("fullTime", &["2:30:45 PM", "8:15:30 AM", "11:59:59 PM"]),
            ("shortTime", &["2:30 PM", "8:15 AM", "11:59 PM"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "fullTime": {
                    "type": "string",
                    "format": "local-time",
                    "temporalFormat": "%I:%M:%S %p",
                },
                "shortTime": {
                    "type": "string",
                    "format": "local-time",
                    "temporalFormat": "%I:%M %p",
                },
            }})
        );
    }

    #[test]
    fn test_should_infer_times_with_timezone_offset() {
        let table = strings(&[(
            "name",
            &["14:30:45+01:00", "08:15:30-05:00", "23:59:59+00:00"],
        )]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"name": {"type": "string", "format": "time"}}})
        );
    }

    #[test]
    fn test_should_infer_datetimes_with_iso_format() {
        let table = strings(&[
            (
                "standard",
                &[
                    "2023-01-15T14:30:45",
                    "2023-02-20T08:15:30",
                    "2023-03-25T23:59:59",
                ],
            ),
            (
                "utc",
                &[
                    "2023-01-15T14:30:45Z",
                    "2023-02-20T08:15:30Z",
                    "2023-03-25T23:59:59Z",
                ],
            ),
            (
                "withTz",
                &[
                    "2023-01-15T14:30:45+01:00",
                    "2023-02-20T08:15:30-05:00",
                    "2023-03-25T23:59:59+00:00",
                ],
            ),
            (
                "withSpace",
                &[
                    "2023-01-15 14:30:45",
                    "2023-02-20 08:15:30",
                    "2023-03-25 23:59:59",
                ],
            ),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "standard": {"type": "string", "format": "local-date-time"},
                "utc": {"type": "string", "format": "date-time"},
                "withTz": {"type": "string", "format": "date-time"},
                "withSpace": local_date_time("%Y-%m-%d %H:%M:%S"),
            }})
        );
    }

    #[test]
    fn test_should_infer_datetimes_with_custom_formats() {
        let table = strings(&[
            (
                "shortDayMonth",
                &["15/01/2023 14:30", "20/02/2023 08:15", "25/03/2023 23:59"],
            ),
            (
                "fullDayMonth",
                &[
                    "15/01/2023 14:30:45",
                    "20/02/2023 08:15:30",
                    "25/03/2023 23:59:59",
                ],
            ),
            (
                "shortMonthDay",
                &["01/15/2023 14:30", "02/20/2023 08:15", "03/25/2023 23:59"],
            ),
            (
                "fullMonthDay",
                &[
                    "01/15/2023 14:30:45",
                    "02/20/2023 08:15:30",
                    "03/25/2023 23:59:59",
                ],
            ),
        ]);
        assert_eq!(
            infer(table.clone(), json!({})),
            json!({"properties": {
                "shortDayMonth": local_date_time("%d/%m/%Y %H:%M"),
                "fullDayMonth": local_date_time("%d/%m/%Y %H:%M:%S"),
                "shortMonthDay": local_date_time("%d/%m/%Y %H:%M"),
                "fullMonthDay": local_date_time("%d/%m/%Y %H:%M:%S"),
            }})
        );
        assert_eq!(
            infer(table, json!({"monthFirst": true})),
            json!({"properties": {
                "shortDayMonth": local_date_time("%m/%d/%Y %H:%M"),
                "fullDayMonth": local_date_time("%m/%d/%Y %H:%M:%S"),
                "shortMonthDay": local_date_time("%m/%d/%Y %H:%M"),
                "fullMonthDay": local_date_time("%m/%d/%Y %H:%M:%S"),
            }})
        );
    }

    #[test]
    fn test_should_infer_urls() {
        let table = strings(&[("url", &["https://example.com", "http://foo.bar/baz"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"url": {"type": "string", "format": "url"}}})
        );
    }

    #[test]
    fn test_should_infer_emails() {
        let table = strings(&[("email", &["user@example.com", "test.name+tag@domain.org"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"email": {"type": "string", "format": "email"}}})
        );
    }

    #[test]
    fn test_should_infer_wkt() {
        let table = strings(&[("geom", &["POINT(1 2)", "LINESTRING(0 0, 1 1)"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"geom": {"type": "string", "format": "wkt"}}})
        );
    }

    #[test]
    fn test_should_infer_durations() {
        let table = strings(&[("duration", &["P1Y2M3D", "PT1H30M", "P1D"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"duration": {"type": "string", "format": "duration"}}})
        );
    }

    #[test]
    fn test_should_infer_hex() {
        let table = strings(&[("hex", &["1a2b3c4d5e6f7890", "abcdef0123456789"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"hex": {"type": "string", "format": "hex"}}})
        );
    }

    #[test]
    fn test_should_not_infer_url_email_wkt_hex_for_similar_text() {
        let table = strings(&[
            ("notUrl", &["ftp://example.com", "ftp://other.com"]),
            ("notEmail", &["user@", "test@"]),
            ("notWkt", &["POINT", "LINESTRING"]),
            ("notHex", &["cafe1234", "deadbeef"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "notUrl": {"type": "string"},
                "notEmail": {"type": "string"},
                "notWkt": {"type": "string"},
                "notHex": {"type": "string"},
            }})
        );
    }

    #[test]
    fn test_should_infer_lists() {
        let table = strings(&[
            ("numericList", &["1.5,2.3", "4.1,5.9", "7.2,8.6"]),
            ("integerList", &["1,2", "3,4", "5,6"]),
            ("singleValue", &["1.5", "2.3", "4.1"]),
        ]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {
                "numericList": {"type": "string", "format": "list", "itemType": "number"},
                "integerList": {"type": "string", "format": "list", "itemType": "integer"},
                "singleValue": {"type": "number"},
            }})
        );
    }
}

mod from_table_nullable {
    use super::*;

    #[test]
    fn test_should_infer_nullable_string_from_missing_values() {
        let table = strings(&[("name", &["Alice", "Bob", "NA"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({
                "properties": {"name": {"type": ["string", "null"]}},
                "missingValues": ["NA"],
            })
        );
    }

    #[test]
    fn test_should_infer_nullable_integer_from_polars_nulls() {
        let table = df!("value" => [Some(1i32), Some(2), None]).unwrap().lazy();
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"value": {"type": ["integer", "null"]}}})
        );
    }

    #[test]
    fn test_should_infer_nullable_number_from_polars_nulls() {
        let table = df!("value" => [Some(1.1f64), None, Some(3.3)])
            .unwrap()
            .lazy();
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"value": {"type": ["number", "null"]}}})
        );
    }

    #[test]
    fn test_should_infer_nullable_url_with_missing_values() {
        let table = strings(&[("link", &["https://a.com", "http://b.com", "NA"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({
                "properties": {"link": {"type": ["string", "null"], "format": "url"}},
                "missingValues": ["NA"],
            })
        );
    }

    #[test]
    fn test_should_infer_nullable_string_when_all_values_are_missing() {
        let table = strings(&[("empty", &["NA", "N/A", ""])]);
        let result = infer(table, json!({}));
        assert_eq!(
            result["properties"]["empty"],
            json!({"type": ["string", "null"]})
        );
        let mut missing_values: Vec<&str> = result["missingValues"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        missing_values.sort();
        assert_eq!(missing_values, ["", "N/A", "NA"]);
    }

    #[test]
    fn test_should_use_explicit_missing_values_option() {
        let table = strings(&[("name", &["Alice", "MISSING"])]);
        assert_eq!(
            infer(table, json!({"missingValues": ["MISSING"]})),
            json!({
                "properties": {"name": {"type": ["string", "null"]}},
                "missingValues": ["MISSING"],
            })
        );
    }

    #[test]
    fn test_should_not_make_columns_nullable_when_no_nulls_exist() {
        let table = strings(&[("name", &["Alice", "Bob"])]);
        assert_eq!(
            infer(table, json!({})),
            json!({"properties": {"name": {"type": "string"}}})
        );
    }

    #[test]
    fn test_should_infer_nullable_integer_from_empty_string() {
        let table = strings(&[("value", &["1", "2", ""])]);
        assert_eq!(
            infer(table, json!({})),
            json!({
                "properties": {"value": {"type": ["integer", "null"]}},
                "missingValues": [""],
            })
        );
    }
}

mod from_table_options_steer_detection {
    use super::*;

    #[test]
    fn test_should_steer_boolean_detection_from_true_values_false_values() {
        let table = strings(&[("value", &["yes", "no", "yes"])]);
        assert_eq!(
            infer(table, json!({"trueValues": ["yes"], "falseValues": ["no"]})),
            json!({"properties": {"value": {
                "type": "boolean",
                "trueValues": ["yes"],
                "falseValues": ["no"],
            }}})
        );
    }

    #[test]
    fn test_should_steer_number_detection_from_group_char() {
        let table = strings(&[("value", &["1.000", "2.000", "3.000"])]);
        assert_eq!(
            infer(table, json!({"groupChar": "."})),
            json!({"properties": {"value": {"type": "integer", "groupChar": "."}}})
        );
    }

    #[test]
    fn test_should_steer_number_detection_from_decimal_char() {
        let table = strings(&[("value", &["1.000,5", "2.000,5"])]);
        assert_eq!(
            infer(table, json!({"decimalChar": ","})),
            json!({"properties": {
                "value": {"type": "number", "decimalChar": ",", "groupChar": "."},
            }})
        );
    }

    #[test]
    fn test_should_steer_list_detection_from_list_delimiter() {
        let table = strings(&[("value", &["1;2", "3;4", "5;6"])]);
        assert_eq!(
            infer(table, json!({"listDelimiter": ";"})),
            json!({"properties": {"value": {
                "type": "string",
                "format": "list",
                "itemType": "integer",
                "delimiter": ";",
            }}})
        );
    }

    #[test]
    fn test_should_steer_date_detection_from_date_format() {
        let table = strings(&[("value", &["15/01/2023", "20/02/2023"])]);
        assert_eq!(
            infer(table, json!({"dateFormat": "%d/%m/%Y"})),
            json!({"properties": {"value": {
                "type": "string",
                "format": "date",
                "temporalFormat": "%d/%m/%Y",
            }}})
        );
    }

    #[test]
    fn test_should_derive_month_first_from_date_format() {
        let table = strings(&[("value", &["01/15/2023", "02/20/2023"])]);
        assert_eq!(
            infer(table, json!({"dateFormat": "%m/%d/%Y"})),
            json!({"properties": {"value": {
                "type": "string",
                "format": "date",
                "temporalFormat": "%m/%d/%Y",
            }}})
        );
    }

    #[test]
    fn test_should_filter_time_patterns_from_time_format() {
        let table = strings(&[("value", &["14:30", "08:15"])]);
        assert_eq!(
            infer(table, json!({"timeFormat": "%H:%M"})),
            json!({"properties": {"value": {
                "type": "string",
                "format": "local-time",
                "temporalFormat": "%H:%M",
            }}})
        );
    }
}

mod from_table_column_properties {
    use super::*;

    #[test]
    fn test_should_use_a_typed_override_verbatim_and_skip_refinement() {
        let table = strings(&[("id", &["1", "2", "NA"]), ("name", &["a", "b", "c"])]);
        assert_eq!(
            infer(
                table,
                json!({"columnProperties": {"id": {"type": "string", "format": "categorical"}}})
            ),
            json!({"properties": {
                "id": {"type": "string", "format": "categorical"},
                "name": {"type": "string"},
            }})
        );
    }

    #[test]
    fn test_should_merge_an_untyped_override_over_the_inferred_property() {
        let table = strings(&[("id", &["1", "", "3"])]);
        assert_eq!(
            infer(
                table,
                json!({"columnProperties": {"id": {"title": "Identifier", "description": "Row id"}}})
            ),
            json!({
                "properties": {"id": {
                    "type": ["integer", "null"],
                    "title": "Identifier",
                    "description": "Row id",
                }},
                "missingValues": [""],
            })
        );
    }

    #[test]
    fn test_should_take_precedence_over_column_types() {
        let table = df!("id" => [1i64, 2]).unwrap().lazy();
        assert_eq!(
            infer(
                table,
                json!({
                    "columnTypes": {"id": "number"},
                    "columnProperties": {"id": {"type": "string"}},
                })
            ),
            json!({"properties": {"id": {"type": "string"}}})
        );
    }

    #[test]
    fn test_should_throw_for_a_column_not_in_the_table() {
        let table = df!("id" => [1i64, 2]).unwrap().lazy();
        let options: InferTableSchemaOptions =
            serde_json::from_value(json!({"columnProperties": {"missing": {"type": "string"}}}))
                .unwrap();
        let error = infer_table_schema_from_table(table, &options).unwrap_err();
        assert!(
            error
                .to_string()
                .contains(r#"Column "missing" not found in the table"#)
        );
    }
}
