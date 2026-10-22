use fairspec_dataset::get_is_local_path_exist;
use fairspec_metadata::{Error, Result};
use rusqlite::Connection;

pub fn connect_database(path: &str, create: bool) -> Result<Connection> {
    let path = path.strip_prefix("sqlite://").unwrap_or(path);

    if path == ":memory:" || path.starts_with("file::memory") {
        return Err(Error::Load(
            "In-memory databases are not supported".to_string(),
        ));
    }

    if !create && !get_is_local_path_exist(path) {
        return Err(Error::Load(format!(
            "Database file \"{path}\" does not exist"
        )));
    }

    Connection::open(path).map_err(|error| Error::Load(error.to_string()))
}
