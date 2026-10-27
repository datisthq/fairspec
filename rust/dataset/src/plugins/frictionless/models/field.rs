use super::shape::{INTEGER, STRING_LIST, Shape, optional, required};

pub const FRICTIONLESS_CONSTRAINTS: Shape = Shape::Object(&[
    optional("required", Shape::Boolean),
    optional("unique", Shape::Boolean),
    optional("minimum", Shape::Unknown),
    optional("maximum", Shape::Unknown),
    optional("exclusiveMinimum", Shape::Unknown),
    optional("exclusiveMaximum", Shape::Unknown),
    optional("minLength", INTEGER),
    optional("maxLength", INTEGER),
    optional("pattern", Shape::String),
    optional("enum", Shape::Array(&Shape::Unknown)),
    optional("jsonSchema", Shape::Record),
]);

pub const FRICTIONLESS_MISSING_VALUE: Shape = Shape::Union(&[
    Shape::String,
    Shape::Object(&[
        required("value", Shape::String),
        required("label", Shape::String),
    ]),
]);

pub const FRICTIONLESS_FIELD: Shape = Shape::Object(&[
    required("name", Shape::String),
    optional("type", Shape::String),
    optional("format", Shape::String),
    optional("title", Shape::String),
    optional("description", Shape::String),
    optional("example", Shape::Unknown),
    optional("rdfType", Shape::String),
    optional("constraints", FRICTIONLESS_CONSTRAINTS),
    optional("missingValues", Shape::Array(&FRICTIONLESS_MISSING_VALUE)),
    optional("trueValues", STRING_LIST),
    optional("falseValues", STRING_LIST),
    optional("decimalChar", Shape::String),
    optional("groupChar", Shape::String),
    optional("bareNumber", Shape::Boolean),
    optional("categories", Shape::Array(&Shape::Unknown)),
    optional("categoriesOrdered", Shape::Boolean),
    optional("delimiter", Shape::String),
    optional("itemType", Shape::String),
]);
