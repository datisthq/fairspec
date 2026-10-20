import polars as pl
from fairspec_library import query_dataset

from fairspec_terminal.params import DatasetQuery, Debug, Json, RequiredPath
from fairspec_terminal.program import dataset_program
from fairspec_terminal.session import Session


@dataset_program.command()
def query(
    path: RequiredPath,
    sql_query: DatasetQuery,
    json: Json = False,
    debug: Debug = False,
) -> None:
    """Query a dataset from a local or remote path with SQL."""
    session = Session(debug=debug, json=json)

    def _query() -> pl.LazyFrame:
        return query_dataset(path, sql_query)

    table = session.task("Executing query", _query)

    def _collect() -> pl.DataFrame:
        result = table.collect()
        assert isinstance(result, pl.DataFrame)
        return result

    frame = session.task("Collecting data", _collect)

    session.render_frame_result(frame)
