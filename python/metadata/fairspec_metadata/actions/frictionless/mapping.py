from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Protocol

from fairspec_metadata.models.error.foreign_key import ForeignKeyError

if TYPE_CHECKING:
    from fairspec_metadata.models.error.error import FairspecError


@dataclass
class FrictionlessPointer:
    fairspec: str
    frictionless: str
    verbatim: bool


@dataclass
class FrictionlessMapping:
    """How a converted Frictionless descriptor maps back to the one the user wrote"""

    pointers: list[FrictionlessPointer] = field(default_factory=list)
    resource_names: dict[str, str] = field(default_factory=dict)


class FrictionlessRecorder(Protocol):
    """Record that a Fairspec pointer was written from a Frictionless one

    A verbatim value was copied as it is, so pointers below it map key for key
    """

    def __call__(
        self, fairspec: str, frictionless: str, *, verbatim: bool = False
    ) -> None: ...


def create_frictionless_recorder(
    mapping: FrictionlessMapping,
    *,
    fairspec_prefix: str = "",
    frictionless_prefix: str = "",
) -> FrictionlessRecorder:
    def record(fairspec: str, frictionless: str, *, verbatim: bool = False) -> None:
        mapping.pointers.append(
            FrictionlessPointer(
                fairspec=fairspec_prefix + fairspec,
                frictionless=frictionless_prefix + frictionless,
                verbatim=verbatim,
            )
        )

    return record


def nest_frictionless_recorder(
    record: FrictionlessRecorder, fairspec_prefix: str, frictionless_prefix: str
) -> FrictionlessRecorder:
    def nested(fairspec: str, frictionless: str, *, verbatim: bool = False) -> None:
        record(
            fairspec_prefix + fairspec,
            frictionless_prefix + frictionless,
            verbatim=verbatim,
        )

    return nested


def skip_frictionless_record(
    fairspec: str, frictionless: str, *, verbatim: bool = False
) -> None:
    pass


def map_frictionless_errors(
    errors: list[FairspecError], mapping: FrictionlessMapping
) -> list[FairspecError]:
    """Rewrite errors about a converted descriptor to point at the Frictionless one"""
    names = mapping.resource_names
    mapped_errors: list[FairspecError] = []

    for error in errors:
        update: dict[str, object] = {}

        pointer = getattr(error, "jsonPointer", None)
        if isinstance(pointer, str):
            update["jsonPointer"] = _map_frictionless_pointer(pointer, mapping)

        if error.resourceName and error.resourceName in names:
            update["resourceName"] = names[error.resourceName]

        if isinstance(error, ForeignKeyError):
            reference = error.foreignKey.reference
            if reference.resource and reference.resource in names:
                update["foreignKey"] = error.foreignKey.model_copy(
                    update={
                        "reference": reference.model_copy(
                            update={"resource": names[reference.resource]}
                        )
                    }
                )

        mapped_errors.append(error.model_copy(update=update) if update else error)

    return mapped_errors


def _map_frictionless_pointer(pointer: str, mapping: FrictionlessMapping) -> str:
    best: FrictionlessPointer | None = None
    for entry in mapping.pointers:
        is_prefix = pointer == entry.fairspec or pointer.startswith(f"{entry.fairspec}/")
        if is_prefix and (best is None or len(entry.fairspec) > len(best.fairspec)):
            best = entry

    if best is None:
        return pointer
    if not best.verbatim:
        return best.frictionless
    return best.frictionless + pointer[len(best.fairspec) :]
