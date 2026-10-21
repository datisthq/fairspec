import code

import polars as pl

import fairspec_library
from fairspec_library import load_table
from fairspec_metadata import Resource
from fireargs import command

from fairspec_terminal.helpers.file_dialect import (
    create_file_dialect_from_path_and_options,
)
from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.models.output import Empty
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


class ScriptTableInput(TableSchemaParams, DialectParams):
    path: TablePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    debug: Debug = False
    table_schema: TableSchemaPath = None


@command(description="Start a scripting session for a table", json_option=False)
def script_table_command(input: ScriptTableInput) -> Empty:
    session = Session(debug=input.debug)
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
        table = load_table(res, denormalized=True, **options)
        if table is None:
            raise ValueError("Could not load table")
        return table

    table = session.task("Loading table", _load)

    session.render_text(
        "[dim]`fairspec` and `table` variables are available in the session[/dim]",
        status="warning",
    )

    code.interact(
        banner="",
        local={"fairspec": fairspec_library, "table": table},
    )

    return Empty()
