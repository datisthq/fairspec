from fairspec_metadata import validate_table_schema
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import ReportResult
from fairspec_terminal.params import Debug, RequiredFilePath, Silent
from fairspec_terminal.session import Session


class ValidateTableSchemaInput(CommandInput):
    path: RequiredFilePath
    silent: Silent = False
    debug: Debug = False


@command(description="Validate a Table Schema")
def validate_table_schema_command(
    input: ValidateTableSchemaInput, context: Context
) -> ReportResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)

    report = session.task(
        "Validating table schema", lambda: validate_table_schema(input.path)
    )

    return session.render_report_result(report)
