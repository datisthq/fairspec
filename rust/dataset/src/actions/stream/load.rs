use std::fs::File;
use std::io::{Cursor, Read};

use fairspec_metadata::{Error, ResourceDataPath, Result, get_is_remote_path};

use crate::helpers::http::load_remote_stream;

pub type FileStream = Box<dyn Read + Send>;

pub fn load_file_stream(
    data_path: impl Into<ResourceDataPath>,
    index: Option<usize>,
    max_bytes: Option<usize>,
) -> Result<FileStream> {
    let paths = match data_path.into() {
        ResourceDataPath::Path(path) => vec![path],
        ResourceDataPath::Paths(paths) => paths,
    };
    let index = index.unwrap_or(0);
    let Some(path) = paths.get(index) else {
        return Err(Error::Load(format!(
            "Cannot stream resource None at index {index}"
        )));
    };

    if get_is_remote_path(path) {
        return load_remote_file_stream(path, max_bytes);
    }
    load_local_file_stream(path, max_bytes)
}

fn load_local_file_stream(path: &str, max_bytes: Option<usize>) -> Result<FileStream> {
    let file = File::open(path)?;
    let Some(max_bytes) = max_bytes else {
        return Ok(Box::new(file));
    };
    let mut data = Vec::new();
    file.take(max_bytes as u64).read_to_end(&mut data)?;
    Ok(Box::new(Cursor::new(data)))
}

// Python reads the whole response before returning, so a failed transfer fails here
fn load_remote_file_stream(path: &str, max_bytes: Option<usize>) -> Result<FileStream> {
    let mut data = Vec::new();
    load_remote_stream(path, max_bytes)?.read_to_end(&mut data)?;
    Ok(Box::new(Cursor::new(data)))
}
