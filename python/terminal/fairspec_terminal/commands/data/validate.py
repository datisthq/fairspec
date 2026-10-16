from fairspec_library import validate_data
from fairspec_metadata import Report, load_data_schema

from fairspec_terminal.params import DataSchemaPath, Debug, Json, RequiredFilePath, Silent
from fairspec_terminal.program import data_program
from fairspec_terminal.session import Session


@data_program.command()
def validate(
    path: RequiredFilePath,
    schema: DataSchemaPath,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Validate a JSON Data from a local or remote path."""
    session = Session(silent=silent, debug=debug, json=json)

    def _load_schema() -> object:
        if not schema:
            raise ValueError("No data schema provided")
        return load_data_schema(schema)

    data_schema = session.task("Loading data schema", _load_schema)

    def _validate() -> Report:
        return validate_data({"data": path, "dataSchema": data_schema})  # type: ignore[arg-type]

    report = session.task("Validating data", _validate)

    session.render_report_result(report)
