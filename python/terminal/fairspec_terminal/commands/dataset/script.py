import code

import fairspec_library
from fairspec_library import load_dataset
from fairspec_metadata import Dataset

from fairspec_terminal.params import Debug, RequiredPath
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command()
def script(
    path: RequiredPath,
    debug: Debug = False,
) -> None:
    """Script a dataset descriptor."""
    session = Session(debug=debug)

    def _load() -> Dataset:
        descriptor = load_dataset(path)
        if not descriptor:
            raise ValueError("Could not load dataset")
        return Dataset.model_validate(descriptor)

    dataset = session.task("Loading dataset", _load)

    session.render_text(
        "[dim]`fairspec` and `dataset` variables are available in the session[/dim]",
        status="warning",
    )

    code.interact(
        banner="",
        local={"fairspec": fairspec_library, "dataset": dataset},
    )
