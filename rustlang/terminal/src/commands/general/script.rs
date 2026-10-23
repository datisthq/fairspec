use std::io::{self, BufRead};

use fairspec_library::fairspec_metadata::Result;
use fairspec_library::fairspec_table::Table;

use crate::helpers::console::Console;

/// The `script` commands' session: Python opens an interactive interpreter, which a
/// binary cannot embed, so each line runs as a SQL query and the result frame prints.
pub fn run_sql_session(console: &Console, run_query: impl Fn(&str) -> Result<Table>) {
    let stdin = io::stdin();
    loop {
        console.write_out("> ");
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            console.write_out("\n");
            break;
        }
        let query = line.trim();
        if query.is_empty() {
            break;
        }
        match run_query(query).and_then(|table| {
            table.collect().map_err(|error| {
                fairspec_library::fairspec_metadata::Error::Load(error.to_string())
            })
        }) {
            Ok(frame) => console.write_out(&format!("{frame}\n")),
            Err(error) => console.write_out(&format!("{error}\n")),
        }
    }
}
