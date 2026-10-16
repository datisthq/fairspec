from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path
from typer.testing import CliRunner

from fairspec_terminal.program import dataset_program, register_commands


class TestDatasetCopyCommand:
    def test_should_copy_dataset_to_folder(self, monkeypatch):
        register_commands()
        source = get_temp_folder_path()
        with open(os.path.join(source, "users.csv"), "w") as file:
            file.write("id,name\n1,Alice\n")
        descriptor = {"resources": [{"name": "users", "data": "users.csv"}]}
        with open(os.path.join(source, "datapackage.json"), "w") as file:
            json.dump(descriptor, file)
        monkeypatch.chdir(source)

        runner = CliRunner()
        result = runner.invoke(
            dataset_program, ["copy", "datapackage.json", "--to-path", "out"]
        )

        assert result.exit_code == 0
        assert os.path.isfile(os.path.join(source, "out", "dataset.json"))
        assert os.path.isfile(os.path.join(source, "out", "users.csv"))
