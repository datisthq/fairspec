from fairspec_dataset import copy_file

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.params import (
    Debug,
    FromDataset,
    FromResource,
    Json,
    OptionalPath,
    Silent,
    ToPathRequired,
)
from fairspec_terminal.program import file_program
from fairspec_terminal.session import Session


@file_program.command()
def copy(
    path: OptionalPath = None,
    to_path: ToPathRequired = ...,  # type: ignore[assignment]
    dataset: FromDataset = None,
    resource: FromResource = None,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Copy a local or remote file to a local path."""
    session = Session(silent=silent, debug=debug, json=json)

    if not path:
        path = select_file(session, dataset=dataset, resource=resource)

    def _copy() -> None:
        assert path
        copy_file(source_path=path, target_path=to_path)

    session.task("Copying file", _copy)

    session.render_text_result(
        f"Copied file from [bold]{path}[/bold] to [bold]{to_path}[/bold]",
        status="success",
    )
