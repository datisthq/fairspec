from fairspec_library import validate_data
from fairspec_metadata import Resource, load_data_schema
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import ReportResult
from fairspec_terminal.params import DataSchemaPath, Debug, RequiredFilePath, Silent
from fairspec_terminal.session import Session


class ValidateDataInput(CommandInput):
    path: RequiredFilePath
    data_schema: DataSchemaPath
    silent: Silent = False
    debug: Debug = False


@command(description="Validate a JSON Data from a local or remote path")
def validate_data_command(input: ValidateDataInput, context: Context) -> ReportResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)

    data_schema = session.task(
        "Loading data schema", lambda: load_data_schema(input.data_schema)
    )

    report = session.task(
        "Validating data",
        lambda: validate_data(Resource(data=input.path, dataSchema=data_schema)),
    )

    return session.render_report_result(report)
