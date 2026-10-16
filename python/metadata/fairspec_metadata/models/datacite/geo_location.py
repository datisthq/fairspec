from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel

from .common import Latitude, Longitude


class GeoLocationPoint(FairspecModel):
    pointLongitude: Longitude = Field(description="Longitudinal dimension of point")
    pointLatitude: Latitude = Field(description="Latitudinal dimension of point")


class GeoLocationBox(FairspecModel):
    westBoundLongitude: Longitude = Field(
        description="Western longitudinal dimension of box"
    )
    eastBoundLongitude: Longitude = Field(
        description="Eastern longitudinal dimension of box"
    )
    southBoundLatitude: Latitude = Field(
        description="Southern latitudinal dimension of box"
    )
    northBoundLatitude: Latitude = Field(
        description="Northern latitudinal dimension of box"
    )


class GeoLocationPolygonItem(FairspecModel):
    polygonPoint: GeoLocationPoint | None = Field(
        default=None,
        description="A point location in a polygon",
    )
    inPolygonPoint: GeoLocationPoint | None = Field(
        default=None,
        description="For any bound area that is larger than half the earth, define a (random) point inside",
    )


class GeoLocation(FairspecModel):
    geoLocationPlace: str | None = Field(
        default=None,
        description="Spatial region or named place where the data was gathered or about which the data is focused",
    )
    geoLocationPoint: GeoLocationPoint | None = Field(
        default=None,
        description="A point location in space",
    )
    geoLocationBox: GeoLocationBox | None = Field(
        default=None,
        description="The spatial limits of a box",
    )
    geoLocationPolygon: list[GeoLocationPolygonItem] | None = Field(
        default=None,
        description="A drawn polygon area, defined by a set of points and lines connecting the points in a closed chain",
    )


GeoLocations = Annotated[
    list[GeoLocation],
    Field(
        description="Spatial region or named place where the data was gathered or about which the data is focused"
    ),
]
