from typing import Annotated

from fireargs import Argument, Option
from pydantic import AfterValidator

TableQuery = Annotated[
    str | None,
    Argument(
        description="a SQL query to execute against a table (use `self` to refer to the table)"
    ),
]


def assert_table_check(value: str) -> str:
    name, separator, _ = value.partition("=")
    if not name or not separator:
        raise ValueError('Expected "name=condition"')
    return value


Check = Annotated[
    list[Annotated[str, AfterValidator(assert_table_check)]] | None,
    Option(
        help_group="Checks",
        description="a named SQL condition every row must satisfy, as name=condition (repeatable)",
    ),
]
