from fairspec_dataset import preview_file

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.params import (
    Debug,
    FromDataset,
    FromResource,
    Json,
    Lines,
    OptionalPath,
    SampleBytes,
    Silent,
)
from fairspec_terminal.program import file_program
from fairspec_terminal.session import Session


@file_program.command()
def preview(
    path: OptionalPath = None,
    dataset: FromDataset = None,
    resource: FromResource = None,
    lines: Lines = None,
    sample_bytes: SampleBytes = None,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Show the raw source of a local or remote file."""
    session = Session(silent=silent, debug=debug, json=json)

    if not path:
        path = select_file(session, dataset=dataset, resource=resource)

    def _preview():
        assert path
        return preview_file(path, max_lines=lines, max_bytes=sample_bytes)

    result = session.task("Loading file preview", _preview)

    if json:
        session.render_data_result(result.model_dump())
        return

    session.render_raw_result(result.text)
