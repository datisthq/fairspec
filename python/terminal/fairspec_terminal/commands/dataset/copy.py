from fairspec_library import load_dataset, save_dataset
from fairspec_metadata import Dataset
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import TextResult
from fairspec_terminal.params import Debug, DescriptorPath, Silent, ToPathRequired
from fairspec_terminal.session import Session


class CopyDatasetInput(CommandInput):
    path: DescriptorPath
    to_path: ToPathRequired
    silent: Silent = False
    debug: Debug = False


@command(description="Copy a local or remote dataset to a local folder")
def copy_dataset_command(input: CopyDatasetInput, context: Context) -> TextResult:
    session = Session(silent=input.silent, debug=input.debug, json=context.json)

    def _copy() -> None:
        descriptor = load_dataset(input.path)
        if not descriptor:
            raise ValueError("Could not load dataset")
        result = save_dataset(Dataset.model_validate(descriptor), target=input.to_path)
        if result is None:
            raise ValueError(f'Could not copy dataset to "{input.to_path}"')

    session.task("Copy dataset", _copy)

    return session.render_text_result(
        f"Copied dataset from [bold]{input.path}[/bold] to [bold]{input.to_path}[/bold]",
        status="success",
    )
