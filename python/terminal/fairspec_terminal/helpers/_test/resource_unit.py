from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path

from fairspec_terminal.helpers.resource import select_resource
from fairspec_terminal.session import Session


class TestSelectResource:
    def test_should_select_resource_by_name(self):
        folder = get_temp_folder_path()
        path = os.path.join(folder, "datapackage.json")
        descriptor = {
            "resources": [
                {"name": "users", "data": "users.csv"},
                {"name": "orders", "data": "orders.csv"},
            ]
        }
        with open(path, "w") as file:
            json.dump(descriptor, file)

        session = Session(json=True)
        resource = select_resource(session, dataset=path, resource="orders")

        assert resource.name == "orders"
