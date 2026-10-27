use super::field::{FRICTIONLESS_FIELD, FRICTIONLESS_MISSING_VALUE};
use super::shape::{STRING_LIST, Shape, optional, required};

const FIELD_NAMES: Shape = Shape::Union(&[Shape::String, STRING_LIST]);

pub const FRICTIONLESS_FOREIGN_KEY: Shape = Shape::Object(&[
    required("fields", FIELD_NAMES),
    required(
        "reference",
        Shape::Object(&[
            optional("resource", Shape::String),
            required("fields", FIELD_NAMES),
        ]),
    ),
]);

pub const FRICTIONLESS_SCHEMA: Shape = Shape::Object(&[
    required("fields", Shape::Array(&FRICTIONLESS_FIELD)),
    optional("primaryKey", FIELD_NAMES),
    optional("uniqueKeys", Shape::Array(&STRING_LIST)),
    optional("foreignKeys", Shape::Array(&FRICTIONLESS_FOREIGN_KEY)),
    optional("missingValues", Shape::Array(&FRICTIONLESS_MISSING_VALUE)),
]);
