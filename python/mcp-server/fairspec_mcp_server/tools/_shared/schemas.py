from typing import Annotated

from pydantic import Field

from ...helpers.limit import DEFAULT_LIMIT, MAX_LIMIT

# Tool parameters carry their wire name (`maxErrors`): the SDK passes an alias as the
# keyword, so a snake_case parameter cannot be aliased to match the TypeScript server
MaxErrors = Annotated[
    int | None,
    Field(
        ge=1,
        le=MAX_LIMIT,
        description=f"Maximum number of errors to return (default {DEFAULT_LIMIT})",
    ),
]

Truncated = Annotated[
    bool, Field(description="Whether errors were left out of the report")
]
