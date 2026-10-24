use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::{Latitude, Longitude};

pub type GeoLocations = Vec<GeoLocation>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_location_place: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_location_point: Option<GeoLocationPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_location_box: Option<GeoLocationBox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_location_polygon: Option<Vec<GeoLocationPolygonItem>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationPoint {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub point_longitude: Longitude,
    pub point_latitude: Latitude,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationBox {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub west_bound_longitude: Longitude,
    pub east_bound_longitude: Longitude,
    pub south_bound_latitude: Latitude,
    pub north_bound_latitude: Latitude,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocationPolygonItem {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub polygon_point: Option<GeoLocationPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_polygon_point: Option<GeoLocationPoint>,
}
