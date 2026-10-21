from typing import Annotated

from fireargs import Argument, Option

FromDataset = Annotated[
    str | None, Option(short="d", description="dataset to select resource from")
]
FromResource = Annotated[
    str | None, Option(short="r", description="resource in provided dataset")
]
DatasetQuery = Annotated[
    str,
    Argument(
        description="a SQL query to execute against a dataset (refer to each table by its resource name)"
    ),
]
