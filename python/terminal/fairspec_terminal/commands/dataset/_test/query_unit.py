from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path
from click.testing import CliRunner

from fairspec_terminal.program import dataset_program


class TestDatasetQueryCommand:
    def test_should_join_two_resources_by_name(self):
        folder = get_temp_folder_path()
        descriptor = {
            "resources": [
                {
                    "name": "users",
                    "data": [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}],
                },
                {
                    "name": "posts",
                    "data": [{"id": 1, "user_id": 2}, {"id": 2, "user_id": 2}],
                },
            ]
        }
        path = os.path.join(folder, "dataset.json")
        with open(path, "w") as f:
            json.dump(descriptor, f)

        runner = CliRunner()
        result = runner.invoke(
            dataset_program,
            [
                "query",
                path,
                "SELECT users.name, COUNT(*) AS n FROM posts "
                "JOIN users ON posts.user_id = users.id GROUP BY users.name",
                "--json",
            ],
        )

        assert result.exit_code == 0, result.output
        assert json.loads(result.stdout) == [{"name": "bob", "n": 2}]
