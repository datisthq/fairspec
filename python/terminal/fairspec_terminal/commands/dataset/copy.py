from fairspec_library import load_dataset, save_dataset
from fairspec_metadata import Dataset

from fairspec_terminal.params import Debug, Json, RequiredPath, Silent, ToPathRequired
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command()
def copy(
    path: RequiredPath,
    to_path: ToPathRequired,
    silent: Silent = False,
    debug: Debug = False,
    json: Json = False,
) -> None:
    """Copy a local or remote dataset to a local folder."""
    session = Session(silent=silent, debug=debug, json=json)

    def _copy() -> None:
        descriptor = load_dataset(path)
        if not descriptor:
            raise ValueError("Could not load dataset")
        result = save_dataset(Dataset.model_validate(descriptor), target=to_path)
        if result is None:
            raise ValueError(f'Could not copy dataset to "{to_path}"')

    session.task("Copy dataset", _copy)

    session.render_text_result(
        f"Copied dataset from [bold]{path}[/bold] to [bold]{to_path}[/bold]",
        status="success",
    )
