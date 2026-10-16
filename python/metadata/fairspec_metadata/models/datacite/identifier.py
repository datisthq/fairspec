from __future__ import annotations

from typing import Annotated

from pydantic import Field

Doi = Annotated[
    str,
    Field(
        pattern=r"^10[.][0-9]{4,9}[/][^\s]+$",
        description="The Digital Object Identifier (DOI) is a persistent identifier for the resource, following the DOI syntax",
    ),
]

DoiPrefix = Annotated[
    str,
    Field(
        pattern=r"^10[.][0-9]{4,9}$",
        description="The DOI prefix, which is the part of the DOI before the slash. It uniquely identifies the registrant",
    ),
]

DoiSuffix = Annotated[
    str,
    Field(
        pattern=r"^[^\s]+$",
        description="The DOI suffix, which is the part of the DOI after the slash. It is assigned by the registrant",
    ),
]
