from fairspec_library import validate_dataset
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import ReportResult
from fairspec_terminal.params import Debug, DescriptorPath
from fairspec_terminal.session import Session


class ValidateDatasetInput(CommandInput):
    path: DescriptorPath
    debug: Debug = False


@command(description="Validate a dataset from a local or remote path")
def validate_dataset_command(
    input: ValidateDatasetInput, context: Context
) -> ReportResult:
    session = Session(debug=input.debug, json=context.json)

    report = session.task("Validating dataset", lambda: validate_dataset(input.path))

    return session.render_report_result(report)
