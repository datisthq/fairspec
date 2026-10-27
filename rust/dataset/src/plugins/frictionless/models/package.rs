use super::license::FRICTIONLESS_LICENSE;
use super::resource::FRICTIONLESS_RESOURCE;
use super::shape::{STRING_LIST, Shape, optional, required};

pub const FRICTIONLESS_CONTRIBUTOR: Shape = Shape::Object(&[
    optional("title", Shape::String),
    optional("givenName", Shape::String),
    optional("familyName", Shape::String),
    optional("path", Shape::String),
    optional("email", Shape::String),
    optional("organization", Shape::String),
    optional("role", Shape::String),
    optional("roles", STRING_LIST),
]);

pub const FRICTIONLESS_SOURCE: Shape = Shape::Object(&[
    optional("title", Shape::String),
    optional("path", Shape::String),
    optional("email", Shape::String),
]);

pub const FRICTIONLESS_PACKAGE: Shape = Shape::Object(&[
    optional("$schema", Shape::String),
    optional("profile", Shape::String),
    optional("name", Shape::String),
    optional("id", Shape::String),
    optional("title", Shape::String),
    optional("description", Shape::String),
    optional("homepage", Shape::String),
    optional("version", Shape::String),
    optional("created", Shape::String),
    optional("keywords", STRING_LIST),
    optional("licenses", Shape::Array(&FRICTIONLESS_LICENSE)),
    optional("contributors", Shape::Array(&FRICTIONLESS_CONTRIBUTOR)),
    optional("sources", Shape::Array(&FRICTIONLESS_SOURCE)),
    required("resources", Shape::Array(&FRICTIONLESS_RESOURCE)),
]);
