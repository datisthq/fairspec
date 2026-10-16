from __future__ import annotations

import polars as pl

from ..infer import infer_table_schema_from_table


class TestInferTableSchemaFromTable:
    def test_should_infer_from_native_types(self):
        table = pl.DataFrame(
            {
                "integer": pl.Series("integer", [1, 2], pl.Int32),
                "number": [1.1, 2.2],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "integer": {"type": "integer"},
                "number": {"type": "number"},
            },
        }

    def test_should_infer_integers_from_floats(self):
        table = pl.DataFrame(
            {
                "id": [1.0, 2.0, 3.0],
                "count": [10.0, 20.0, 30.0],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "id": {"type": "integer"},
                "count": {"type": "integer"},
            },
        }

    def test_should_infer_numeric(self):
        table = pl.DataFrame(
            {
                "name1": ["1", "2", "3"],
                "name2": ["1,000", "2,000", "3,000"],
                "name3": ["1.1", "2.2", "3.3"],
                "name4": ["1,000.1", "2,000.2", "3,000.3"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "integer"},
                "name2": {"type": "integer", "groupChar": ","},
                "name3": {"type": "number"},
                "name4": {"type": "number", "groupChar": ","},
            },
        }

    def test_should_infer_numeric_comma_decimal(self):
        table = pl.DataFrame(
            {
                "name1": ["1.000", "2.000", "3.000"],
                "name2": ["1.000,5", "2.000,5", "3.000,5"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table, commaDecimal=True)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "integer", "groupChar": "."},
                "name2": {"type": "number", "decimalChar": ",", "groupChar": "."},
            },
        }

    def test_should_infer_numeric_with_text(self):
        table = pl.DataFrame(
            {
                "integer": ["$10", "$20", "$30"],
                "percent": ["10%", "20%", "30%"],
                "number": ["$10.50", "$20.75", "$30.99"],
                "percentNumber": ["10.5%", "20.75%", "30.99%"],
                "integerGroupChar": ["$1,000", "$2,000", "$3,000"],
                "numberGroupChar": ["$1,000.50", "$2,000.75", "$3,000.99"],
                "european": ["\u20ac1.000,50", "\u20ac2.000,75", "\u20ac3.000,99"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "integer": {"type": "integer", "withText": True},
                "percent": {"type": "integer", "withText": True},
                "number": {"type": "number", "withText": True},
                "percentNumber": {"type": "number", "withText": True},
                "integerGroupChar": {
                    "type": "integer",
                    "groupChar": ",",
                    "withText": True,
                },
                "numberGroupChar": {
                    "type": "number",
                    "groupChar": ",",
                    "withText": True,
                },
                "european": {
                    "type": "number",
                    "groupChar": ".",
                    "decimalChar": ",",
                    "withText": True,
                },
            },
        }

    def test_should_not_infer_numeric_with_text_for_non_currency_text(self):
        table = pl.DataFrame(
            {
                "ordinal": ["1st", "2nd", "3rd"],
                "unit": ["2d", "5h", "10m"],
                "label": ["Level 5", "Level 10", "Level 15"],
                "hash": ["#10", "#20", "#30"],
                "mixed": ["5x", "10x", "15x"],
                "word": ["abc", "def", "ghi"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "ordinal": {"type": "string"},
                "unit": {"type": "string"},
                "label": {"type": "string"},
                "hash": {"type": "string"},
                "mixed": {"type": "string"},
                "word": {"type": "string"},
            },
        }

    def test_should_infer_booleans(self):
        table = pl.DataFrame(
            {
                "name1": ["true", "True", "TRUE"],
                "name2": ["false", "False", "FALSE"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "boolean"},
                "name2": {"type": "boolean"},
            },
        }

    def test_should_infer_objects(self):
        table = pl.DataFrame(
            {
                "name1": ['{"a": 1}'],
                "name2": ["{}"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "object"},
                "name2": {"type": "object"},
            },
        }

    def test_should_infer_arrays(self):
        table = pl.DataFrame(
            {
                "name1": ["[1,2,3]"],
                "name2": ["[]"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "array"},
                "name2": {"type": "array"},
            },
        }

    def test_should_infer_dates_with_iso_format(self):
        table = pl.DataFrame(
            {
                "name1": ["2023-01-15", "2023-02-20", "2023-03-25"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name1": {"type": "string", "format": "date"},
            },
        }

    def test_should_infer_dates_with_slash_format(self):
        table = pl.DataFrame(
            {
                "yearFirst": ["2023/01/15", "2023/02/20", "2023/03/25"],
                "dayMonth": ["15/01/2023", "20/02/2023", "25/03/2023"],
                "monthDay": ["01/15/2023", "02/20/2023", "03/25/2023"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "yearFirst": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%Y/%m/%d",
                },
                "dayMonth": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d/%m/%Y",
                },
                "monthDay": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d/%m/%Y",
                },
            },
        }

        month_first_result = infer_table_schema_from_table(table, monthFirst=True)
        assert month_first_result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "yearFirst": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%Y/%m/%d",
                },
                "dayMonth": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m/%d/%Y",
                },
                "monthDay": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m/%d/%Y",
                },
            },
        }

    def test_should_infer_dates_with_hyphen_format(self):
        table = pl.DataFrame(
            {
                "dayMonth": ["15-01-2023", "20-02-2023", "25-03-2023"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "dayMonth": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d-%m-%Y",
                },
            },
        }

        month_first_result = infer_table_schema_from_table(table, monthFirst=True)
        assert month_first_result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "dayMonth": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m-%d-%Y",
                },
            },
        }

    def test_should_infer_times_with_standard_format(self):
        table = pl.DataFrame(
            {
                "fullTime": ["14:30:45", "08:15:30", "23:59:59"],
                "shortTime": ["14:30", "08:15", "23:59"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "fullTime": {"type": "string", "format": "time"},
                "shortTime": {
                    "type": "string",
                    "format": "time",
                    "temporalFormat": "%H:%M",
                },
            },
        }

    def test_should_infer_times_with_12_hour_format(self):
        table = pl.DataFrame(
            {
                "fullTime": ["2:30:45 PM", "8:15:30 AM", "11:59:59 PM"],
                "shortTime": ["2:30 PM", "8:15 AM", "11:59 PM"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "fullTime": {
                    "type": "string",
                    "format": "time",
                    "temporalFormat": "%I:%M:%S %p",
                },
                "shortTime": {
                    "type": "string",
                    "format": "time",
                    "temporalFormat": "%I:%M %p",
                },
            },
        }

    def test_should_infer_times_with_timezone_offset(self):
        table = pl.DataFrame(
            {
                "name": ["14:30:45+01:00", "08:15:30-05:00", "23:59:59+00:00"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "name": {"type": "string", "format": "time"},
            },
        }

    def test_should_infer_datetimes_with_iso_format(self):
        table = pl.DataFrame(
            {
                "standard": [
                    "2023-01-15T14:30:45",
                    "2023-02-20T08:15:30",
                    "2023-03-25T23:59:59",
                ],
                "utc": [
                    "2023-01-15T14:30:45Z",
                    "2023-02-20T08:15:30Z",
                    "2023-03-25T23:59:59Z",
                ],
                "withTz": [
                    "2023-01-15T14:30:45+01:00",
                    "2023-02-20T08:15:30-05:00",
                    "2023-03-25T23:59:59+00:00",
                ],
                "withSpace": [
                    "2023-01-15 14:30:45",
                    "2023-02-20 08:15:30",
                    "2023-03-25 23:59:59",
                ],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "standard": {"type": "string", "format": "date-time"},
                "utc": {"type": "string", "format": "date-time"},
                "withTz": {"type": "string", "format": "date-time"},
                "withSpace": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%Y-%m-%d %H:%M:%S",
                },
            },
        }

    def test_should_infer_datetimes_with_custom_formats(self):
        table = pl.DataFrame(
            {
                "shortDayMonth": [
                    "15/01/2023 14:30",
                    "20/02/2023 08:15",
                    "25/03/2023 23:59",
                ],
                "fullDayMonth": [
                    "15/01/2023 14:30:45",
                    "20/02/2023 08:15:30",
                    "25/03/2023 23:59:59",
                ],
                "shortMonthDay": [
                    "01/15/2023 14:30",
                    "02/20/2023 08:15",
                    "03/25/2023 23:59",
                ],
                "fullMonthDay": [
                    "01/15/2023 14:30:45",
                    "02/20/2023 08:15:30",
                    "03/25/2023 23:59:59",
                ],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "shortDayMonth": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M",
                },
                "fullDayMonth": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M:%S",
                },
                "shortMonthDay": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M",
                },
                "fullMonthDay": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M:%S",
                },
            },
        }

        month_first_result = infer_table_schema_from_table(table, monthFirst=True)
        assert month_first_result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "shortDayMonth": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M",
                },
                "fullDayMonth": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M:%S",
                },
                "shortMonthDay": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M",
                },
                "fullMonthDay": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M:%S",
                },
            },
        }

    def test_should_infer_urls(self):
        table = pl.DataFrame(
            {
                "url": ["https://example.com", "http://foo.bar/baz"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "url": {"type": "string", "format": "url"},
            },
        }

    def test_should_infer_emails(self):
        table = pl.DataFrame(
            {
                "email": ["user@example.com", "test.name+tag@domain.org"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "email": {"type": "string", "format": "email"},
            },
        }

    def test_should_infer_wkt(self):
        table = pl.DataFrame(
            {
                "geom": ["POINT(1 2)", "LINESTRING(0 0, 1 1)"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "geom": {"type": "string", "format": "wkt"},
            },
        }

    def test_should_infer_durations(self):
        table = pl.DataFrame(
            {
                "duration": ["P1Y2M3D", "PT1H30M", "P1D"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "duration": {"type": "string", "format": "duration"},
            },
        }

    def test_should_infer_hex(self):
        table = pl.DataFrame(
            {
                "hex": ["1a2b3c4d5e6f7890", "abcdef0123456789"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "hex": {"type": "string", "format": "hex"},
            },
        }

    def test_should_not_infer_url_email_wkt_hex_for_similar_text(self):
        table = pl.DataFrame(
            {
                "notUrl": ["ftp://example.com", "ftp://other.com"],
                "notEmail": ["user@", "test@"],
                "notWkt": ["POINT", "LINESTRING"],
                "notHex": ["cafe1234", "deadbeef"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "notUrl": {"type": "string"},
                "notEmail": {"type": "string"},
                "notWkt": {"type": "string"},
                "notHex": {"type": "string"},
            },
        }

    def test_should_infer_lists(self):
        table = pl.DataFrame(
            {
                "numericList": ["1.5,2.3", "4.1,5.9", "7.2,8.6"],
                "integerList": ["1,2", "3,4", "5,6"],
                "singleValue": ["1.5", "2.3", "4.1"],
            }
        ).lazy()

        result = infer_table_schema_from_table(table)
        dumped = result.model_dump(by_alias=True, exclude_none=True)
        assert dumped == {
            "properties": {
                "numericList": {
                    "type": "string",
                    "format": "list",
                    "itemType": "number",
                },
                "integerList": {
                    "type": "string",
                    "format": "list",
                    "itemType": "integer",
                },
                "singleValue": {"type": "number"},
            },
        }


class TestInferTableSchemaFromTableNullable:
    def test_should_infer_nullable_string_from_missing_values(self):
        table = pl.DataFrame({"name": ["Alice", "Bob", "NA"]}).lazy()
        result = infer_table_schema_from_table(table)
        dumped = result.model_dump(by_alias=True, exclude_none=True)
        assert dumped["properties"] == {"name": {"type": ("string", "null")}}
        assert dumped["missingValues"] == ["NA"]

    def test_should_infer_nullable_integer_from_polars_nulls(self):
        table = pl.DataFrame({"value": pl.Series("value", [1, 2, None], pl.Int32)}).lazy()
        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {"value": {"type": ("integer", "null")}},
        }

    def test_should_infer_nullable_number_from_polars_nulls(self):
        table = pl.DataFrame(
            {"value": pl.Series("value", [1.1, None, 3.3], pl.Float64)}
        ).lazy()
        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {"value": {"type": ("number", "null")}},
        }

    def test_should_infer_nullable_url_with_missing_values(self):
        table = pl.DataFrame({"link": ["https://a.com", "http://b.com", "NA"]}).lazy()
        result = infer_table_schema_from_table(table)
        dumped = result.model_dump(by_alias=True, exclude_none=True)
        assert dumped["properties"] == {
            "link": {"type": ("string", "null"), "format": "url"}
        }
        assert dumped["missingValues"] == ["NA"]

    def test_should_infer_nullable_string_when_all_values_are_missing(self):
        table = pl.DataFrame({"empty": ["NA", "N/A", ""]}).lazy()
        result = infer_table_schema_from_table(table)
        dumped = result.model_dump(by_alias=True, exclude_none=True)
        assert dumped["properties"]["empty"] == {"type": ("string", "null")}
        assert result.missingValues is not None
        assert set(result.missingValues) == {"NA", "N/A", ""}
        assert len(result.missingValues) == 3

    def test_should_use_explicit_missing_values_option(self):
        table = pl.DataFrame({"name": ["Alice", "MISSING"]}).lazy()
        result = infer_table_schema_from_table(table, missingValues=["MISSING"])
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {"name": {"type": ("string", "null")}},
            "missingValues": ["MISSING"],
        }

    def test_should_not_make_columns_nullable_when_no_nulls_exist(self):
        table = pl.DataFrame({"name": ["Alice", "Bob"]}).lazy()
        result = infer_table_schema_from_table(table)
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {"name": {"type": "string"}},
        }

    def test_should_infer_nullable_integer_from_empty_string(self):
        table = pl.DataFrame({"value": ["1", "2", ""]}).lazy()
        result = infer_table_schema_from_table(table)
        dumped = result.model_dump(by_alias=True, exclude_none=True)
        assert dumped["properties"] == {"value": {"type": ("integer", "null")}}
        assert dumped["missingValues"] == [""]


class TestInferTableSchemaFromTableOptionsSteerDetection:
    def test_should_steer_boolean_detection_from_true_values_false_values(self):
        table = pl.DataFrame({"value": ["yes", "no", "yes"]}).lazy()
        result = infer_table_schema_from_table(
            table,
            trueValues=["yes"],
            falseValues=["no"],
        )
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {
                    "type": "boolean",
                    "trueValues": ["yes"],
                    "falseValues": ["no"],
                },
            },
        }

    def test_should_steer_number_detection_from_group_char(self):
        table = pl.DataFrame({"value": ["1.000", "2.000", "3.000"]}).lazy()
        result = infer_table_schema_from_table(table, groupChar=".")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {"value": {"type": "integer", "groupChar": "."}},
        }

    def test_should_steer_number_detection_from_decimal_char(self):
        table = pl.DataFrame({"value": ["1.000,5", "2.000,5"]}).lazy()
        result = infer_table_schema_from_table(table, decimalChar=",")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {"type": "number", "decimalChar": ",", "groupChar": "."},
            },
        }

    def test_should_steer_list_detection_from_list_delimiter(self):
        table = pl.DataFrame({"value": ["1;2", "3;4", "5;6"]}).lazy()
        result = infer_table_schema_from_table(table, listDelimiter=";")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {
                    "type": "string",
                    "format": "list",
                    "itemType": "integer",
                    "delimiter": ";",
                },
            },
        }

    def test_should_steer_date_detection_from_date_format(self):
        table = pl.DataFrame({"value": ["15/01/2023", "20/02/2023"]}).lazy()
        result = infer_table_schema_from_table(table, dateFormat="%d/%m/%Y")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d/%m/%Y",
                },
            },
        }

    def test_should_derive_month_first_from_date_format(self):
        table = pl.DataFrame({"value": ["01/15/2023", "02/20/2023"]}).lazy()
        result = infer_table_schema_from_table(table, dateFormat="%m/%d/%Y")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m/%d/%Y",
                },
            },
        }

    def test_should_filter_time_patterns_from_time_format(self):
        table = pl.DataFrame({"value": ["14:30", "08:15"]}).lazy()
        result = infer_table_schema_from_table(table, timeFormat="%H:%M")
        assert result.model_dump(by_alias=True, exclude_none=True) == {
            "properties": {
                "value": {
                    "type": "string",
                    "format": "time",
                    "temporalFormat": "%H:%M",
                },
            },
        }
