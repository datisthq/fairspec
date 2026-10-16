from __future__ import annotations

import math
import re
from typing import TYPE_CHECKING, Unpack

import polars as pl
from fairspec_metadata import create_column_from_property, get_column_properties
from fairspec_metadata import set_property_nullable
from fairspec_metadata import TableSchema

from fairspec_table.helpers.schema import get_polars_schema
from fairspec_table.models.schema import InferTableSchemaOptions

if TYPE_CHECKING:
    from fairspec_metadata import Column
    from fairspec_metadata import Descriptor

    from fairspec_table.models.table import Table

DEFAULT_MISSING_VALUES = ["", "NA", "N/A", "null", "-"]

TYPE_MAPPING: dict[type[pl.DataType], str] = {
    pl.Boolean: "boolean",
    pl.Categorical: "string",
    pl.Date: "date",
    pl.Datetime: "date-time",
    pl.Decimal: "number",
    pl.Float32: "number",
    pl.Float64: "number",
    pl.Int8: "integer",
    pl.Int16: "integer",
    pl.Int32: "integer",
    pl.Int64: "integer",
    pl.UInt8: "integer",
    pl.UInt16: "integer",
    pl.UInt32: "integer",
    pl.UInt64: "integer",
    pl.List: "array",
    pl.Null: "unknown",
    pl.String: "string",
    pl.Struct: "object",
    pl.Time: "time",
}


def infer_table_schema_from_table(
    table: Table,
    **options: Unpack[InferTableSchemaOptions],
) -> TableSchema:
    sample_rows = options.get("sampleRows") or 100
    sample: pl.DataFrame = table.head(sample_rows).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    return infer_table_schema_from_sample(sample, **options)


def infer_table_schema_from_sample(
    sample: pl.DataFrame,
    **options: Unpack[InferTableSchemaOptions],
) -> TableSchema:
    confidence = options.get("confidence") or 0.9
    column_types = options.get("columnTypes")
    keep_strings = options.get("keepStrings")
    effective_missing_values = (
        options["missingValues"]
        if options.get("missingValues") is not None
        else DEFAULT_MISSING_VALUES
    )
    detected_missing_values: set[str] = set()

    regex_mapping = _create_regex_mapping(**options)

    polars_schema = get_polars_schema(sample.schema)
    column_names = options.get("columnNames") or [c.name for c in polars_schema.columns]

    failure_threshold = (sample.height - math.floor(sample.height * confidence)) or 1

    columns: list[Column] = []
    for name in column_names:
        polars_column = next((c for c in polars_schema.columns if c.name == name), None)
        if not polars_column:
            raise ValueError(f'Column "{name}" not found in the table')

        col_type = (
            (column_types.get(name) if column_types else None)
            or TYPE_MAPPING.get(polars_column.type)
            or "unknown"
        )
        is_nullable = False

        property_dict: Descriptor = _create_property_dict(col_type)
        effective_col_type = col_type

        if not (column_types and name in column_types):
            if col_type == "array":
                if options.get("arrayType") == "list":
                    effective_col_type = "list"
                    property_dict = {"type": "string", "format": "list"}
                else:
                    effective_col_type = "array"
                    property_dict = {"type": "array"}

            if col_type == "string":
                has_polars_nulls = sample.filter(pl.col(name).is_null()).height > 0
                missing_filter: pl.Expr = pl.col(name).is_null()
                has_missing_values = False
                for mv in effective_missing_values:
                    if sample.filter(pl.col(name) == pl.lit(mv)).height > 0:
                        detected_missing_values.add(mv)
                        has_missing_values = True
                        missing_filter = missing_filter | (pl.col(name) == pl.lit(mv))
                is_nullable = has_polars_nulls or has_missing_values

                if not keep_strings:
                    effective_sample = sample.filter(missing_filter.not_())
                    if effective_sample.height > 0:
                        effective_failure_threshold = (
                            effective_sample.height
                            - math.floor(effective_sample.height * confidence)
                        ) or 1
                        for regex, nameless_column in regex_mapping.items():
                            failures = (
                                effective_sample.filter(
                                    pl.col(name).str.contains(regex).not_()
                                )
                                .head(effective_failure_threshold)
                                .height
                            )

                            if failures < effective_failure_threshold:
                                effective_col_type = nameless_column["type"]
                                property_dict = dict(nameless_column["property"])
                                break

            if col_type == "number":
                failures = (
                    sample.filter((pl.col(name) == pl.col(name).round(0)).not_())
                    .head(failure_threshold)
                    .height
                )

                if failures < failure_threshold:
                    effective_col_type = "integer"
                    property_dict = {"type": "integer"}

            if col_type != "string":
                if sample.filter(pl.col(name).is_null()).height > 0:
                    is_nullable = True

        column = _build_column(name, effective_col_type, property_dict)

        if is_nullable:
            set_property_nullable(column)
        _enhance_column(column, **options)
        columns.append(column)

    table_schema = TableSchema(properties=get_column_properties(columns))

    if options.get("missingValues") is None and len(detected_missing_values) > 0:
        table_schema.missingValues = list(detected_missing_values)

    _enhance_schema(table_schema, **options)
    return table_schema


def _create_property_dict(col_type: str) -> Descriptor:
    match col_type:
        case "boolean":
            return {"type": "boolean"}
        case "integer":
            return {"type": "integer"}
        case "number":
            return {"type": "number"}
        case "string":
            return {"type": "string"}
        case "date":
            return {"type": "string", "format": "date"}
        case "date-time":
            return {"type": "string", "format": "date-time"}
        case "time":
            return {"type": "string", "format": "time"}
        case "array":
            return {"type": "array"}
        case "object":
            return {"type": "object"}
        case _:
            return {}


def _build_column(name: str, col_type: str, property_dict: Descriptor) -> Column:
    return create_column_from_property(name, property_dict)


def _escape_regex(value: str) -> str:
    return re.escape(value)


def _derive_month_first(format: str | None) -> bool | None:
    if not format:
        return None
    m_index = format.find("%m")
    d_index = format.find("%d")
    if m_index == -1 or d_index == -1:
        return None
    return m_index < d_index


def _create_regex_mapping(
    **options: Unpack[InferTableSchemaOptions],
) -> dict[str, Descriptor]:
    comma_decimal = options.get("commaDecimal")
    month_first = options.get("monthFirst")
    true_values = options.get("trueValues")
    false_values = options.get("falseValues")
    decimal_char = options.get("decimalChar")
    group_char = options.get("groupChar")
    list_delimiter = options.get("listDelimiter")
    date_format = options.get("dateFormat")
    time_format = options.get("timeFormat")
    datetime_format = options.get("datetimeFormat")

    effective_comma_decimal = (
        comma_decimal
        if comma_decimal is not None
        else (decimal_char == "," or group_char == ".")
    )

    effective_month_first = (
        month_first
        if month_first is not None
        else (
            _derive_month_first(date_format)
            if _derive_month_first(date_format) is not None
            else _derive_month_first(datetime_format)
        )
    )

    all_bool_values = [
        *(true_values or ["true", "True", "TRUE"]),
        *(false_values or ["false", "False", "FALSE"]),
    ]
    bool_regex = f"^({'|'.join(_escape_regex(v) for v in all_bool_values)})$"

    list_esc = _escape_regex(list_delimiter or ",")

    mapping: dict[str, Descriptor] = {
        "^\\d+$": {"type": "integer", "property": {"type": "integer"}},
        "^\\d{1,3}(,\\d{3})+$": (
            {"type": "number", "property": {"type": "number"}}
            if effective_comma_decimal
            else {"type": "integer", "property": {"type": "integer", "groupChar": ","}}
        ),
        "^\\d+\\.\\d+$": (
            {"type": "integer", "property": {"type": "integer", "groupChar": "."}}
            if effective_comma_decimal
            else {"type": "number", "property": {"type": "number"}}
        ),
        "^\\d{1,3}(,\\d{3})+\\.\\d+$": {
            "type": "number",
            "property": {"type": "number", "groupChar": ","},
        },
        "^\\d{1,3}(\\.\\d{3})+,\\d+$": {
            "type": "number",
            "property": {"type": "number", "groupChar": ".", "decimalChar": ","},
        },
        "^[\\p{Sc}\\s-]*\\d+[%\\p{Sc}\\s]*$": {
            "type": "integer",
            "property": {"type": "integer", "withText": True},
        },
        "^[\\p{Sc}\\s-]*\\d{1,3}(,\\d{3})+[%\\p{Sc}\\s]*$": (
            {"type": "number", "property": {"type": "number", "withText": True}}
            if effective_comma_decimal
            else {
                "type": "integer",
                "property": {"type": "integer", "groupChar": ",", "withText": True},
            }
        ),
        "^[\\p{Sc}\\s-]*\\d+\\.\\d+[%\\p{Sc}\\s]*$": (
            {
                "type": "integer",
                "property": {"type": "integer", "groupChar": ".", "withText": True},
            }
            if effective_comma_decimal
            else {"type": "number", "property": {"type": "number", "withText": True}}
        ),
        "^[\\p{Sc}\\s-]*\\d{1,3}(,\\d{3})+\\.\\d+[%\\p{Sc}\\s]*$": {
            "type": "number",
            "property": {"type": "number", "groupChar": ",", "withText": True},
        },
        "^[\\p{Sc}\\s-]*\\d{1,3}(\\.\\d{3})+,\\d+[%\\p{Sc}\\s]*$": {
            "type": "number",
            "property": {
                "type": "number",
                "groupChar": ".",
                "decimalChar": ",",
                "withText": True,
            },
        },
        bool_regex: {
            "type": "boolean",
            "property": {"type": "boolean"},
        },
        "^\\d{4}-\\d{2}-\\d{2}$": {
            "type": "date",
            "property": {"type": "string", "format": "date"},
        },
        "^\\d{4}/\\d{2}/\\d{2}$": {
            "type": "date",
            "property": {
                "type": "string",
                "format": "date",
                "temporalFormat": "%Y/%m/%d",
            },
        },
        "^\\d{2}/\\d{2}/\\d{4}$": (
            {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m/%d/%Y",
                },
            }
            if effective_month_first
            else {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d/%m/%Y",
                },
            }
        ),
        "^\\d{2}-\\d{2}-\\d{4}$": (
            {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m-%d-%Y",
                },
            }
            if effective_month_first
            else {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d-%m-%Y",
                },
            }
        ),
        "^\\d{2}\\.\\d{2}\\.\\d{4}$": (
            {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%m.%d.%Y",
                },
            }
            if effective_month_first
            else {
                "type": "date",
                "property": {
                    "type": "string",
                    "format": "date",
                    "temporalFormat": "%d.%m.%Y",
                },
            }
        ),
        "^\\d{2}:\\d{2}:\\d{2}$": {
            "type": "time",
            "property": {"type": "string", "format": "time"},
        },
        "^\\d{2}:\\d{2}$": {
            "type": "time",
            "property": {"type": "string", "format": "time", "temporalFormat": "%H:%M"},
        },
        "^\\d{1,2}:\\d{2}:\\d{2}\\s*(am|pm|AM|PM)$": {
            "type": "time",
            "property": {
                "type": "string",
                "format": "time",
                "temporalFormat": "%I:%M:%S %p",
            },
        },
        "^\\d{1,2}:\\d{2}\\s*(am|pm|AM|PM)$": {
            "type": "time",
            "property": {
                "type": "string",
                "format": "time",
                "temporalFormat": "%I:%M %p",
            },
        },
        "^\\d{2}:\\d{2}:\\d{2}[+-]\\d{2}:?\\d{2}$": {
            "type": "time",
            "property": {"type": "string", "format": "time"},
        },
        "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}Z?$": {
            "type": "date-time",
            "property": {"type": "string", "format": "date-time"},
        },
        "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}[+-]\\d{2}:?\\d{2}$": {
            "type": "date-time",
            "property": {"type": "string", "format": "date-time"},
        },
        "^\\d{4}-\\d{2}-\\d{2} \\d{2}:\\d{2}:\\d{2}$": {
            "type": "date-time",
            "property": {
                "type": "string",
                "format": "date-time",
                "temporalFormat": "%Y-%m-%d %H:%M:%S",
            },
        },
        "^\\d{2}/\\d{2}/\\d{4} \\d{2}:\\d{2}$": (
            {
                "type": "date-time",
                "property": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M",
                },
            }
            if effective_month_first
            else {
                "type": "date-time",
                "property": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M",
                },
            }
        ),
        "^\\d{2}/\\d{2}/\\d{4} \\d{2}:\\d{2}:\\d{2}$": (
            {
                "type": "date-time",
                "property": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%m/%d/%Y %H:%M:%S",
                },
            }
            if effective_month_first
            else {
                "type": "date-time",
                "property": {
                    "type": "string",
                    "format": "date-time",
                    "temporalFormat": "%d/%m/%Y %H:%M:%S",
                },
            }
        ),
        "^\\{": {"type": "object", "property": {"type": "object"}},
        "^\\[": {"type": "array", "property": {"type": "array"}},
        f"^\\d+{list_esc}\\d+$": {
            "type": "list",
            "property": {"type": "string", "format": "list", "itemType": "integer"},
        },
        f"^[\\d.]+{list_esc}[\\d.]+$": {
            "type": "list",
            "property": {"type": "string", "format": "list", "itemType": "number"},
        },
        "^https?://\\S+$": {
            "type": "url",
            "property": {"type": "string", "format": "url"},
        },
        "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$": {
            "type": "email",
            "property": {"type": "string", "format": "email"},
        },
        "^(POINT|LINESTRING|POLYGON|MULTIPOINT|MULTILINESTRING|MULTIPOLYGON|GEOMETRYCOLLECTION)\\s*\\(": {
            "type": "wkt",
            "property": {"type": "string", "format": "wkt"},
        },
        "^P(\\d+Y)?(\\d+M)?(\\d+W)?(\\d+D)?(T(\\d+H)?(\\d+M)?(\\d+(\\.\\d+)?S)?)?$": {
            "type": "duration",
            "property": {"type": "string", "format": "duration"},
        },
        "^([0-9a-fA-F]{2}){8,}$": {
            "type": "hex",
            "property": {"type": "string", "format": "hex"},
        },
    }

    canonical_formats: dict[str, str] = {
        "date": "%Y-%m-%d",
        "time": "%H:%M:%S",
        "date-time": "%Y-%m-%dT%H:%M:%S",
    }

    to_delete: list[str] = []
    for regex, col in mapping.items():
        user_format = (
            date_format
            if col["type"] == "date"
            else (
                time_format
                if col["type"] == "time"
                else (datetime_format if col["type"] == "date-time" else None)
            )
        )
        if user_format is None:
            continue
        entry_format = (
            col["property"].get("temporalFormat")
            if "temporalFormat" in col.get("property", {})
            else canonical_formats.get(col["type"])
        )
        if entry_format != user_format:
            to_delete.append(regex)

    for regex in to_delete:
        del mapping[regex]

    return mapping


def _enhance_column(column: Column, **options: Unpack[InferTableSchemaOptions]) -> None:
    if not options:
        return
    if column.type == "boolean":
        if options.get("trueValues") is not None:
            column.property.trueValues = options["trueValues"]  # type: ignore[union-attr]
        if options.get("falseValues") is not None:
            column.property.falseValues = options["falseValues"]  # type: ignore[union-attr]
    elif column.type == "integer":
        if options.get("groupChar") is not None:
            column.property.groupChar = options["groupChar"]  # type: ignore[union-attr]
    elif column.type == "number":
        if options.get("decimalChar") is not None:
            column.property.decimalChar = options["decimalChar"]  # type: ignore[union-attr]
        if options.get("groupChar") is not None:
            column.property.groupChar = options["groupChar"]  # type: ignore[union-attr]
    elif column.type == "date-time":
        if options.get("datetimeFormat") is not None:
            column.property.temporalFormat = options["datetimeFormat"]  # type: ignore[union-attr]
    elif column.type == "date":
        if options.get("dateFormat") is not None:
            column.property.temporalFormat = options["dateFormat"]  # type: ignore[union-attr]
    elif column.type == "time":
        if options.get("timeFormat") is not None:
            column.property.temporalFormat = options["timeFormat"]  # type: ignore[union-attr]
    elif column.type == "list":
        if options.get("listDelimiter") is not None:
            column.property.delimiter = options["listDelimiter"]  # type: ignore[union-attr]
        if options.get("listItemType") is not None:
            column.property.itemType = options["listItemType"]  # type: ignore[union-attr]


def _enhance_schema(
    table_schema: TableSchema,
    **options: Unpack[InferTableSchemaOptions],
) -> None:
    if options.get("missingValues") is not None:
        table_schema.missingValues = list(options["missingValues"])
