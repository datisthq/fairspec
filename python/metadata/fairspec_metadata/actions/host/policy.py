from __future__ import annotations

import os
from typing import get_args

from fairspec_metadata.models.host import HostPolicy

HOST_POLICY_ENV = "FAIRSPEC_HOST_POLICY"

_host_policy: HostPolicy | None = None


def get_host_policy() -> HostPolicy:
    """Return the policy remote fetches follow: the one set, then FAIRSPEC_HOST_POLICY, then "any"."""
    if _host_policy is not None:
        return _host_policy

    value = os.environ.get(HOST_POLICY_ENV)
    if not value:
        return "any"
    for policy in get_args(HostPolicy):
        if value == policy:
            return policy
    raise Error(f'Invalid {HOST_POLICY_ENV}: {value} (expected "any" or "public")')


def set_host_policy(policy: HostPolicy | None) -> None:
    """Set the policy remote fetches follow; None falls back to FAIRSPEC_HOST_POLICY."""
    global _host_policy
    _host_policy = policy


class Error(Exception):
    pass
