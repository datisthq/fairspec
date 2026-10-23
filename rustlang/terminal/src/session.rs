use fairspec_library::fairspec_metadata::{Error, Report};
use fairspec_library::fairspec_table::Frame;
use serde_json::{Value, json};

use crate::actions::error::render::render_error;
use crate::helpers::console::Console;
use crate::helpers::frame::get_frame_records;
use crate::helpers::json::dump_json;
use crate::helpers::markup::{render_markup, strip_markup};
use crate::models::output::{CommandOutput, Exit};

/// Python's `Session`: human output with status lines, or nothing but the final result
/// under `--json`; `silent` drops all human output, `debug` re-raises a failing task.
pub struct Session<'a> {
    pub silent: bool,
    pub debug: bool,
    pub json: bool,
    console: &'a Console,
}

impl<'a> Session<'a> {
    pub fn new(console: &'a Console, silent: bool, debug: bool, json: bool) -> Self {
        let session = Session {
            silent,
            debug,
            json,
            console,
        };
        if session.get_is_human() {
            console.write_out("\n");
        }
        session
    }

    pub fn render_text(&self, text: &str, status: Option<&str>) {
        if !self.get_is_human() {
            return;
        }
        let line = match status.map(get_status_symbol) {
            Some(symbol) if !symbol.is_empty() => format!("{symbol} {text}"),
            _ => text.to_string(),
        };
        self.console.write_out(&format!(
            "{}\n",
            render_markup(&line, self.console.is_terminal)
        ));
    }

    pub fn render_text_result(&self, text: &str, status: Option<&str>) -> CommandOutput {
        self.render_text(text, status);
        CommandOutput::new(json!({"result": strip_markup(text)}))
    }

    /// Writes file content as is: console markup would swallow `[...]` in it.
    pub fn render_raw_result(&self, text: &str) {
        if !self.get_is_human() {
            return;
        }
        if text.ends_with('\n') {
            self.console.write_out(text);
        } else {
            self.console.write_out(&format!("{text}\n"));
        }
    }

    pub fn render_data_result(&self, data: Value) -> CommandOutput {
        if self.get_is_human() {
            self.console.write_out(&format!("{}\n", dump_json(&data)));
        }
        CommandOutput::new(data)
    }

    pub fn render_frame_result(&self, frame: &Frame) -> Result<CommandOutput, Exit> {
        if self.json {
            let records = get_frame_records(frame).map_err(|error| self.fail(None, error))?;
            return Ok(CommandOutput::new(Value::Array(records)));
        }
        if !self.silent {
            self.console.write_out(&format!("{frame}\n"));
        }
        Ok(CommandOutput::new(Value::Array(Vec::new())))
    }

    pub fn render_report_result(&self, report: &Report) -> CommandOutput {
        let errors = serde_json::to_value(&report.errors).unwrap_or(Value::Array(Vec::new()));
        let code = if report.valid { 0 } else { 1 };
        if self.get_is_human() {
            if report.valid {
                self.render_text("Validation passed", Some("success"));
            } else {
                for error in &report.errors {
                    self.render_text(&render_error(error), Some("error"));
                }
            }
        }
        CommandOutput {
            data: json!({"valid": report.valid, "errors": errors}),
            code,
        }
    }

    /// Runs one step of a command; a failure ends the command with exit code 1.
    pub fn task<T>(&self, title: &str, func: impl FnOnce() -> Result<T, Error>) -> Result<T, Exit> {
        func().map_err(|error| self.fail(Some(title), error))
    }

    fn fail(&self, title: Option<&str>, error: Error) -> Exit {
        if self.debug {
            self.console.write_err(&format!("{error:?}\n"));
            return Exit(1);
        }
        if self.json {
            self.console.write_out(&format!(
                "{}\n",
                dump_json(&json!({"error": error.to_string()}))
            ));
            return Exit(1);
        }
        if self.silent {
            return Exit(1);
        }
        if let Error::Invalid {
            report: Some(report),
            ..
        } = &error
        {
            self.console.write_out("\n");
            self.render_report_result(report);
            return Exit(1);
        }
        let title = title.map(|title| format!("{title}: ")).unwrap_or_default();
        let line = format!("[red]✖[/red] {title}{error}");
        self.console.write_out(&format!(
            "{}\n",
            render_markup(&line, self.console.is_terminal)
        ));
        Exit(1)
    }

    fn get_is_human(&self) -> bool {
        !self.silent && !self.json
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        if self.get_is_human() {
            self.console.write_out("\n");
        }
    }
}

fn get_status_symbol(status: &str) -> &'static str {
    match status {
        "success" => "[green]✔[/green]",
        "warning" => "[yellow]⚠[/yellow]",
        "error" => "[red]✖[/red]",
        _ => "",
    }
}
