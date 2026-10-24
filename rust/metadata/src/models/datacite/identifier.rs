pub type Doi = String;
pub type DoiPrefix = String;
pub type DoiSuffix = String;

pub const DOI_PATTERN: &str = r"^10[.][0-9]{4,9}[/][^\s]+$";
pub const DOI_PREFIX_PATTERN: &str = r"^10[.][0-9]{4,9}$";
pub const DOI_SUFFIX_PATTERN: &str = r"^[^\s]+$";
