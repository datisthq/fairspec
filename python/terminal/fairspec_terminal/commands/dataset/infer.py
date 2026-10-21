from fairspec_library import infer_dataset
from fairspec_metadata import Dataset, Resource
from fireargs import Context, command

from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.params import Debug, FilePaths, TableSchemaParams
from fairspec_terminal.session import Session


class InferDatasetInput(TableSchemaParams):
    paths: FilePaths
    debug: Debug = False


@command(description="Infer a dataset from local or remote file paths")
def infer_dataset_command(input: InferDatasetInput, context: Context) -> Dataset:
    session = Session(debug=input.debug, json=context.json)
    options = create_infer_table_schema_options(input)

    def _infer() -> Dataset:
        dataset = Dataset(resources=[Resource(data=path) for path in input.paths])
        return infer_dataset(dataset, **options)

    dataset = session.task("Inferring dataset", _infer)

    return session.render_data_result(dataset)
