import os

import pytest
from fairspec_metadata import Resource

from fairspec_dataset.actions.file.temp import write_temp_file

from ..prefetch import prefetch_files

REMOTE_URL = "https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv"


@pytest.mark.vcr
class TestPrefetchFiles:
    def test_prefetches_remote_file(self):
        resource = Resource(data=REMOTE_URL)
        result = prefetch_files(resource)
        assert len(result) == 1
        path = result[0]
        assert os.path.exists(path)
        stats = os.stat(path)
        assert stats.st_size == 27
        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert "id,name" in content

    def test_prefetches_remote_file_with_max_bytes(self):
        resource = Resource(data=REMOTE_URL)
        max_bytes = 18
        result = prefetch_files(resource, max_bytes=max_bytes)
        assert len(result) == 1
        path = result[0]
        assert os.path.exists(path)
        stats = os.stat(path)
        assert stats.st_size == max_bytes
        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert "id,name" in content
        assert "1,english" in content
        assert "中文" not in content


class TestPrefetchLocalFiles:
    def test_prefetches_local_paths_in_order(self):
        paths = [write_temp_file(f"part-{index}\n") for index in range(4)]
        resource = Resource(data=paths)

        assert prefetch_files(resource, concurrency=1) == paths
        assert prefetch_files(resource, concurrency=4) == paths
