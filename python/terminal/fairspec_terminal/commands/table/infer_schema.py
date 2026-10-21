from fairspec_library import infer_table_schema
from fairspec_metadata import Resource, TableSchema
from fireargs import Context, command

from fairspec_terminal.helpers.file_dialect import (
    create_file_dialect_from_path_and_options,
)
from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.params import (
    Debug,
    DialectParams,
    FromDataset,
    FromResource,
    TablePath,
    TableSchemaParams,
)
from fairspec_terminal.session import Session


class InferTableSchemaInput(TableSchemaParams, DialectParams):
    path: TablePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    debug: Debug = False


@command(description="Infer a table schema from a table")
def infer_table_schema_command(
    input: InferTableSchemaInput, context: Context
) -> TableSchema:
    session = Session(debug=input.debug, json=context.json)
    path = input.path
    options = create_infer_table_schema_options(input)

    file_dialect = (
        (input.dialect or create_file_dialect_from_path_and_options(path, input))
        if path
        else None
    )

    res: Resource = (
        Resource(data=path, fileDialect=file_dialect)
        if path
        else select_resource(session, dataset=input.dataset, resource=input.resource)
    )

    def _infer() -> TableSchema:
        table_schema = infer_table_schema(res, **options)
        if not table_schema:
            raise ValueError("Could not infer table schema")
        return table_schema

    table_schema = session.task("Inferring schema", _infer)

    return session.render_data_result(table_schema)
