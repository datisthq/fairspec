use std::path::Path;

use fairspec_metadata::Result;

use crate::actions::stream::save::create_exists_error;

pub fn get_is_local_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn assert_local_path_vacant(path: &str) -> Result<()> {
    if Path::new(path).exists() {
        return Err(create_exists_error(path));
    }
    Ok(())
}
