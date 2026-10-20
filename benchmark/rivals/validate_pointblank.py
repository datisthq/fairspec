import pointblank as pb
import polars as pl
from common import (
    BOOLEAN_PATTERN,
    DATE_TIME_PATTERN,
    Schema,
    load_schema,
    print_result,
    read_arguments,
)

CASTS = {"integer": pl.Int64, "number": pl.Float64}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    # a LazyFrame is re-scanned for every check, about 10x slower than reading once
    frame = pl.read_csv(csv_path, infer_schema=False).with_columns(
        pl.col(column.name).cast(CASTS[column.kind], strict=False)
        for column in schema.columns
        if column.kind in CASTS
    )
    validation = create_validation(schema, frame).interrogate()
    print_result(sum(validation.n_failed().values()))


def create_validation(schema: Schema, frame: pl.DataFrame) -> pb.Validate:
    validation = pb.Validate(data=frame)
    for column in schema.columns:
        name = column.name
        if not column.nullable:
            validation = validation.col_vals_not_null(columns=name)
        if column.minimum is not None:
            validation = validation.col_vals_ge(
                columns=name, value=column.minimum, na_pass=True
            )
        if column.maximum is not None:
            validation = validation.col_vals_le(
                columns=name, value=column.maximum, na_pass=True
            )
        if column.enum is not None:
            validation = validation.col_vals_in_set(
                columns=name, set=column.enum + [None]
            )
        patterns = [column.pattern]
        if column.min_length is not None:
            patterns.append(f"^.{{{column.min_length},}}$")
        if column.kind == "date-time":
            patterns.append(DATE_TIME_PATTERN)
        if column.kind == "boolean":
            patterns.append(BOOLEAN_PATTERN)
        for pattern in filter(None, patterns):
            validation = validation.col_vals_regex(
                columns=name, pattern=pattern, na_pass=True
            )
    if schema.primary_key:
        validation = validation.rows_distinct(columns_subset=schema.primary_key)
    return validation


if __name__ == "__main__":
    main()
