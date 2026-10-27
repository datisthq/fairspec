use super::dialect::FRICTIONLESS_DIALECT;
use super::license::FRICTIONLESS_LICENSE;
use super::schema::FRICTIONLESS_SCHEMA;
use super::shape::{INTEGER, STRING_LIST, Shape, optional};

pub const FRICTIONLESS_RESOURCE: Shape = Shape::Object(&[
    optional("name", Shape::String),
    optional("path", Shape::Union(&[Shape::String, STRING_LIST])),
    optional("data", Shape::Unknown),
    optional("title", Shape::String),
    optional("description", Shape::String),
    optional("format", Shape::String),
    optional("mediatype", Shape::String),
    optional("encoding", Shape::String),
    optional("bytes", INTEGER),
    optional("hash", Shape::String),
    optional("licenses", Shape::Array(&FRICTIONLESS_LICENSE)),
    optional(
        "dialect",
        Shape::Union(&[Shape::String, FRICTIONLESS_DIALECT]),
    ),
    optional(
        "schema",
        Shape::Union(&[Shape::String, FRICTIONLESS_SCHEMA]),
    ),
]);
