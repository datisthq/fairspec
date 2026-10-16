from fairspec_library import validate_dataset
from fairspec_metadata import Report

from fairspec_terminal.params import Debug, Json, RequiredPath
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command()
def validate(
    path: RequiredPath,
    json: Json = False,
    debug: Debug = False,
) -> None:
    """Validate a dataset from a local or remote path."""
    session = Session(debug=debug, json=json)

    def _validate() -> Report:
        return validate_dataset(path)

    report = session.task("Validating dataset", _validate)

    session.render_report_result(report)
