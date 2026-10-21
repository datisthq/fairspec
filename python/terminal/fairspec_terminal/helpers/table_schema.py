from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_table.models.schema import InferTableSchemaOptions

    from fairspec_terminal.params import TableSchemaParams


def create_infer_table_schema_options(
    params: TableSchemaParams,
) -> InferTableSchemaOptions:
    options: InferTableSchemaOptions = {}

    if params.sample_rows is not None:
        options["sampleRows"] = params.sample_rows
    if params.confidence is not None:
        options["confidence"] = params.confidence
    if params.comma_decimal:
        options["commaDecimal"] = True
    if params.month_first:
        options["monthFirst"] = True
    if params.keep_strings:
        options["keepStrings"] = True
    if params.column_types:
        options["columnTypes"] = params.column_types
    if params.column_properties:
        options["columnProperties"] = params.column_properties
    if params.missing_values:
        options["missingValues"] = params.missing_values
    if params.decimal_char:
        options["decimalChar"] = params.decimal_char
    if params.group_char:
        options["groupChar"] = params.group_char
    if params.true_values:
        options["trueValues"] = params.true_values
    if params.false_values:
        options["falseValues"] = params.false_values
    if params.datetime_format:
        options["datetimeFormat"] = params.datetime_format
    if params.date_format:
        options["dateFormat"] = params.date_format
    if params.time_format:
        options["timeFormat"] = params.time_format
    if params.array_type:
        options["arrayType"] = params.array_type
    if params.list_delimiter:
        options["listDelimiter"] = params.list_delimiter
    if params.list_item_type:
        options["listItemType"] = params.list_item_type

    return options
