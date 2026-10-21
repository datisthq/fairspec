from fairspec_library import load_dataset
from fairspec_metadata import Dataset, infer_resource_name
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.params import Debug, DescriptorPath
from fairspec_terminal.session import Session


class ListDatasetInput(CommandInput):
    path: DescriptorPath
    debug: Debug = False


@command(description="List Dataset resources")
def list_dataset_command(input: ListDatasetInput, context: Context) -> list[str]:
    session = Session(debug=input.debug, json=context.json)

    def _load() -> Dataset:
        descriptor = load_dataset(input.path)
        if not descriptor:
            raise ValueError("Could not load dataset")
        return Dataset.model_validate(descriptor)

    dataset = session.task("Loading dataset", _load)

    resource_names = [
        resource.name or infer_resource_name(resource, resource_number=index + 1)
        for index, resource in enumerate(dataset.resources or [])
    ]

    return session.render_data_result(resource_names)
