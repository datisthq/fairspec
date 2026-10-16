from fairspec_dataset import validate_file
from fairspec_metadata import Report, Resource

from fairspec_terminal.params import (
    Debug,
    Hash_,
    HashType,
    Json,
    RequiredFilePath,
    Silent,
)
from fairspec_terminal.program import file_program
from fairspec_terminal.session import Session


@file_program.command()
def validate(
    path: RequiredFilePath,
    hash: Hash_ = None,
    hash_type: HashType = "sha256",
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Validate a file from a local or remote path."""
    session = Session(silent=silent, debug=debug, json=json)

    def _validate() -> Report:
        integrity = {"hash": hash, "type": hash_type or "md5"} if hash else None
        return validate_file(Resource(data=path, integrity=integrity))  # type: ignore[arg-type]

    report = session.task("Validating file", _validate)

    session.render_report_result(report)
