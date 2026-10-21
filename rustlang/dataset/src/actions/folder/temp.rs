use fairspec_metadata::Result;

use crate::actions::file::temp::format_path;
use crate::helpers::cleanup::register_cleanup;

pub fn get_temp_folder_path(persist: bool) -> Result<String> {
    let path = tempfile::Builder::new().prefix("tmp").tempdir()?.keep();
    if !persist {
        register_cleanup(path.clone());
    }
    Ok(format_path(&path))
}
