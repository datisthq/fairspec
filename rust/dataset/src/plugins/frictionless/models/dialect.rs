use super::shape::{INTEGER, STRING_LIST, Shape, optional};

const ROW_NUMBERS: Shape = Shape::Array(&Shape::Integer { minimum: Some(1) });

pub const FRICTIONLESS_DIALECT: Shape = Shape::Object(&[
    optional("header", Shape::Boolean),
    optional("headerRows", ROW_NUMBERS),
    optional("headerJoin", Shape::String),
    optional("commentRows", ROW_NUMBERS),
    optional("commentChar", Shape::String),
    optional("delimiter", Shape::String),
    optional("lineTerminator", Shape::String),
    optional("quoteChar", Shape::String),
    optional("nullSequence", Shape::String),
    optional("property", Shape::String),
    optional("itemType", Shape::Enum(&["array", "object"])),
    optional("itemKeys", STRING_LIST),
    optional("sheetNumber", INTEGER),
    optional("sheetName", Shape::String),
    optional("table", Shape::String),
]);
