from __future__ import annotations

import polars as pl

from fairspec_metadata import TopojsonColumn, TopojsonColumnProperty

from ..topojson import inspect_topojson_column


class TestInspectTopojsonColumn:
    def test_valid_topojson(self):
        table = pl.DataFrame(
            {
                "topology": [
                    '{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[{"type":"Point","coordinates":[0,0]}]}},"arcs":[]}',
                    '{"type":"Topology","objects":{"collection":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}',
                ],
            }
        ).lazy()
        column = TopojsonColumn(
            name="topology",
            type="topojson",
            property=TopojsonColumnProperty(),
        )

        errors = inspect_topojson_column(column, table)

        assert len(errors) == 0

    def test_invalid_topojson_structure(self):
        table = pl.DataFrame(
            {
                "topology": [
                    '{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}',
                    '{"type":"Topology","objects":{}}',
                    '{"type":"Topology"}',
                ],
            }
        ).lazy()
        column = TopojsonColumn(
            name="topology",
            type="topojson",
            property=TopojsonColumnProperty(),
        )

        errors = inspect_topojson_column(column, table)

        assert len(errors) == 2
        assert any(
            e.rowNumber == 2 and e.cell == '{"type":"Topology","objects":{}}'
            for e in errors
        )
        assert any(e.rowNumber == 3 and e.cell == '{"type":"Topology"}' for e in errors)

    def test_topojson_geometry_objects(self):
        table = pl.DataFrame(
            {
                "geometry": [
                    '{"type":"Point","coordinates":[0,0]}',
                    '{"type":"LineString","arcs":[0,1]}',
                    '{"type":"Polygon","arcs":[[0,1,2]]}',
                ],
            }
        ).lazy()
        column = TopojsonColumn(
            name="geometry",
            type="topojson",
            property=TopojsonColumnProperty(),
        )

        errors = inspect_topojson_column(column, table)

        assert len(errors) == 0

    def test_null_values(self):
        table = pl.DataFrame(
            {
                "topology": [
                    '{"type":"Topology","objects":{"example":{"type":"GeometryCollection","geometries":[]}},"arcs":[]}',
                    None,
                ],
            }
        ).lazy()
        column = TopojsonColumn(
            name="topology",
            type="topojson",
            property=TopojsonColumnProperty(),
        )

        errors = inspect_topojson_column(column, table)

        assert len(errors) == 0
