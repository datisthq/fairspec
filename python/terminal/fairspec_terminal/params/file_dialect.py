from typing import Annotated

import typer

Dialect = Annotated[
    str | None, typer.Option("--dialect", help="path to a Dialect descriptor")
]
Format = Annotated[
    str | None, typer.Option("--format", help="format type (csv, json, xlsx, etc)")
]
Delimiter = Annotated[
    str | None,
    typer.Option("--delimiter", help="character used to separate fields in the data"),
]
LineTerminator = Annotated[
    str | None,
    typer.Option("--line-terminator", help="character sequence used to terminate rows"),
]
QuoteChar = Annotated[
    str | None, typer.Option("--quote-char", help="character used to quote fields")
]
NullSequence = Annotated[
    str | None,
    typer.Option(
        "--null-sequence", help="character sequence representing null or missing values"
    ),
]
HeaderRows = Annotated[
    str | None,
    typer.Option(
        "--header-rows",
        help="comma-separated row numbers (1-indexed) for headers, or 'false' to disable",
    ),
]
HeaderJoin = Annotated[
    str | None,
    typer.Option("--header-join", help="character used to join multi-line headers"),
]
CommentRows = Annotated[
    str | None,
    typer.Option(
        "--comment-rows",
        help="comma-separated row numbers (1-indexed) to exclude from data",
    ),
]
CommentPrefix = Annotated[
    str | None,
    typer.Option(
        "--comment-prefix", help="character sequence denoting the start of a comment line"
    ),
]
ColumnNamesParam = Annotated[
    str | None,
    typer.Option("--column-names", help="comma-separated list of column names"),
]
JsonPointer = Annotated[
    str | None,
    typer.Option(
        "--json-pointer", help="JSON pointer to the data array within the JSON document"
    ),
]
RowType = Annotated[
    str | None, typer.Option("--row-type", help="the type of each row in the data")
]
SheetNumber = Annotated[
    int | None,
    typer.Option(
        "--sheet-number",
        help="for spreadsheet data, the sheet number to read (0-indexed)",
    ),
]
SheetName = Annotated[
    str | None,
    typer.Option("--sheet-name", help="for spreadsheet data, the sheet name to read"),
]
TableName = Annotated[
    str | None,
    typer.Option("--table-name", help="for database sources, the table name to read"),
]
