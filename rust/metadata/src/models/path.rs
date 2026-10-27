use serde::{Deserialize, Serialize};

pub type Path = String;
pub type InternalPath = String;
pub type ExternalPath = String;

pub const INTERNAL_PATH_PATTERN: &str = r"^[^./~\\:][^./\\:]*(\.[^./\\:]+)*\.?(/([^./\\:]+(\.[^./\\:]+)*\.?|\.([^./\\:]+(\.[^./\\:]+)*\.?)?))*$";
pub const EXTERNAL_PATH_PATTERN: &str = r"^[Hh][Tt][Tt][Pp][Ss]?://";

/// A value given either inline or as a path to a JSON file holding it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PathOr<T> {
    Path(Path),
    Value(T),
}
