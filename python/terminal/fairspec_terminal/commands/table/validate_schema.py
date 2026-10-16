from fairspec_metadata import validate_table_schema

from fairspec_terminal.params import Debug, Json, RequiredFilePath, Silent
from fairspec_terminal.program import table_program
from fairspec_terminal.session import Session


@table_program.command(name="validate-schema")
def validate_schema(
    path: RequiredFilePath,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Validate a Table Schema."""
    session = Session(silent=silent, debug=debug, json=json)

    def _validate() -> object:
        result = validate_table_schema(path)
        return result

    report = session.task("Validating table schema", _validate)

    session.render_report_result(report)  # type: ignore[arg-type]
