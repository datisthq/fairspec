import code

import polars as pl

import fairspec_library
from fairspec_library import load_table
from fairspec_metadata import Resource

from fairspec_terminal.helpers.file_dialect import (
    create_file_dialect_from_path_and_options,
)
from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.params import (
    ArrayType,
    ColumnNamesParam,
    ColumnTypes,
    CommaDecimal,
    CommentPrefix,
    CommentRows,
    Confidence,
    DateFormat,
    DatetimeFormat,
    Debug,
    DecimalChar,
    Delimiter,
    Dialect,
    FalseValues,
    Format,
    FromDataset,
    FromResource,
    GroupChar,
    HeaderJoin,
    HeaderRows,
    JsonPointer,
    KeepStrings,
    LineTerminator,
    ListDelimiter,
    ListItemType,
    MissingValues,
    MonthFirst,
    NullSequence,
    OptionalPath,
    QuoteChar,
    RowType,
    SampleRows,
    SheetName,
    SheetNumber,
    TableName,
    TableSchemaPath,
    TimeFormat,
    TrueValues,
)
from fairspec_terminal.program import table_program
from fairspec_terminal.session import Session


@table_program.command()
def script(
    path: OptionalPath = None,
    dataset: FromDataset = None,
    resource: FromResource = None,
    debug: Debug = False,
    dialect: Dialect = None,
    format: Format = None,
    delimiter: Delimiter = None,
    line_terminator: LineTerminator = None,
    quote_char: QuoteChar = None,
    null_sequence: NullSequence = None,
    header_rows: HeaderRows = None,
    header_join: HeaderJoin = None,
    comment_rows: CommentRows = None,
    comment_prefix: CommentPrefix = None,
    column_names: ColumnNamesParam = None,
    json_pointer: JsonPointer = None,
    row_type: RowType = None,
    sheet_number: SheetNumber = None,
    sheet_name: SheetName = None,
    table_name: TableName = None,
    schema: TableSchemaPath = None,
    sample_rows: SampleRows = None,
    confidence: Confidence = None,
    comma_decimal: CommaDecimal = False,
    month_first: MonthFirst = False,
    keep_strings: KeepStrings = False,
    column_types: ColumnTypes = None,
    missing_values: MissingValues = None,
    decimal_char: DecimalChar = None,
    group_char: GroupChar = None,
    true_values: TrueValues = None,
    false_values: FalseValues = None,
    datetime_format: DatetimeFormat = None,
    date_format: DateFormat = None,
    time_format: TimeFormat = None,
    array_type: ArrayType = None,
    list_delimiter: ListDelimiter = None,
    list_item_type: ListItemType = None,
) -> None:
    """Start a scripting session for a table."""
    session = Session(debug=debug)

    file_dialect = (
        (
            dialect
            or create_file_dialect_from_path_and_options(
                path,
                format=format,
                delimiter=delimiter,
                line_terminator=line_terminator,
                quote_char=quote_char,
                null_sequence=null_sequence,
                header_rows=header_rows,
                header_join=header_join,
                comment_rows=comment_rows,
                comment_prefix=comment_prefix,
                column_names=column_names,
                json_pointer=json_pointer,
                row_type=row_type,
                sheet_number=sheet_number,
                sheet_name=sheet_name,
                table_name=table_name,
            )
        )
        if path
        else None
    )

    res: Resource = (
        Resource(data=path, fileDialect=file_dialect, tableSchema=schema)
        if path
        else select_resource(session, dataset=dataset, resource=resource)
    )

    def _load() -> pl.LazyFrame:
        table = load_table(res, denormalized=True)
        if table is None:
            raise ValueError("Could not load table")
        return table

    table = session.task("Loading table", _load)

    session.render_text(
        "[dim]`fairspec` and `table` variables are available in the session[/dim]",
        status="warning",
    )

    code.interact(
        banner="",
        local={"fairspec": fairspec_library, "table": table},
    )
