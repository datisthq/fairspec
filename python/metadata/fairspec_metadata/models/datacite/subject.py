from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel


class Subject(FairspecModel):
    subject: str = Field(
        description="Subject, keyword, classification code, or key phrase describing the resource"
    )
    subjectScheme: str | None = Field(
        default=None,
        description="The name of the subject scheme or classification code or authority if one is used",
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the subject identifier scheme",
    )
    valueUri: str | None = Field(
        default=None,
        description="The URI of the subject term",
    )
    classificationCode: str | None = Field(
        default=None,
        description="The classification code used for the subject term in the subject scheme",
    )
    lang: str | None = Field(
        default=None,
        description="Language of the subject, specified using ISO 639-1 or ISO 639-3 codes",
    )


Subjects = Annotated[
    list[Subject],
    Field(
        description="Subject, keyword, classification code, or key phrase describing the resource"
    ),
]
