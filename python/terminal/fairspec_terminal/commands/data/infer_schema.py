from fairspec_library import infer_data_schema
from fairspec_metadata import DataSchema, Resource
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.params import Debug, RequiredFilePath, Silent
from fairspec_terminal.session import Session


class InferDataSchemaInput(CommandInput):
    path: RequiredFilePath
    silent: Silent = False
    debug: Debug = False


@command(description="Infer a Data Schema")
def infer_data_schema_command(
    input: InferDataSchemaInput, context: Context
) -> DataSchema:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)

    def _infer() -> DataSchema:
        data_schema = infer_data_schema(Resource(data=input.path))
        if not data_schema:
            raise ValueError("Could not infer data schema")
        return data_schema

    data_schema = session.task("Inferring data schema", _infer)

    return session.render_data_result(data_schema)
