use fairspec_library::fairspec_metadata::{Error, Report};
use fairspec_library::fairspec_table::Frame;
use serde::Serialize;
use serde_json::json;

use crate::actions::error::render::render_error;
use crate::helpers::console::Console;
use crate::helpers::frame::get_frame_records;
use crate::helpers::json::dump_json;
use crate::helpers::markup::{render_markup, strip_markup};
use crate::models::output::{Exit, Records, ReportResult, TextResult};

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

    pub fn render_text_result(&self, text: &str, status: Option<&str>) -> TextResult {
        self.render_text(text, status);
        TextResult {
            result: strip_markup(text),
        }
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

    pub fn render_data_result<T: Serialize>(&self, data: T) -> T {
        if self.get_is_human() {
            let value = serde_json::to_value(&data).unwrap_or_default();
            self.console.write_out(&format!("{}\n", dump_json(&value)));
        }
        data
    }

    pub fn render_frame_result(&self, frame: &Frame) -> Result<Records, Exit> {
        if self.json {
            return get_frame_records(frame).map_err(|error| self.fail(None, error));
        }
        if !self.silent {
            self.console.write_out(&format!("{frame}\n"));
        }
        Ok(Vec::new())
    }

    /// The report as the result; an invalid one ends the command with exit code 1.
    pub fn render_report_result(&self, report: &Report) -> Result<ReportResult, Exit> {
        self.print_report(report);
        let errors = report
            .errors
            .iter()
            .map(|error| serde_json::to_value(error).unwrap_or_default())
            .collect();
        let result = ReportResult {
            valid: report.valid,
            errors,
        };
        if report.valid {
            return Ok(result);
        }
        Err(Exit {
            data: self
                .json
                .then(|| serde_json::to_value(&result).unwrap_or_default()),
        })
    }

    /// Runs one step of a command; a failure ends the command with exit code 1.
    pub fn task<T>(&self, title: &str, func: impl FnOnce() -> Result<T, Error>) -> Result<T, Exit> {
        func().map_err(|error| self.fail(Some(title), error))
    }

    fn fail(&self, title: Option<&str>, error: Error) -> Exit {
        if self.debug {
            self.console.write_err(&format!("{error:?}\n"));
            return Exit::default();
        }
        if self.json {
            return Exit {
                data: Some(json!({"error": error.to_string()})),
            };
        }
        if self.silent {
            return Exit::default();
        }
        if let Error::Invalid {
            report: Some(report),
            ..
        } = &error
        {
            self.console.write_out("\n");
            self.print_report(report);
            return Exit::default();
        }
        let title = title.map(|title| format!("{title}: ")).unwrap_or_default();
        let line = format!("[red]✖[/red] {title}{error}");
        self.console.write_out(&format!(
            "{}\n",
            render_markup(&line, self.console.is_terminal)
        ));
        Exit::default()
    }

    fn print_report(&self, report: &Report) {
        if !self.get_is_human() {
            return;
        }
        if report.valid {
            self.render_text("Validation passed", Some("success"));
            return;
        }
        for error in &report.errors {
            self.render_text(&render_error(error), Some("error"));
        }
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
