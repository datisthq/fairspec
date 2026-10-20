import dataframely as dy
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


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = convert_schema(load_schema(schema_path))
    frame = pl.scan_csv(csv_path, infer_schema=False)
    result = schema.filter(frame, cast=True)
    failure = result.failure
    print_result(sum(failure.counts().values()))


def convert_schema(schema: Schema) -> type[dy.Schema]:
    columns = {
        column.name: convert_column(column, schema.primary_key == [column.name])
        for column in schema.columns
    }
    return type("Generated", (dy.Schema,), columns)


def convert_column(column: Column, primary_key: bool) -> dy.Column:
    nullable = column.nullable and not primary_key
    if column.enum is not None:
        return dy.Enum(column.enum, nullable=nullable, primary_key=primary_key)
    match column.kind:
        case "integer":
            return dy.Int64(
                nullable=nullable,
                primary_key=primary_key,
                min=to_integer(column.minimum),
                max=to_integer(column.maximum),
            )
        case "number":
            return dy.Float64(
                nullable=nullable,
                primary_key=primary_key,
                min=column.minimum,
                max=column.maximum,
            )
        case "boolean":
            return dy.String(
                nullable=nullable, primary_key=primary_key, regex=BOOLEAN_PATTERN
            )
        case "date-time":
            return dy.String(
                nullable=nullable, primary_key=primary_key, regex=DATE_TIME_PATTERN
            )
        case "string":
            return dy.String(
                nullable=nullable,
                primary_key=primary_key,
                min_length=column.min_length,
                regex=column.pattern,
            )


def to_integer(value: float | None) -> int | None:
    return None if value is None else int(value)


if __name__ == "__main__":
    main()
