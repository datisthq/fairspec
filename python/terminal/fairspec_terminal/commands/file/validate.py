from fairspec_dataset import validate_file
from fairspec_metadata import Integrity, IntegrityType, Resource
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import ReportResult
from fairspec_terminal.params import Debug, Hash, HashType, RequiredFilePath, Silent
from fairspec_terminal.session import Session


class ValidateFileInput(CommandInput):
    path: RequiredFilePath
    hash: Hash = None
    hash_type: HashType = "sha256"
    silent: Silent = False
    debug: Debug = False


@command(description="Validate a file from a local or remote path")
def validate_file_command(input: ValidateFileInput, context: Context) -> ReportResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)
    integrity = (
        Integrity(hash=input.hash, type=IntegrityType(input.hash_type))
        if input.hash
        else None
    )

    report = session.task(
        "Validating file",
        lambda: validate_file(Resource(data=input.path, integrity=integrity)),
    )

    return session.render_report_result(report)
