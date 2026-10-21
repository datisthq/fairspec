import polars as pl
from fairspec_library import query_dataset
from fireargs import Context, command

from fairspec_terminal.models.input import CommandInput
from fairspec_terminal.models.output import Records
from fairspec_terminal.params import DatasetQuery, Debug, DescriptorPath
from fairspec_terminal.session import Session


class QueryDatasetInput(CommandInput):
    path: DescriptorPath
    query: DatasetQuery
    debug: Debug = False


@command(description="Query a dataset from a local or remote path with SQL")
def query_dataset_command(input: QueryDatasetInput, context: Context) -> Records:
    session = Session(debug=input.debug, json=context.json)

    table = session.task(
        "Executing query", lambda: query_dataset(input.path, input.query)
    )

    def _collect() -> pl.DataFrame:
        result = table.collect()
        assert isinstance(result, pl.DataFrame)
        return result

    frame = session.task("Collecting data", _collect)

    return session.render_frame_result(frame)
