macro_rules! define_column {
    ($name:ident, $property:ty) => {
        #[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            pub name: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub required: Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub nullable: Option<bool>,
            pub property: $property,
        }
    };
}

pub mod array;
pub mod base;
pub mod base64;
pub mod boolean;
pub mod categorical;
#[allow(clippy::module_inception)]
pub mod column;
pub mod date;
pub mod date_time;
pub mod decimal;
pub mod duration;
pub mod email;
pub mod geojson;
pub mod hex;
pub mod integer;
pub mod list;
pub mod local_date_time;
pub mod local_time;
pub mod number;
pub mod object;
pub mod string;
pub mod time;
pub mod topojson;
pub mod unknown;
pub mod url;
pub mod wkb;
pub mod wkt;
