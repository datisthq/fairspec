import re

import patito as pt
import polars as pl
from common import (
    BOOLEAN_PATTERN,
    DATE_TIME_PATTERN,
    Column,
    Schema,
    load_schema,
    print_result,
    read_arguments,
)
from patito.exceptions import DataFrameValidationError

TYPES: dict[str, type] = {
    "integer": int,
    "number": float,
    "string": str,
    "boolean": str,
    "date-time": str,
}
CASTS = {"integer": pl.Int64, "number": pl.Float64}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    model = convert_schema(schema)
    frame = pl.read_csv(csv_path, infer_schema=False).with_columns(
        pl.col(column.name).cast(CASTS[column.kind], strict=False)
        for column in schema.columns
        if column.kind in CASTS
    )
    try:
        model.validate(frame)
        print_result(0)
    except DataFrameValidationError as error:
        print_result(sum(count_rows(item["msg"]) for item in error.errors()))


def count_rows(message: str) -> int:
    # patito reports counts only inside its messages: "3 missing values", "2 rows ..."
    match = re.match(r"(\d+) ", message)
    if match is None:
        raise ValueError(f"Uncounted patito error: {message}")
    return int(match.group(1))


def convert_schema(schema: Schema) -> type[pt.Model]:
    annotations = {
        column.name: TYPES[column.kind] | None
        if column.nullable
        else TYPES[column.kind]
        for column in schema.columns
    }
    fields = {
        column.name: convert_column(column, schema.primary_key == [column.name])
        for column in schema.columns
    }
    return type("Generated", (pt.Model,), {"__annotations__": annotations, **fields})


def convert_column(column: Column, primary_key: bool):
    constraints: list[pl.Expr] = []
    if column.enum is not None:
        # patito's own enum check reports the bad values, not how many rows hold them
        constraints.append(pl.col(column.name).is_in(column.enum))
    if column.kind == "date-time":
        constraints.append(pl.col(column.name).str.contains(DATE_TIME_PATTERN))
    if column.kind == "boolean":
        constraints.append(pl.col(column.name).str.contains(BOOLEAN_PATTERN))
    return pt.Field(
        ge=column.minimum,
        le=column.maximum,
        min_length=column.min_length,
        pattern=column.pattern,
        unique=primary_key,
        constraints=constraints or None,
    )


if __name__ == "__main__":
    main()
