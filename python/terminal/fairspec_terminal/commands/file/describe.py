from fairspec_dataset import FileDescription, describe_file
from fireargs import Context, command

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.params import (
    Debug,
    FilePath,
    FromDataset,
    FromResource,
    HashType,
    Silent,
)
from fairspec_terminal.session import Session


class DescribeFileInput(CommandInput):
    path: FilePath = None
    dataset: FromDataset = None
    resource: FromResource = None
    hash_type: HashType = "sha256"
    silent: Silent = False
    debug: Debug = False


@command(description="Show stats for a local or remote file")
def describe_file_command(input: DescribeFileInput, context: Context) -> FileDescription:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)
    path = input.path or select_file(
        session, dataset=input.dataset, resource=input.resource
    )

    stats = session.task(
        "Describing file", lambda: describe_file(path, hash_type=input.hash_type)
    )

    return session.render_data_result(stats)
