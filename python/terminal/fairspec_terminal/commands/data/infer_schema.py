from fairspec_library import infer_data_schema
from fairspec_metadata import Resource

from fairspec_terminal.params import Debug, Json, RequiredFilePath, Silent
from fairspec_terminal.program import data_program
from fairspec_terminal.session import Session


@data_program.command(name="infer-schema")
def infer_schema(
    path: RequiredFilePath,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Infer a Data Schema."""
    session = Session(silent=silent, debug=debug, json=json)

    def _infer() -> object:
        data_schema = infer_data_schema(Resource(data=path))
        if not data_schema:
            raise ValueError("Could not infer data schema")
        return data_schema

    data_schema = session.task("Inferring data schema", _infer)

    session.render_data_result(data_schema)
