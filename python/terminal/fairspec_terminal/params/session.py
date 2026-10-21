from typing import Annotated

from pydantic import Field

Debug = Annotated[
    bool, Field(description="Enable debug mode to print exception details to stderr")
]
Silent = Annotated[bool, Field(description="suppress all output except errors")]
