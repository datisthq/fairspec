use std::io::Read;

use fairspec_metadata::Result;

use crate::actions::stream::load::load_file_stream;

pub fn load_file(path: &str, max_bytes: Option<usize>) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    load_file_stream(path, None, max_bytes)?.read_to_end(&mut data)?;
    Ok(data)
}
