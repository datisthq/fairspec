from fairspec_dataset import describe_file

from fairspec_terminal.helpers.file import select_file
from fairspec_terminal.params import (
    Debug,
    FromDataset,
    FromResource,
    HashType,
    Json,
    OptionalPath,
    Silent,
)
from fairspec_terminal.program import file_program
from fairspec_terminal.session import Session


@file_program.command()
def describe(
    path: OptionalPath = None,
    dataset: FromDataset = None,
    resource: FromResource = None,
    hash_type: HashType = "sha256",
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Show stats for a local or remote file."""
    session = Session(silent=silent, debug=debug, json=json)

    if not path:
        path = select_file(session, dataset=dataset, resource=resource)

    def _describe() -> object:
        assert path
        return describe_file(path, hash_type=hash_type)

    stats = session.task("Describing file", _describe)

    session.render_data_result(stats)
