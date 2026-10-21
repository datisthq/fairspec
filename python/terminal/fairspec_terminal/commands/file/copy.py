from fairspec_dataset import copy_file
from fireargs import Context, command

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import TextResult
from fairspec_terminal.params import (
    Debug,
    FilePath,
    FromDataset,
    FromResource,
    Silent,
    ToPathRequired,
)
from fairspec_terminal.session import Session


class CopyFileInput(CommandInput):
    path: FilePath = None
    to_path: ToPathRequired
    dataset: FromDataset = None
    resource: FromResource = None
    silent: Silent = False
    debug: Debug = False


@command(description="Copy a local or remote file to a local path")
def copy_file_command(input: CopyFileInput, context: Context) -> TextResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)
    path = input.path or select_file(
        session, dataset=input.dataset, resource=input.resource
    )

    session.task(
        "Copying file", lambda: copy_file(source_path=path, target_path=input.to_path)
    )

    return session.render_text_result(
        f"Copied file from [bold]{path}[/bold] to [bold]{input.to_path}[/bold]",
        status="success",
    )
