from __future__ import annotations

from pydantic import ConfigDict, JsonValue, ValidationInfo, field_validator

from fairspec_metadata.models.base import FairspecModel


class FrictionlessModel(FairspecModel):
    # Lax mode would accept `"bytes": "5"` or `"header": "yes"`, which zod rejects
    model_config = ConfigDict(strict=True, populate_by_name=True)

    # An optional zod key rejects `null`, a `T | None` field would accept it
    @field_validator("*", mode="before")
    @classmethod
    def reject_null(cls, value: object, info: ValidationInfo) -> object:
        if value is None and info.field_name is not None:
            if cls.model_fields[info.field_name].annotation is not JsonValue:
                raise ValueError("Input should not be null")
        return value
