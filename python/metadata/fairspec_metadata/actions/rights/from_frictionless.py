from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata.actions.frictionless.mapping import FrictionlessRecorder
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.frictionless.license import FrictionlessLicense


def convert_rights_from_frictionless(
    licenses: list[FrictionlessLicense], *, record: FrictionlessRecorder | None = None
) -> list[Descriptor]:
    rights_list: list[Descriptor] = []
    for index, license in enumerate(licenses):
        rights: Descriptor = {}
        title = license.title if license.title is not None else license.name
        if title:
            rights["rights"] = title
        if license.path:
            rights["rightsUri"] = license.path
        if license.name:
            rights["rightsIdentifier"] = license.name

        if record is not None:
            record(f"/{index}", f"/{index}")
            title_key = "title" if license.title else "name"
            record(f"/{index}/rights", f"/{index}/{title_key}")
            record(f"/{index}/rightsUri", f"/{index}/path")
            record(f"/{index}/rightsIdentifier", f"/{index}/name")

        rights_list.append(rights)
    return rights_list
