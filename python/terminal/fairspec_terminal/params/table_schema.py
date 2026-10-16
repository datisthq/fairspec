from typing import Annotated

import typer

TableSchemaPath = Annotated[
    str | None, typer.Option("--schema", help="path to a Table Schema descriptor")
]
SampleRows = Annotated[
    int | None,
    typer.Option("--sample-rows", help="number of rows to sample for schema inference"),
]
Confidence = Annotated[
    float | None,
    typer.Option("--confidence", help="confidence threshold for schema inference"),
]
CommaDecimal = Annotated[
    bool,
    typer.Option(
        "--comma-decimal", help="use comma as decimal separator in schema inference"
    ),
]
MonthFirst = Annotated[
    bool,
    typer.Option(
        "--month-first", help="interpret dates as month-first in schema inference"
    ),
]
KeepStrings = Annotated[
    bool,
    typer.Option(
        "--keep-strings", help="keep fields as strings instead of inferring types"
    ),
]
ColumnTypes = Annotated[
    str | None,
    typer.Option(
        "--column-types",
        help="a list of comma-separated column name:type pairs to use for the schema",
    ),
]
MissingValues = Annotated[
    str | None,
    typer.Option("--missing-values", help="comma-separated values to treat as missing"),
]
DecimalChar = Annotated[
    str | None,
    typer.Option("--decimal-char", help="character to use as decimal separator"),
]
GroupChar = Annotated[
    str | None, typer.Option("--group-char", help="character to use for digit grouping")
]
TrueValues = Annotated[
    str | None, typer.Option("--true-values", help="values to treat as true")
]
FalseValues = Annotated[
    str | None, typer.Option("--false-values", help="values to treat as false")
]
DatetimeFormat = Annotated[
    str | None, typer.Option("--datetime-format", help="datetime format pattern")
]
DateFormat = Annotated[
    str | None, typer.Option("--date-format", help="date format pattern")
]
TimeFormat = Annotated[
    str | None, typer.Option("--time-format", help="time format pattern")
]
ArrayType = Annotated[
    str | None, typer.Option("--array-type", help="array type (array or list)")
]
ListDelimiter = Annotated[
    str | None, typer.Option("--list-delimiter", help="delimiter for list values")
]
ListItemType = Annotated[
    str | None, typer.Option("--list-item-type", help="type of items in lists")
]
