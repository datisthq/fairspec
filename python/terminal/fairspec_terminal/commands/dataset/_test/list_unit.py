from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path
from typer.testing import CliRunner

from fairspec_terminal.program import dataset_program, register_commands


class TestDatasetListCommand:
    def test_should_list_resource_names_from_descriptor(self):
        register_commands()
        folder = get_temp_folder_path()
        descriptor = {"resources": [{"name": "products", "data": "products.csv"}]}
        path = os.path.join(folder, "datapackage.json")
        with open(path, "w") as f:
            json.dump(descriptor, f)

        runner = CliRunner()
        result = runner.invoke(dataset_program, ["list", path, "--json"])

        assert result.exit_code == 0
        assert json.loads(result.stdout) == ["products"]
