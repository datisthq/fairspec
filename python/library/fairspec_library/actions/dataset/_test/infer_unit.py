from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import Dataset, Resource

from fairspec_metadata.actions.dataset.validate import validate_dataset_descriptor

from ..infer import infer_dataset


class TestInferDataset:
    def test_should_infer_resource_names(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        dataset = Dataset(resources=[Resource(data=path)])
        result = infer_dataset(dataset)
        assert result.resources is not None
        assert len(result.resources) == 1
        assert result.resources[0].name is not None

    def test_should_not_mutate_original(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        dataset = Dataset(resources=[Resource(data=path)])
        result = infer_dataset(dataset)
        assert result is not dataset

    def test_should_handle_empty_resources(self):
        dataset = Dataset(resources=[])
        result = infer_dataset(dataset)
        assert result.resources == []

    def test_should_handle_no_resources(self):
        dataset = Dataset()
        result = infer_dataset(dataset)
        assert result.resources is None

    def test_should_round_trip_inferred_dataset_through_validation(
        self, tmp_path, monkeypatch
    ):
        monkeypatch.chdir(tmp_path)
        (tmp_path / "products.csv").write_bytes(b"id,name\n1,english\n2,spanish\n")
        dataset = Dataset(resources=[Resource(data="products.csv")])
        inferred = infer_dataset(dataset)
        descriptor = inferred.model_dump(exclude_none=True)
        result = validate_dataset_descriptor(descriptor)
        assert result.valid is True
        assert result.errors == []


class TestInferDatasetConcurrency:
    def test_should_infer_resources_in_order_under_concurrency(self):
        paths = [
            write_temp_file(f"id,name\n{index},english", format="csv")
            for index in range(4)
        ]
        dataset = Dataset(resources=[Resource(data=path) for path in paths])

        serial = infer_dataset(dataset, concurrency=1)
        concurrent = infer_dataset(dataset, concurrency=4)

        assert serial.model_dump() == concurrent.model_dump()
        assert serial.resources is not None
        assert [resource.data for resource in serial.resources] == paths
