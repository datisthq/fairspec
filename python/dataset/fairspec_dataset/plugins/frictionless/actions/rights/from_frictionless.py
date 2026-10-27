from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor

    from fairspec_dataset.plugins.frictionless.models.license import (
        FrictionlessLicense,
    )


def convert_rights_from_frictionless(
    licenses: list[FrictionlessLicense],
) -> list[Descriptor]:
    rights_list: list[Descriptor] = []
    for license in licenses:
        rights: Descriptor = {}
        title = license.title if license.title is not None else license.name
        if title:
            rights["rights"] = title
        if license.path:
            rights["rightsUri"] = license.path
        if license.name:
            rights["rightsIdentifier"] = license.name
        rights_list.append(rights)
    return rights_list
