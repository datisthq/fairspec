use crate::actions::column::property::get_base_property_type;
use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::array::{ArrayColumn, ArrayColumnProperty};
use super::base::{BasePropertyType, NullPosition};
use super::base64::{Base64Column, Base64ColumnProperty};
use super::boolean::{BooleanColumn, BooleanColumnProperty};
use super::categorical::{
    CategoricalColumn, CategoricalColumnProperty, IntegerCategoricalColumnProperty,
    StringCategoricalColumnProperty,
};
use super::date::{DateColumn, DateColumnProperty};
use super::date_time::{DateTimeColumn, DateTimeColumnProperty};
use super::decimal::{DecimalColumn, DecimalColumnProperty};
use super::duration::{DurationColumn, DurationColumnProperty};
use super::email::{EmailColumn, EmailColumnProperty};
use super::geojson::{GeojsonColumn, GeojsonColumnProperty};
use super::hex::{HexColumn, HexColumnProperty};
use super::integer::{IntegerColumn, IntegerColumnProperty};
use super::list::{ListColumn, ListColumnProperty};
use super::local_date_time::{LocalDateTimeColumn, LocalDateTimeColumnProperty};
use super::local_time::{LocalTimeColumn, LocalTimeColumnProperty};
use super::number::{NumberColumn, NumberColumnProperty};
use super::object::{ObjectColumn, ObjectColumnProperty};
use super::string::{StringColumn, StringColumnProperty};
use super::time::{TimeColumn, TimeColumnProperty};
use super::topojson::{TopojsonColumn, TopojsonColumnProperty};
use super::unknown::{UnknownColumn, UnknownColumnProperty};
use super::url::{UrlColumn, UrlColumnProperty};
use super::wkb::{WkbColumn, WkbColumnProperty};
use super::wkt::{WktColumn, WktColumnProperty};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Column {
    Array(ArrayColumn),
    Base64(Base64Column),
    Boolean(BooleanColumn),
    Categorical(CategoricalColumn),
    Date(DateColumn),
    DateTime(DateTimeColumn),
    Decimal(DecimalColumn),
    Duration(DurationColumn),
    Email(EmailColumn),
    Geojson(GeojsonColumn),
    Hex(HexColumn),
    Integer(IntegerColumn),
    List(ListColumn),
    LocalDateTime(LocalDateTimeColumn),
    LocalTime(LocalTimeColumn),
    Number(NumberColumn),
    Object(ObjectColumn),
    String(StringColumn),
    Time(TimeColumn),
    Topojson(TopojsonColumn),
    Unknown(UnknownColumn),
    Url(UrlColumn),
    Wkb(WkbColumn),
    Wkt(WktColumn),
}

// Every column struct has the same `name`, `required` and `nullable` fields
macro_rules! match_column {
    ($column:expr, $inner:ident => $body:expr) => {
        match $column {
            Column::Array($inner) => $body,
            Column::Base64($inner) => $body,
            Column::Boolean($inner) => $body,
            Column::Categorical($inner) => $body,
            Column::Date($inner) => $body,
            Column::DateTime($inner) => $body,
            Column::Decimal($inner) => $body,
            Column::Duration($inner) => $body,
            Column::Email($inner) => $body,
            Column::Geojson($inner) => $body,
            Column::Hex($inner) => $body,
            Column::Integer($inner) => $body,
            Column::List($inner) => $body,
            Column::LocalDateTime($inner) => $body,
            Column::LocalTime($inner) => $body,
            Column::Number($inner) => $body,
            Column::Object($inner) => $body,
            Column::String($inner) => $body,
            Column::Time($inner) => $body,
            Column::Topojson($inner) => $body,
            Column::Unknown($inner) => $body,
            Column::Url($inner) => $body,
            Column::Wkb($inner) => $body,
            Column::Wkt($inner) => $body,
        }
    };
}

impl Column {
    /// Wraps a property in the column of the kind its variant implies.
    pub fn from_property(name: impl Into<String>, property: ColumnProperty) -> Self {
        use ColumnProperty as P;
        let name = name.into();
        macro_rules! column {
            ($kind:ident, $column:ident, $property:expr) => {
                Column::$kind($column {
                    name,
                    required: None,
                    nullable: None,
                    property: $property,
                })
            };
        }
        match property {
            P::Boolean(p) => column!(Boolean, BooleanColumn, p),
            P::Integer(p) => column!(Integer, IntegerColumn, p),
            P::IntegerCategorical(p) => {
                column!(
                    Categorical,
                    CategoricalColumn,
                    CategoricalColumnProperty::Integer(p)
                )
            }
            P::Number(p) => column!(Number, NumberColumn, p),
            P::List(p) => column!(List, ListColumn, p),
            P::Base64(p) => column!(Base64, Base64Column, p),
            P::Hex(p) => column!(Hex, HexColumn, p),
            P::Email(p) => column!(Email, EmailColumn, p),
            P::Url(p) => column!(Url, UrlColumn, p),
            P::DateTime(p) => column!(DateTime, DateTimeColumn, p),
            P::LocalDateTime(p) => column!(LocalDateTime, LocalDateTimeColumn, p),
            P::Date(p) => column!(Date, DateColumn, p),
            P::Time(p) => column!(Time, TimeColumn, p),
            P::LocalTime(p) => column!(LocalTime, LocalTimeColumn, p),
            P::Duration(p) => column!(Duration, DurationColumn, p),
            P::Wkt(p) => column!(Wkt, WktColumn, p),
            P::Wkb(p) => column!(Wkb, WkbColumn, p),
            P::StringCategorical(p) => {
                column!(
                    Categorical,
                    CategoricalColumn,
                    CategoricalColumnProperty::String(p)
                )
            }
            P::Decimal(p) => column!(Decimal, DecimalColumn, p),
            P::String(p) => column!(String, StringColumn, p),
            P::Array(p) => column!(Array, ArrayColumn, p),
            P::Geojson(p) => column!(Geojson, GeojsonColumn, p),
            P::Topojson(p) => column!(Topojson, TopojsonColumn, p),
            P::Object(p) => column!(Object, ObjectColumn, p),
            P::Unknown(p) => column!(Unknown, UnknownColumn, p),
        }
    }

    pub fn name(&self) -> &str {
        match_column!(self, column => &column.name)
    }

    pub fn required(&self) -> Option<bool> {
        match_column!(self, column => column.required)
    }

    pub fn set_required(&mut self, required: Option<bool>) {
        match_column!(self, column => column.required = required)
    }

    pub fn nullable(&self) -> Option<bool> {
        match_column!(self, column => column.nullable)
    }

    pub fn set_nullable(&mut self, nullable: Option<bool>) {
        match_column!(self, column => column.nullable = nullable)
    }

    pub fn column_type(&self) -> ColumnType {
        match self {
            Column::Array(_) => ColumnType::Array,
            Column::Base64(_) => ColumnType::Base64,
            Column::Boolean(_) => ColumnType::Boolean,
            Column::Categorical(_) => ColumnType::Categorical,
            Column::Date(_) => ColumnType::Date,
            Column::DateTime(_) => ColumnType::DateTime,
            Column::Decimal(_) => ColumnType::Decimal,
            Column::Duration(_) => ColumnType::Duration,
            Column::Email(_) => ColumnType::Email,
            Column::Geojson(_) => ColumnType::Geojson,
            Column::Hex(_) => ColumnType::Hex,
            Column::Integer(_) => ColumnType::Integer,
            Column::List(_) => ColumnType::List,
            Column::LocalDateTime(_) => ColumnType::LocalDateTime,
            Column::LocalTime(_) => ColumnType::LocalTime,
            Column::Number(_) => ColumnType::Number,
            Column::Object(_) => ColumnType::Object,
            Column::String(_) => ColumnType::String,
            Column::Time(_) => ColumnType::Time,
            Column::Topojson(_) => ColumnType::Topojson,
            Column::Unknown(_) => ColumnType::Unknown,
            Column::Url(_) => ColumnType::Url,
            Column::Wkb(_) => ColumnType::Wkb,
            Column::Wkt(_) => ColumnType::Wkt,
        }
    }

    pub fn property(&self) -> ColumnProperty {
        use ColumnProperty as P;
        match self {
            Column::Array(c) => P::Array(c.property.clone()),
            Column::Base64(c) => P::Base64(c.property.clone()),
            Column::Boolean(c) => P::Boolean(c.property.clone()),
            Column::Categorical(c) => match &c.property {
                CategoricalColumnProperty::String(p) => P::StringCategorical(p.clone()),
                CategoricalColumnProperty::Integer(p) => P::IntegerCategorical(p.clone()),
            },
            Column::Date(c) => P::Date(c.property.clone()),
            Column::DateTime(c) => P::DateTime(c.property.clone()),
            Column::Decimal(c) => P::Decimal(c.property.clone()),
            Column::Duration(c) => P::Duration(c.property.clone()),
            Column::Email(c) => P::Email(c.property.clone()),
            Column::Geojson(c) => P::Geojson(c.property.clone()),
            Column::Hex(c) => P::Hex(c.property.clone()),
            Column::Integer(c) => P::Integer(c.property.clone()),
            Column::List(c) => P::List(c.property.clone()),
            Column::LocalDateTime(c) => P::LocalDateTime(c.property.clone()),
            Column::LocalTime(c) => P::LocalTime(c.property.clone()),
            Column::Number(c) => P::Number(c.property.clone()),
            Column::Object(c) => P::Object(c.property.clone()),
            Column::String(c) => P::String(c.property.clone()),
            Column::Time(c) => P::Time(c.property.clone()),
            Column::Topojson(c) => P::Topojson(c.property.clone()),
            Column::Unknown(c) => P::Unknown(c.property.clone()),
            Column::Url(c) => P::Url(c.property.clone()),
            Column::Wkb(c) => P::Wkb(c.property.clone()),
            Column::Wkt(c) => P::Wkt(c.property.clone()),
        }
    }

    /// Replaces the property, keeping name, required and nullable; the column
    /// kind follows the property, as `from_property` decides it.
    pub fn set_property(&mut self, property: ColumnProperty) {
        let mut column = Column::from_property(self.name(), property);
        column.set_required(self.required());
        column.set_nullable(self.nullable());
        *self = column;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColumnType {
    Array,
    Base64,
    Boolean,
    Categorical,
    Date,
    DateTime,
    Decimal,
    Duration,
    Email,
    Geojson,
    Hex,
    Integer,
    List,
    LocalDateTime,
    LocalTime,
    Number,
    Object,
    String,
    Time,
    Topojson,
    Unknown,
    Url,
    Wkb,
    Wkt,
}

/// A table schema property. Deserialization picks the variant from the base
/// `type` and the `format`; a property with no `type` (or `"null"`) is unknown.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ColumnProperty {
    Boolean(BooleanColumnProperty),
    Integer(IntegerColumnProperty),
    IntegerCategorical(IntegerCategoricalColumnProperty),
    Number(NumberColumnProperty),
    List(ListColumnProperty),
    Base64(Base64ColumnProperty),
    Hex(HexColumnProperty),
    Email(EmailColumnProperty),
    Url(UrlColumnProperty),
    DateTime(DateTimeColumnProperty),
    LocalDateTime(LocalDateTimeColumnProperty),
    Date(DateColumnProperty),
    Time(TimeColumnProperty),
    LocalTime(LocalTimeColumnProperty),
    Duration(DurationColumnProperty),
    Wkt(WktColumnProperty),
    Wkb(WkbColumnProperty),
    StringCategorical(StringCategoricalColumnProperty),
    Decimal(DecimalColumnProperty),
    String(StringColumnProperty),
    Array(ArrayColumnProperty),
    Geojson(GeojsonColumnProperty),
    Topojson(TopojsonColumnProperty),
    Object(ObjectColumnProperty),
    Unknown(UnknownColumnProperty),
}

impl ColumnProperty {
    pub fn base_type(&self) -> Option<BasePropertyType> {
        use ColumnProperty as P;
        match self {
            P::Boolean(_) => Some(BasePropertyType::Boolean),
            P::Integer(_) | P::IntegerCategorical(_) => Some(BasePropertyType::Integer),
            P::Number(_) => Some(BasePropertyType::Number),
            P::Array(_) => Some(BasePropertyType::Array),
            P::Object(_) | P::Geojson(_) | P::Topojson(_) => Some(BasePropertyType::Object),
            P::Unknown(_) => None,
            _ => Some(BasePropertyType::String),
        }
    }

    pub fn is_nullable(&self) -> bool {
        self.null_position()
            .is_some_and(|null| null != NullPosition::None)
    }

    /// Widens a non-nullable `type` to `[type, "null"]`; unknown properties stay as they are.
    pub fn set_nullable(&mut self) {
        if self.null_position() == Some(NullPosition::None) {
            self.set_null_position(NullPosition::After);
        }
    }

    pub fn null_position(&self) -> Option<NullPosition> {
        use ColumnProperty as P;
        let null = match self {
            P::Boolean(p) => p.r#type.null,
            P::Integer(p) => p.base.r#type.null,
            P::IntegerCategorical(p) => p.base.r#type.null,
            P::Number(p) => p.base.r#type.null,
            P::List(p) => p.base.r#type.null,
            P::Base64(p) => p.base.r#type.null,
            P::Hex(p) => p.base.r#type.null,
            P::Email(p) => p.base.r#type.null,
            P::Url(p) => p.base.r#type.null,
            P::DateTime(p) => p.base.r#type.null,
            P::LocalDateTime(p) => p.base.r#type.null,
            P::Date(p) => p.base.r#type.null,
            P::Time(p) => p.base.r#type.null,
            P::LocalTime(p) => p.base.r#type.null,
            P::Duration(p) => p.base.r#type.null,
            P::Wkt(p) => p.base.r#type.null,
            P::Wkb(p) => p.base.r#type.null,
            P::StringCategorical(p) => p.base.r#type.null,
            P::Decimal(p) => p.base.r#type.null,
            P::String(p) => p.base.r#type.null,
            P::Array(p) => p.r#type.null,
            P::Geojson(p) => p.base.r#type.null,
            P::Topojson(p) => p.base.r#type.null,
            P::Object(p) => p.base.r#type.null,
            P::Unknown(_) => return None,
        };
        Some(null)
    }

    pub fn set_null_position(&mut self, null: NullPosition) {
        use ColumnProperty as P;
        match self {
            P::Boolean(p) => p.r#type.null = null,
            P::Integer(p) => p.base.r#type.null = null,
            P::IntegerCategorical(p) => p.base.r#type.null = null,
            P::Number(p) => p.base.r#type.null = null,
            P::List(p) => p.base.r#type.null = null,
            P::Base64(p) => p.base.r#type.null = null,
            P::Hex(p) => p.base.r#type.null = null,
            P::Email(p) => p.base.r#type.null = null,
            P::Url(p) => p.base.r#type.null = null,
            P::DateTime(p) => p.base.r#type.null = null,
            P::LocalDateTime(p) => p.base.r#type.null = null,
            P::Date(p) => p.base.r#type.null = null,
            P::Time(p) => p.base.r#type.null = null,
            P::LocalTime(p) => p.base.r#type.null = null,
            P::Duration(p) => p.base.r#type.null = null,
            P::Wkt(p) => p.base.r#type.null = null,
            P::Wkb(p) => p.base.r#type.null = null,
            P::StringCategorical(p) => p.base.r#type.null = null,
            P::Decimal(p) => p.base.r#type.null = null,
            P::String(p) => p.base.r#type.null = null,
            P::Array(p) => p.r#type.null = null,
            P::Geojson(p) => p.base.r#type.null = null,
            P::Topojson(p) => p.base.r#type.null = null,
            P::Object(p) => p.base.r#type.null = null,
            P::Unknown(_) => {}
        }
    }
}

impl<'de> Deserialize<'de> for ColumnProperty {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use ColumnProperty as P;
        let value = Value::deserialize(deserializer)?;
        let Value::Object(object) = &value else {
            return Err(D::Error::custom("expected a column property object"));
        };

        let base_type = match object.get("type") {
            None | Some(Value::Null) => return parse_variant(value, P::Unknown),
            Some(Value::String(name)) if name == "null" => {
                return parse_variant(value, P::Unknown);
            }
            Some(Value::String(name)) => name.as_str(),
            Some(Value::Array(_)) => match get_base_property_type(object.get("type")) {
                Some(name) => name,
                None => {
                    return Err(D::Error::custom(format!(
                        "unsupported column property type {}",
                        object["type"]
                    )));
                }
            },
            Some(other) => {
                return Err(D::Error::custom(format!(
                    "invalid column property type {other}"
                )));
            }
        };
        let format = match object.get("format") {
            None | Some(Value::Null) => None,
            Some(Value::String(format)) => Some(format.as_str()),
            Some(other) => {
                return Err(D::Error::custom(format!(
                    "invalid column property format {other}"
                )));
            }
        };

        match (base_type, format) {
            ("boolean", None) => parse_variant(value, P::Boolean),
            ("integer", None) => parse_variant(value, P::Integer),
            ("integer", Some("categorical")) => parse_variant(value, P::IntegerCategorical),
            ("number", None) => parse_variant(value, P::Number),
            ("string", None) => parse_variant(value, P::String),
            ("string", Some("list")) => parse_variant(value, P::List),
            ("string", Some("base64")) => parse_variant(value, P::Base64),
            ("string", Some("hex")) => parse_variant(value, P::Hex),
            ("string", Some("email")) => parse_variant(value, P::Email),
            ("string", Some("url")) => parse_variant(value, P::Url),
            ("string", Some("date-time")) => parse_variant(value, P::DateTime),
            ("string", Some("local-date-time")) => parse_variant(value, P::LocalDateTime),
            ("string", Some("date")) => parse_variant(value, P::Date),
            ("string", Some("time")) => parse_variant(value, P::Time),
            ("string", Some("local-time")) => parse_variant(value, P::LocalTime),
            ("string", Some("duration")) => parse_variant(value, P::Duration),
            ("string", Some("wkt")) => parse_variant(value, P::Wkt),
            ("string", Some("wkb")) => parse_variant(value, P::Wkb),
            ("string", Some("categorical")) => parse_variant(value, P::StringCategorical),
            ("string", Some("decimal")) => parse_variant(value, P::Decimal),
            ("array", None) => parse_variant(value, P::Array),
            ("object", None) => parse_variant(value, P::Object),
            ("object", Some("geojson")) => parse_variant(value, P::Geojson),
            ("object", Some("topojson")) => parse_variant(value, P::Topojson),
            ("boolean", Some(_)) => parse_variant(value, P::Boolean),
            ("integer", Some(_)) => parse_variant(value, P::Integer),
            ("number", Some(_)) => parse_variant(value, P::Number),
            ("string", Some(_)) => parse_variant(value, P::String),
            ("array", Some(_)) => parse_variant(value, P::Array),
            ("object", Some(_)) => parse_variant(value, P::Object),
            (base_type, format) => Err(D::Error::custom(format!(
                "unsupported column property type \"{base_type}\" with format {format:?}"
            ))),
        }
    }
}

fn parse_variant<T: DeserializeOwned, E: serde::de::Error>(
    value: Value,
    variant: fn(T) -> ColumnProperty,
) -> Result<ColumnProperty, E> {
    serde_json::from_value(value)
        .map(variant)
        .map_err(E::custom)
}
