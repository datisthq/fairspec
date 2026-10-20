import pandera.polars as pa
import polars as pl
from common import (
    BOOLEAN_PATTERN,
    DATE_TIME_PATTERN,
    Schema,
    load_schema,
    print_result,
    read_arguments,
)
from pandera.errors import SchemaErrors

TYPES = {
    "integer": pl.Int64,
    "number": pl.Float64,
    "string": pl.String,
    "boolean": pl.String,
    "date-time": pl.String,
}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = convert_schema(load_schema(schema_path))
    # a LazyFrame is re-scanned for every check, about 20x slower than reading once
    frame = pl.read_csv(csv_path, infer_schema=False)
    try:
        schema.validate(frame, lazy=True)
        print_result(0)
    except SchemaErrors as error:
        print_result(count_cell_failures(error.failure_cases))


def count_cell_failures(failures: pl.DataFrame) -> int:
    # pandera reports a null input as a failed cast; a null cannot fail one
    is_null_cast = (
        pl.col("check").str.starts_with("coerce_dtype") & pl.col("failure_case").is_null()
    )
    return failures.filter(pl.col("index").is_not_null() & ~is_null_cast).height


def convert_schema(schema: Schema) -> pa.DataFrameSchema:
    columns: dict[str, pa.Column] = {}
    for column in schema.columns:
        checks = []
        if column.minimum is not None:
            checks.append(pa.Check.ge(column.minimum))
        if column.maximum is not None:
            checks.append(pa.Check.le(column.maximum))
        if column.min_length is not None:
            checks.append(pa.Check.str_length(min_value=column.min_length))
        if column.pattern is not None:
            checks.append(pa.Check.str_matches(column.pattern))
        if column.kind == "date-time":
            checks.append(pa.Check.str_matches(DATE_TIME_PATTERN))
        if column.kind == "boolean":
            checks.append(pa.Check.str_matches(BOOLEAN_PATTERN))
        if column.enum is not None:
            checks.append(pa.Check.isin(column.enum))
        columns[column.name] = pa.Column(
            TYPES[column.kind],
            checks=checks,
            nullable=column.nullable,
            unique=schema.primary_key == [column.name],
            coerce=True,
        )
    return pa.DataFrameSchema(columns)


if __name__ == "__main__":
    main()
