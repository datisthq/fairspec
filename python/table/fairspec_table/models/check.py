from __future__ import annotations

from typing import Annotated, Required, TypedDict

import polars as pl
from pydantic import Field


class TableCheck(TypedDict, total=False):
    """
    A named condition every row must satisfy, beyond what the Table Schema expresses.
    A condition reducing to one value checks the whole table instead.
    """

    name: Required[str]
    condition: Required[str | pl.Expr]
    columnName: str
    message: str


class SqlTableCheck(TypedDict, total=False):
    """A table check whose condition is polars SQL, the form JSON carries"""

    name: Required[
        Annotated[
            str, Field(description="The name of the check, reported with its errors")
        ]
    ]
    condition: Required[
        Annotated[
            str,
            Field(description="A polars SQL condition; false fails, null passes"),
        ]
    ]
    columnName: Annotated[
        str, Field(description="The column whose cell a failing row is reported against")
    ]
    message: Annotated[str, Field(description="The message reported with each error")]
