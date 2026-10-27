use super::shape::{Shape, optional};

pub const FRICTIONLESS_LICENSE: Shape = Shape::Object(&[
    optional("name", Shape::String),
    optional("path", Shape::String),
    optional("title", Shape::String),
]);
