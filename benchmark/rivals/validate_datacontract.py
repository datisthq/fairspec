import json
import os
import tempfile

import duckdb
from common import (
    BOOLEAN_PATTERN,
    DATE_TIME_PATTERN,
    Column,
    Schema,
    load_schema,
    print_result,
    read_arguments,
)
from datacontract.data_contract import DataContract
from datacontract.model.run import ResultEnum, Run

TEXT_TABLE = "data"
TYPED_TABLE = "data_typed"
TYPE_PATTERNS = {
    "integer": r"^-?\d+$",
    "number": r"^-?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$",
    "boolean": BOOLEAN_PATTERN,
    "date-time": DATE_TIME_PATTERN,
}
CAST_TYPES = {"integer": "BIGINT", "number": "DOUBLE"}
COUNTED_METRICS = {"missing_count", "invalid_count", "duplicate_count"}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    with tempfile.TemporaryDirectory() as directory:
        database = os.path.join(directory, "data.duckdb")
        create_views(database, os.path.abspath(csv_path), schema)
        contract = create_contract(database, schema)
        run = DataContract(data_contract_str=json.dumps(contract)).test()
    print_result(count_failures(run))


def create_views(database: str, csv_path: str, schema: Schema) -> None:
    # the local CSV server sniffs types, and one unparsable cell aborts the whole run
    path = csv_path.replace("'", "''")
    source = f"read_csv('{path}', header = true, all_varchar = true)"
    casts = [
        f"TRY_CAST({quote(column.name)} AS {CAST_TYPES[column.kind]}) AS {quote(column.name)}"
        for column in ranged_columns(schema)
    ]
    connection = duckdb.connect(database)
    connection.execute(f"CREATE VIEW {TEXT_TABLE} AS SELECT * FROM {source}")
    if casts:
        connection.execute(
            f"CREATE VIEW {TYPED_TABLE} AS SELECT {', '.join(casts)} FROM {source}"
        )
    connection.close()


def create_contract(database: str, schema: Schema) -> dict[str, object]:
    tables: list[dict[str, object]] = [
        {
            "name": TEXT_TABLE,
            "physicalType": "view",
            "properties": [
                create_text_property(column, schema.primary_key == [column.name])
                for column in schema.columns
            ],
        }
    ]
    ranged = ranged_columns(schema)
    if ranged:
        tables.append(
            {
                "name": TYPED_TABLE,
                "physicalType": "view",
                "properties": [create_typed_property(column) for column in ranged],
            }
        )
    return {
        "apiVersion": "v3.2.0",
        "kind": "DataContract",
        "id": "benchmark",
        "version": "1.0.0",
        "status": "active",
        "servers": [{"server": "local", "type": "duckdb", "database": database}],
        "schema": tables,
    }


def create_text_property(column: Column, primary_key: bool) -> dict[str, object]:
    options: dict[str, object] = {}
    if column.min_length is not None:
        options["minLength"] = column.min_length
    if column.pattern is not None:
        options["pattern"] = column.pattern
    prop: dict[str, object] = {
        "name": column.name,
        "logicalType": "string",
        "required": not column.nullable,
    }
    if options:
        prop["logicalTypeOptions"] = options
    if column.enum is not None:
        prop["enum"] = [{"value": value} for value in column.enum]
    if column.kind in TYPE_PATTERNS:
        prop["quality"] = [
            {
                "type": "library",
                "metric": "invalidValues",
                "arguments": {"pattern": TYPE_PATTERNS[column.kind]},
                "mustBe": 0,
            }
        ]
    if primary_key:
        prop["primaryKey"] = True
    return prop


def create_typed_property(column: Column) -> dict[str, object]:
    options: dict[str, object] = {}
    if column.minimum is not None:
        options["minimum"] = column.minimum
    if column.maximum is not None:
        options["maximum"] = column.maximum
    return {
        "name": column.name,
        "logicalType": column.kind,
        "logicalTypeOptions": options,
    }


def count_failures(run: Run) -> int:
    count = 0
    for check in run.checks or []:
        if check.result == ResultEnum.error or check.type == "general":
            raise RuntimeError(f"{check.name}: {check.reason}")
        diagnostics = check.diagnostics or {}
        if diagnostics.get("metric") in COUNTED_METRICS:
            count += int(diagnostics["value"])
    return count


def ranged_columns(schema: Schema) -> list[Column]:
    return [
        column
        for column in schema.columns
        if column.kind in CAST_TYPES
        and (column.minimum is not None or column.maximum is not None)
    ]


def quote(name: str) -> str:
    return '"' + name.replace('"', '""') + '"'


if __name__ == "__main__":
    main()
