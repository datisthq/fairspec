from __future__ import annotations

from typing import TYPE_CHECKING

import re

from fairspec_metadata import (
    IntegrityError,
    MetadataError,
    Report,
    TextualError,
    create_report,
)
from fairspec_metadata import FairspecError

from .infer import infer_hash, infer_textual

if TYPE_CHECKING:
    from fairspec_metadata import Resource

HASH_LENGTHS = {"md5": 32, "sha1": 40, "sha256": 64, "sha512": 128}


def validate_file(resource: Resource, *, concurrency: int | None = None) -> Report:
    errors: list[FairspecError] = []

    if resource.textual:
        actual_textual = infer_textual(resource)

        if not actual_textual:
            errors.append(TextualError(type="file/textual"))

    integrity = resource.integrity
    if integrity:
        expected_hash = integrity.hash
        length = HASH_LENGTHS[integrity.type]
        if not re.fullmatch(f"[0-9a-f]{{{length}}}", expected_hash):
            errors.append(
                MetadataError(
                    type="metadata",
                    message=f"must be {length} lowercase hexadecimal digits",
                    jsonPointer="/integrity/hash",
                )
            )
            return create_report(errors)

        actual_hash = infer_hash(
            resource, hash_type=integrity.type, concurrency=concurrency
        )

        if actual_hash != expected_hash:
            errors.append(
                IntegrityError(
                    type="file/integrity",
                    hashType=integrity.type,
                    expectedHash=expected_hash,
                    actualHash=actual_hash or "",
                )
            )

    return create_report(errors)
