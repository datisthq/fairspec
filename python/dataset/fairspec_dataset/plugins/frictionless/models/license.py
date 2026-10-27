from __future__ import annotations

from .base import FrictionlessModel


class FrictionlessLicense(FrictionlessModel):
    name: str | None = None
    path: str | None = None
    title: str | None = None
