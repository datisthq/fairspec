use std::fs;

use fairspec_metadata::Result;

pub fn create_folder(path: &str) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}
