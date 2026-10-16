from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import Resource, get_data_first_path

from .resource import select_resource

if TYPE_CHECKING:
    from fairspec_terminal.session import Session


def select_file(
    session: Session,
    *,
    dataset: str | None = None,
    resource: str | None = None,
) -> str:
    selected = select_resource(session, dataset=dataset, resource=resource)

    path = session.task("Selecting file", lambda: _get_first_path(selected))

    return path


def _get_first_path(resource: Resource) -> str:
    first_path = get_data_first_path(resource)
    if not first_path:
        raise ValueError("Resource does not have files")
    return first_path
