from __future__ import annotations

import polars as pl

from fairspec_metadata import CellTypeError, GeojsonColumn, GeojsonColumnProperty

from ..geojson import inspect_geojson_column


class TestInspectGeojsonColumn:
    def test_valid_geojson_point(self):
        table = pl.DataFrame(
            {
                "location": [
                    '{"type":"Point","coordinates":[0,0]}',
                    '{"type":"Point","coordinates":[12.5,41.9]}',
                    '{"type":"Point","coordinates":[-73.9,40.7]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="location",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 0

    def test_valid_geojson_geometries(self):
        table = pl.DataFrame(
            {
                "geometry": [
                    '{"type":"LineString","coordinates":[[0,0],[1,1]]}',
                    '{"type":"Polygon","coordinates":[[[0,0],[1,0],[1,1],[0,1],[0,0]]]}',
                    '{"type":"MultiPoint","coordinates":[[0,0],[1,1]]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="geometry",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 0

    def test_valid_positions_with_altitude(self):
        table = pl.DataFrame(
            {
                "location": [
                    '{"type":"Point","coordinates":[1,2,3]}',
                    '{"type":"LineString","coordinates":[[0,0,1],[1,1,2]]}',
                    '{"type":"Point","coordinates":[1,2,3,4]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="location",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 0

    def test_null_values(self):
        table = pl.DataFrame(
            {
                "location": [
                    '{"type":"Point","coordinates":[0,0]}',
                    None,
                    '{"type":"LineString","coordinates":[[0,0],[1,1]]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="location",
            type="geojson",
            nullable=True,
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 0

    def test_json_arrays_error(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"type":"Point","coordinates":[0,0]}',
                    "[[0,0],[1,1]]",
                    '{"type":"LineString","coordinates":[[0,0],[1,1]]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="data",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 1
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].columnName == "data"
        assert errors[0].columnType == "geojson"
        assert errors[0].rowNumber == 2
        assert errors[0].cell == "[[0,0],[1,1]]"

    def test_invalid_json_error(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"type":"Point","coordinates":[0,0]}',
                    "invalid json",
                    "{broken}",
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="data",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        type_errors = [e for e in errors if e.type == "cell/type"]
        assert len(type_errors) == 2
        assert any(e.rowNumber == 2 and e.cell == "invalid json" for e in type_errors)
        assert any(e.rowNumber == 3 and e.cell == "{broken}" for e in type_errors)

    def test_empty_strings_error(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"type":"Point","coordinates":[0,0]}',
                    "",
                    '{"type":"LineString","coordinates":[[0,0],[1,1]]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="data",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"
        assert errors[0].rowNumber == 2
        assert errors[0].cell == ""

    def test_json_primitives_error(self):
        table = pl.DataFrame(
            {
                "data": ['"string"', "123", "true", "false", "null"],
            }
        ).lazy()
        column = GeojsonColumn(
            name="data",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 5
        assert errors[0].rowNumber == 1
        assert errors[0].cell == '"string"'
        assert errors[1].rowNumber == 2
        assert errors[1].cell == "123"
        assert errors[2].rowNumber == 3
        assert errors[2].cell == "true"
        assert errors[3].rowNumber == 4
        assert errors[3].cell == "false"
        assert errors[4].rowNumber == 5
        assert errors[4].cell == "null"

    def test_invalid_geojson_point_coordinates(self):
        table = pl.DataFrame(
            {
                "location": [
                    '{"type":"Point","coordinates":[0,0]}',
                    '{"type":"Point","coordinates":[0]}',
                    '{"type":"Point","coordinates":[0,"0"]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="location",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 2
        assert any(
            e.rowNumber == 2 and e.cell == '{"type":"Point","coordinates":[0]}'
            for e in errors
        )
        assert any(
            e.rowNumber == 3 and e.cell == '{"type":"Point","coordinates":[0,"0"]}'
            for e in errors
        )

    def test_invalid_geojson_linestring(self):
        table = pl.DataFrame(
            {
                "line": [
                    '{"type":"LineString","coordinates":[[0,0],[1,1]]}',
                    '{"type":"LineString","coordinates":[[0,0]]}',
                    '{"type":"LineString","coordinates":[0,0]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="line",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert len(errors) == 2
        assert any(
            e.rowNumber == 2 and e.cell == '{"type":"LineString","coordinates":[[0,0]]}'
            for e in errors
        )
        assert any(
            e.rowNumber == 3 and e.cell == '{"type":"LineString","coordinates":[0,0]}'
            for e in errors
        )

    def test_feature_and_feature_collection_error(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"type":"Point","coordinates":[0,0]}',
                    '{"type":"Feature","geometry":{"type":"Point","coordinates":[1,2]},"properties":{}}',
                    '{"type":"FeatureCollection","features":[]}',
                ],
            }
        ).lazy()
        column = GeojsonColumn(
            name="data",
            type="geojson",
            property=GeojsonColumnProperty(),
        )

        errors = inspect_geojson_column(column, table)

        assert [(e.type, e.rowNumber, e.cell) for e in errors] == [
            (
                "cell/type",
                2,
                '{"type":"Feature","geometry":{"type":"Point","coordinates":[1,2]},"properties":{}}',
            ),
            ("cell/type", 3, '{"type":"FeatureCollection","features":[]}'),
        ]
