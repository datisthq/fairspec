from fairspec_dataset import FilePreview, preview_file
from fireargs import Context, command

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.params import (
    Debug,
    FilePath,
    FromDataset,
    FromResource,
    Lines,
    SampleBytes,
    Silent,
)
from fairspec_terminal.session import Session


class PreviewFileInput(CommandInput):
    path: FilePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    lines: Lines = None
    sample_bytes: SampleBytes = None
    silent: Silent = False
    debug: Debug = False


@command(description="Show the raw source of a local or remote file")
def preview_file_command(input: PreviewFileInput, context: Context) -> FilePreview:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)
    path = input.path or select_file(
        session, dataset=input.dataset, resource=input.resource
    )

    preview = session.task(
        "Loading file preview",
        lambda: preview_file(path, max_lines=input.lines, max_bytes=input.sample_bytes),
    )

    session.render_raw_result(preview.text)
    return preview
