/// The shape of a Frictionless descriptor, checked by `assert_frictionless_model`.
/// Objects are loose: unknown keys are kept, as in the other twins' models.
#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Unknown,
    String,
    Boolean,
    Integer { minimum: Option<i64> },
    Enum(&'static [&'static str]),
    Array(&'static Shape),
    Record,
    Object(&'static [ShapeProperty]),
    Union(&'static [Shape]),
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeProperty {
    pub name: &'static str,
    pub shape: Shape,
    pub required: bool,
}

pub(crate) const fn optional(name: &'static str, shape: Shape) -> ShapeProperty {
    ShapeProperty {
        name,
        shape,
        required: false,
    }
}

pub(crate) const fn required(name: &'static str, shape: Shape) -> ShapeProperty {
    ShapeProperty {
        name,
        shape,
        required: true,
    }
}

pub(crate) const INTEGER: Shape = Shape::Integer { minimum: None };
pub(crate) const STRING_LIST: Shape = Shape::Array(&Shape::String);
