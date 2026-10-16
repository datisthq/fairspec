from fairspec_metadata import validate_data_schema

from fairspec_terminal.params import Debug, Json, RequiredFilePath, Silent
from fairspec_terminal.program import data_program
from fairspec_terminal.session import Session


@data_program.command(name="validate-schema")
def validate_schema(
    path: RequiredFilePath,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Validate a Data Schema."""
    session = Session(silent=silent, debug=debug, json=json)

    def _validate() -> object:
        result = validate_data_schema(path)
        return result

    report = session.task("Validating data schema", _validate)

    session.render_report_result(report)  # type: ignore[arg-type]
