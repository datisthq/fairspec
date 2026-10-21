import polars as pl
from fairspec_library import TableCheck, load_table
from fairspec_metadata import Report, Resource, create_report, resolve_table_schema
from fairspec_table import infer_table_schema_from_table, inspect_table
from fireargs import Context, command

from fairspec_terminal.helpers.file_dialect import (
    create_file_dialect_from_path_and_options,
)
from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.models.output import ReportResult
from fairspec_terminal.params import (
    Check,
    Debug,
    DialectParams,
    FromDataset,
    FromResource,
    TablePath,
    TableSchemaParams,
    TableSchemaPath,
)
from fairspec_terminal.session import Session


class ValidateTableInput(TableSchemaParams, DialectParams):
    path: TablePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    debug: Debug = False
    table_schema: TableSchemaPath = None
    check: Check = None


@command(description="Validate a table from a local or remote path")
def validate_table_command(input: ValidateTableInput, context: Context) -> ReportResult:
    session = Session(debug=input.debug, json=context.json)
    path = input.path
    options = create_infer_table_schema_options(input)

    file_dialect = (
        (input.dialect or create_file_dialect_from_path_and_options(path, input))
        if path
        else None
    )

    res: Resource = (
        Resource(data=path, fileDialect=file_dialect, tableSchema=input.table_schema)
        if path
        else select_resource(session, dataset=input.dataset, resource=input.resource)
    )

    def _load() -> pl.LazyFrame:
        table = load_table(res, denormalized=True)
        if table is None:
            raise ValueError("Could not load table")
        return table

    table = session.task("Loading table", _load)

    table_schema = session.task(
        "Loading schema",
        lambda: resolve_table_schema(res.tableSchema),
    )

    if not table_schema:
        table_schema = session.task(
            "Inferring schema",
            lambda: infer_table_schema_from_table(table, **options),
        )

    checks = [_create_table_check(check) for check in input.check or []]

    def _validate() -> Report:
        errors = list(inspect_table(table, table_schema=table_schema, checks=checks))
        return create_report(errors)  # type: ignore[arg-type]

    report = session.task("Validating table", _validate)

    return session.render_report_result(report)


def _create_table_check(check: str) -> TableCheck:
    name, _, condition = check.partition("=")
    return {"name": name, "condition": condition}
