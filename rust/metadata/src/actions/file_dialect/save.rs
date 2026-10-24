use crate::actions::descriptor::save::save_profiled_model;
use crate::models::exception::Result;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::profile::ProfileType;

pub fn save_file_dialect(file_dialect: &FileDialect, path: &str, overwrite: bool) -> Result<()> {
    save_profiled_model(file_dialect, ProfileType::FileDialect, path, overwrite)
}
