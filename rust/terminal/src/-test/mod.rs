mod program_unit;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::helpers::console::Console;
use crate::program::run_program;

/// The exit code, stdout and stderr of `fairspec <args>` run in process.
pub struct RunResult {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl RunResult {
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.stdout)
            .unwrap_or_else(|error| panic!("{error}: {}", self.stdout))
    }
}

pub fn run(args: &[&str]) -> RunResult {
    let stdout = SharedBuffer::default();
    let stderr = SharedBuffer::default();
    let console = Console::from_writers(Box::new(stdout.clone()), Box::new(stderr.clone()));
    let code = run_program(
        std::iter::once("fairspec").chain(args.iter().copied()),
        &console,
    );
    RunResult {
        code,
        stdout: stdout.get_text(),
        stderr: stderr.get_text(),
    }
}

pub fn write_file(folder: &std::path::Path, name: &str, content: &str) -> String {
    let path = folder.join(name);
    std::fs::write(&path, content).unwrap();
    path.to_string_lossy().replace('\\', "/")
}

#[derive(Clone, Default)]
struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

impl SharedBuffer {
    fn get_text(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().unwrap()).into_owned()
    }
}

impl Write for SharedBuffer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
