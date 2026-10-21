use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

use fairspec_metadata::Result;

pub fn save_file_stream(mut stream: impl Read, path: &str, overwrite: bool) -> Result<()> {
    if let Some(parent) = Path::new(path)
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    if !overwrite && Path::new(path).exists() {
        return Err(create_exists_error(path));
    }

    let mut file = File::create(path)?;
    io::copy(&mut stream, &mut file)?;
    Ok(())
}

pub(crate) fn create_exists_error(path: &str) -> fairspec_metadata::Error {
    io::Error::new(
        io::ErrorKind::AlreadyExists,
        format!("Path \"{path}\" already exists"),
    )
    .into()
}
