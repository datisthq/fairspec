import polars as pl
from fairspec_library import load_table
from fairspec_metadata import Resource
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
    TableSchemaParams,
    TableSchemaPath,
)
from fairspec_terminal.session import Session


class PreviewTableInput(TableSchemaParams, DialectParams):
    path: TablePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    debug: Debug = False
    table_schema: TableSchemaPath = None


@command(description="Preview a table from a local or remote path")
def preview_table_command(input: PreviewTableInput, context: Context) -> Records:
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
        table = load_table(res, previewBytes=10_000, **options)
        if table is None:
            raise ValueError("Could not load table")
        return table

    table = session.task("Loading table preview", _load)

    def _collect() -> pl.DataFrame:
        result = table.limit(100).collect()
        assert isinstance(result, pl.DataFrame)
        return result

    frame = session.task("Collecting data", _collect)

    return session.render_frame_result(frame)
