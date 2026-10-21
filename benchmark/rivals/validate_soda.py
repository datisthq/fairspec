import json

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
from soda_core.common.yaml import ContractYamlSource
from soda_core.contracts.contract_verification import ContractVerificationSession
from soda_duckdb import DuckDBDataSource

DATA_SOURCE = "duckdb"
TABLE = "data"
TYPE_PATTERNS = {
    "integer": r"^-?\d+$",
    "number": r"^-?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$",
    "boolean": BOOLEAN_PATTERN,
    "date-time": DATE_TIME_PATTERN,
}
CAST_TYPES = {"integer": "BIGINT", "number": "DOUBLE"}


def main() -> None:
    csv_path, schema_path = read_arguments()
    schema = load_schema(schema_path)
    connection = duckdb.connect()
    path = "'" + csv_path.replace("'", "''") + "'"
    connection.execute(
        f"CREATE VIEW {TABLE} AS SELECT * FROM read_csv({path}, header = true, all_varchar = true)"
    )
    data_source = DuckDBDataSource.from_existing_cursor(connection, name=DATA_SOURCE)
    contract = ContractYamlSource(yaml_str=json.dumps(create_contract(schema)))
    session = ContractVerificationSession.execute(
        contract_yaml_sources=[contract], data_source_impls=[data_source]
    )
    if session.has_errors:
        raise RuntimeError(session.get_errors_str())
    print_result(
        sum(
            int(check_result.threshold_value or 0)
            for result in session.contract_verification_results
            for check_result in result.check_results
        )
    )


def create_contract(schema: Schema) -> dict[str, object]:
    return {
        "dataset": f"{DATA_SOURCE}/main/{TABLE}",
        "columns": [create_column(column) for column in schema.columns],
        "checks": [{"duplicate": {"columns": schema.primary_key}}],
    }


def create_column(column: Column) -> dict[str, object]:
    checks = create_checks(column)
    return {"name": column.name, "checks": checks} if checks else {"name": column.name}


def create_checks(column: Column) -> list[object]:
    checks: list[object] = []
    if not column.nullable:
        checks.append({"missing": {}})
    if column.kind in TYPE_PATTERNS:
        checks.append(
            create_invalid(
                "type",
                valid_format={"name": column.kind, "regex": TYPE_PATTERNS[column.kind]},
            )
        )
    if column.minimum is not None or column.maximum is not None:
        quoted = '"' + column.name.replace('"', '""') + '"'
        bounds = {"valid_min": column.minimum, "valid_max": column.maximum}
        checks.append(
            create_invalid(
                "range",
                column_expression=f"TRY_CAST({quoted} AS {CAST_TYPES[column.kind]})",
                **{key: value for key, value in bounds.items() if value is not None},
            )
        )
    if column.min_length is not None:
        checks.append(create_invalid("length", valid_min_length=column.min_length))
    if column.pattern is not None:
        checks.append(
            create_invalid(
                "pattern", valid_format={"name": "pattern", "regex": column.pattern}
            )
        )
    if column.enum is not None:
        checks.append(create_invalid("enum", valid_values=column.enum))
    return checks


def create_invalid(qualifier: str, **options: object):
    return {"invalid": {"qualifier": qualifier, **options}}


if __name__ == "__main__":
    main()
