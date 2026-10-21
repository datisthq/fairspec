use super::assert::assert_file_dialect;
use crate::actions::descriptor::load::load_descriptor;
use crate::models::exception::Result;
use crate::models::file_dialect::file_dialect::FileDialect;

pub fn load_file_dialect(path: &str) -> Result<FileDialect> {
    let descriptor = load_descriptor(path, false)?;
    assert_file_dialect(&descriptor)
}
