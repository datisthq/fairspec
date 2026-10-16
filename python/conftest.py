from __future__ import annotations

import os

import pytest


@pytest.fixture(scope="module")
def vcr_cassette_dir(request):
    return os.path.join(os.path.dirname(request.fspath), "fixtures", "generated")
