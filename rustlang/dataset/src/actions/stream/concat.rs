use std::io::{Cursor, Read};

use fairspec_metadata::Result;

use super::load::FileStream;

pub fn concat_file_streams(streams: Vec<FileStream>) -> Result<FileStream> {
    let mut data = Vec::new();
    for mut stream in streams {
        stream.read_to_end(&mut data)?;
    }
    Ok(Box::new(Cursor::new(data)))
}
