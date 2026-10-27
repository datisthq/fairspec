from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path
from click.testing import CliRunner

from fairspec_terminal.program import dataset_program


class TestDatasetCopyCommand:
    def test_should_copy_dataset_to_folder(self, monkeypatch):
        source = get_temp_folder_path()
        with open(os.path.join(source, "users.csv"), "w") as file:
            file.write("id,name\n1,Alice\n")
        descriptor = {"resources": [{"name": "users", "data": "users.csv"}]}
        with open(os.path.join(source, "dataset.json"), "w") as file:
            json.dump(descriptor, file)
        monkeypatch.chdir(source)

        runner = CliRunner()
        result = runner.invoke(
            dataset_program, ["copy", "dataset.json", "--to-path", "out"]
        )

        assert result.exit_code == 0
        assert os.path.isfile(os.path.join(source, "out", "dataset.json"))
        assert os.path.isfile(os.path.join(source, "out", "users.csv"))
