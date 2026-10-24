use std::io::{self, Write};

use serde::Serialize;
use serde_json::Value;
use serde_json::ser::Formatter;

use crate::helpers::general::format_python_float;

/// Python's `json.dumps(..., ensure_ascii=False)`: `", "` and `": "` separators on one line,
/// a two-space indent for a document, and floats as Python writes them.
pub fn encode_json_buffer(data: &Value, is_lines: bool) -> Vec<u8> {
    match (is_lines, data) {
        (true, Value::Array(items)) => items
            .iter()
            .map(|item| dump_python_json(item, None))
            .collect::<Vec<_>>()
            .join(b"\n".as_slice()),
        _ => dump_python_json(data, Some(2)),
    }
}

fn dump_python_json(value: &Value, indent: Option<usize>) -> Vec<u8> {
    let mut output = Vec::new();
    let formatter = PythonFormatter {
        indent,
        depth: 0,
        has_value: false,
    };
    let mut serializer = serde_json::Serializer::with_formatter(&mut output, formatter);
    let _ = value.serialize(&mut serializer);
    output
}

struct PythonFormatter {
    indent: Option<usize>,
    depth: usize,
    has_value: bool,
}

impl PythonFormatter {
    fn write_newline<W: ?Sized + Write>(&self, writer: &mut W) -> io::Result<()> {
        if let Some(indent) = self.indent {
            writer.write_all(b"\n")?;
            writer.write_all(" ".repeat(indent * self.depth).as_bytes())?;
        }
        Ok(())
    }

    fn write_separator<W: ?Sized + Write>(&self, writer: &mut W, first: bool) -> io::Result<()> {
        if !first {
            writer.write_all(if self.indent.is_some() { b"," } else { b", " })?;
        }
        self.write_newline(writer)
    }

    fn begin_nested<W: ?Sized + Write>(&mut self, writer: &mut W, open: &[u8]) -> io::Result<()> {
        self.depth += 1;
        self.has_value = false;
        writer.write_all(open)
    }

    fn end_nested<W: ?Sized + Write>(&mut self, writer: &mut W, close: &[u8]) -> io::Result<()> {
        self.depth -= 1;
        if self.has_value {
            self.write_newline(writer)?;
        }
        self.has_value = true;
        writer.write_all(close)
    }
}

impl Formatter for PythonFormatter {
    fn write_f64<W: ?Sized + Write>(&mut self, writer: &mut W, value: f64) -> io::Result<()> {
        writer.write_all(format_python_float(value).as_bytes())
    }

    fn write_f32<W: ?Sized + Write>(&mut self, writer: &mut W, value: f32) -> io::Result<()> {
        self.write_f64(writer, value as f64)
    }

    fn begin_array<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.begin_nested(writer, b"[")
    }

    fn end_array<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.end_nested(writer, b"]")
    }

    fn begin_array_value<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.write_separator(writer, first)
    }

    fn end_array_value<W: ?Sized + Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.has_value = true;
        Ok(())
    }

    fn begin_object<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.begin_nested(writer, b"{")
    }

    fn end_object<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.end_nested(writer, b"}")
    }

    fn begin_object_key<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.write_separator(writer, first)
    }

    fn begin_object_value<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        writer.write_all(b": ")
    }

    fn end_object_value<W: ?Sized + Write>(&mut self, _writer: &mut W) -> io::Result<()> {
        self.has_value = true;
        Ok(())
    }
}
