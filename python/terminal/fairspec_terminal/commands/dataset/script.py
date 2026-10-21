import code

import fairspec_library
from fairspec_library import load_dataset
from fairspec_metadata import Dataset
from fireargs import command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import Empty
from fairspec_terminal.params import Debug, DescriptorPath
from fairspec_terminal.session import Session


class ScriptDatasetInput(CommandInput):
    path: DescriptorPath
    debug: Debug = False


@command(description="Script a dataset descriptor", json_option=False)
def script_dataset_command(input: ScriptDatasetInput) -> Empty:
    session = Session(debug=input.debug)

    def _load() -> Dataset:
        descriptor = load_dataset(input.path)
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

    return Empty()
