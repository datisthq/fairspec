from typing import Annotated

from fireargs import Option

DataSchemaPath = Annotated[
    str,
    Option(
        name="schema",
        alias="schema",
        description="path to a data schema descriptor (JSON Schema)",
    ),
]
