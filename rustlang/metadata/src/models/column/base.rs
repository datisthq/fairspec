use std::marker::PhantomData;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BasePropertyType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

impl BasePropertyType {
    pub fn as_str(self) -> &'static str {
        match self {
            BasePropertyType::String => "string",
            BasePropertyType::Number => "number",
            BasePropertyType::Integer => "integer",
            BasePropertyType::Boolean => "boolean",
            BasePropertyType::Array => "array",
            BasePropertyType::Object => "object",
        }
    }
}

/// Fixes the base type a [`NullablePropertyType`] accepts.
pub trait PropertyBase {
    const TYPE: BasePropertyType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StringBase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NumberBase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IntegerBase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BooleanBase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ArrayBase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ObjectBase;

impl PropertyBase for StringBase {
    const TYPE: BasePropertyType = BasePropertyType::String;
}
impl PropertyBase for NumberBase {
    const TYPE: BasePropertyType = BasePropertyType::Number;
}
impl PropertyBase for IntegerBase {
    const TYPE: BasePropertyType = BasePropertyType::Integer;
}
impl PropertyBase for BooleanBase {
    const TYPE: BasePropertyType = BasePropertyType::Boolean;
}
impl PropertyBase for ArrayBase {
    const TYPE: BasePropertyType = BasePropertyType::Array;
}
impl PropertyBase for ObjectBase {
    const TYPE: BasePropertyType = BasePropertyType::Object;
}

/// Where `"null"` sits in a nullable `type`: `"integer"`, `["integer", "null"]`
/// or `["null", "integer"]`. Kept so the descriptor round-trips exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NullPosition {
    #[default]
    None,
    After,
    Before,
}

/// A property `type` of one base type, optionally paired with `"null"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct NullablePropertyType<B> {
    pub null: NullPosition,
    base: PhantomData<B>,
}

impl<B: PropertyBase> NullablePropertyType<B> {
    pub fn new(null: NullPosition) -> Self {
        Self {
            null,
            base: PhantomData,
        }
    }

    pub fn base_type(&self) -> BasePropertyType {
        B::TYPE
    }

    pub fn is_nullable(&self) -> bool {
        self.null != NullPosition::None
    }
}

impl<B: PropertyBase> Serialize for NullablePropertyType<B> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let base = B::TYPE.as_str();
        match self.null {
            NullPosition::None => serializer.serialize_str(base),
            NullPosition::After => [base, "null"].serialize(serializer),
            NullPosition::Before => ["null", base].serialize(serializer),
        }
    }
}

impl<'de, B: PropertyBase> Deserialize<'de> for NullablePropertyType<B> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let base = B::TYPE.as_str();
        let value = Value::deserialize(deserializer)?;
        let null = match &value {
            Value::String(name) if name == base => Some(NullPosition::None),
            Value::Array(items) if items.len() == 2 => {
                match (items[0].as_str(), items[1].as_str()) {
                    (Some(first), Some("null")) if first == base => Some(NullPosition::After),
                    (Some("null"), Some(second)) if second == base => Some(NullPosition::Before),
                    _ => None,
                }
            }
            _ => None,
        };
        null.map(Self::new).ok_or_else(|| {
            D::Error::custom(format!("expected property type \"{base}\", got {value}"))
        })
    }
}

pub type StringNullablePropertyType = NullablePropertyType<StringBase>;
pub type NumberNullablePropertyType = NullablePropertyType<NumberBase>;
pub type IntegerNullablePropertyType = NullablePropertyType<IntegerBase>;
pub type BooleanNullablePropertyType = NullablePropertyType<BooleanBase>;
pub type ArrayNullablePropertyType = NullablePropertyType<ArrayBase>;
pub type ObjectNullablePropertyType = NullablePropertyType<ObjectBase>;

/// Python's `V | Item` in `missingValues` and `categories` lists.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueOrItem<V, I> {
    Value(V),
    Item(I),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrInteger {
    String(String),
    Integer(i64),
}
