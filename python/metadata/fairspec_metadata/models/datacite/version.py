from __future__ import annotations

from typing import Annotated

from pydantic import Field

Version = Annotated[
    str,
    Field(
        description="The version number of the resource. Suggested practice: track major_version.minor_version"
    ),
]
