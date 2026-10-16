from fairspec_library import load_dataset
from fairspec_metadata import Dataset, infer_resource_name

from fairspec_terminal.params import Debug, Json, RequiredPath
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command(name="list")
def list_(
    path: RequiredPath,
    json: Json = False,
    debug: Debug = False,
) -> None:
    """List Dataset resources."""
    session = Session(debug=debug, json=json)

    def _load() -> Dataset:
        descriptor = load_dataset(path)
        if not descriptor:
            raise ValueError("Could not load dataset")
        return Dataset.model_validate(descriptor)

    dataset = session.task("Loading dataset", _load)

    resource_names = [
        resource.name or infer_resource_name(resource, resource_number=index + 1)
        for index, resource in enumerate(dataset.resources or [])
    ]

    session.render_data_result(resource_names)
