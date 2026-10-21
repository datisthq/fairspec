from typing import Annotated

from fireargs import Argument
from pydantic import Field

DescriptorPath = Annotated[
    str, Argument(description="local or remote path to the descriptor")
]
TablePath = Annotated[
    str | None, Argument(description="local or remote path to the table")
]
FilePath = Annotated[str | None, Argument(description="local or remote path to the file")]
RequiredFilePath = Annotated[
    str, Argument(description="local or remote path to the file")
]
FilePaths = Annotated[
    list[str], Argument(name="path", description="local paths to files")
]
ToPath = Annotated[str | None, Field(description="a local output path")]
ToPathRequired = Annotated[str, Field(description="a local output path")]
