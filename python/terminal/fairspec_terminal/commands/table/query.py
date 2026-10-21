import polars as pl
from fairspec_library import load_table
from fairspec_metadata import Resource
from fairspec_table import query_table
from fireargs import Context, command

from fairspec_terminal.helpers.file_dialect import (
    create_file_dialect_from_path_and_options,
)
from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.models.output import Records
from fairspec_terminal.params import (
    Debug,
    DialectParams,
    FromDataset,
    FromResource,
    TablePath,
    TableQuery,
    TableSchemaParams,
    TableSchemaPath,
)
from fairspec_terminal.session import Session


class QueryTableInput(TableSchemaParams, DialectParams):
    path: TablePath = None
    query: TableQuery = None
    dataset: FromDataset = None
    resource: FromResource = None
    debug: Debug = False
    table_schema: TableSchemaPath = None


@command(description="Query a table from a local or remote path")
def query_table_command(input: QueryTableInput, context: Context) -> Records:
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
        table = load_table(res, **options)
        if table is None:
            raise ValueError("Could not load table")
        return table

    table = session.task("Loading table", _load)

    query = input.query
    if query:
        table = session.task("Executing query", lambda: query_table(table, query))

    def _collect() -> pl.DataFrame:
        result = table.collect()
        assert isinstance(result, pl.DataFrame)
        return result

    frame = session.task("Collecting data", _collect)

    return session.render_frame_result(frame)
