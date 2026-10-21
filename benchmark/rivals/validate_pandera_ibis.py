import tempfile
from os.path import join

import ibis
import ibis.expr.datatypes as dt
import pandera.ibis as pa
from common import (
    BOOLEAN_PATTERN,
    DATE_TIME_PATTERN,
    Schema,
    load_schema,
    print_result,
    read_arguments,
)
from pandera.errors import SchemaError, SchemaErrors

TYPES = {
    "integer": dt.int64,
    "number": dt.float64,
    "string": dt.string,
    "boolean": dt.string,
    "date-time": dt.string,
}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    with tempfile.TemporaryDirectory() as directory:
        connection = ibis.duckdb.connect(join(directory, "data.duckdb"))
        # a view over the CSV is re-read for every check, about 5x slower than a table
        source = connection.read_csv(csv_path, all_varchar=True)
        validate_table(schema, connection.create_table("data", source))


def validate_table(schema: Schema, table: ibis.Table) -> None:
    table = table.mutate(
        **{
            column.name: table[column.name].try_cast(TYPES[column.kind])
            for column in schema.columns
            if column.kind in ("integer", "number")
        }
    )
    try:
        convert_schema(schema).validate(table, lazy=True)
        print_result(0)
    except SchemaErrors as error:
        print_result(sum(count_failures(item) for item in error.schema_errors))


def count_failures(error: SchemaError) -> int:
    # pandera leaves a null check's failing rows as an unexecuted ibis expression
    failure_cases = error.failure_cases
    if isinstance(failure_cases, ibis.Table):
        return failure_cases.count().execute()
    return 0 if failure_cases is None else len(failure_cases)


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
        )
    return pa.DataFrameSchema(columns)


if __name__ == "__main__":
    main()
