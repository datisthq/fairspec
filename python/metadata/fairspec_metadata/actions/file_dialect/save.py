from __future__ import annotations

from fairspec_metadata.actions.descriptor.save import save_descriptor
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect
from fairspec_metadata.settings import FAIRSPEC_VERSION


def save_file_dialect(
    file_dialect: FileDialect,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    descriptor = file_dialect.model_dump(by_alias=True, exclude_none=True)

    if "$schema" not in descriptor:
        descriptor["$schema"] = (
            f"https://fairspec.org/profiles/{FAIRSPEC_VERSION}/file-dialect.json"
        )

    save_descriptor(descriptor, path=path, overwrite=overwrite)
