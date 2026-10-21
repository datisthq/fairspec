use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(DurationFormat, Duration, "duration");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DurationColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: DurationFormat,
}

define_column!(DurationColumn, DurationColumnProperty);
