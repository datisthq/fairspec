use std::io::{Cursor, Write};

use fairspec_metadata::{Error, Result};
use rust_xlsxwriter::Workbook;
use zip::CompressionMethod;
use zip::write::{SimpleFileOptions, ZipWriter};

use super::sheet::SheetCell;
use super::table::format_sheet_cell;

#[derive(Debug, Clone, PartialEq)]
pub enum SheetValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SheetData {
    pub name: String,
    pub rows: Vec<Vec<Option<SheetValue>>>,
}

pub fn write_xlsx_buffer(sheets: &[SheetData]) -> Result<Vec<u8>> {
    let to_error = |error: rust_xlsxwriter::XlsxError| Error::Load(error.to_string());
    let mut book = Workbook::new();

    for sheet_data in sheets {
        let sheet = book.add_worksheet();
        sheet.set_name(&sheet_data.name).map_err(to_error)?;
        for (row, values) in sheet_data.rows.iter().enumerate() {
            let row = row as u32;
            for (col, value) in values.iter().enumerate() {
                let col = col as u16;
                match value {
                    Some(SheetValue::Boolean(value)) => {
                        sheet.write_boolean(row, col, *value).map_err(to_error)?;
                    }
                    Some(SheetValue::Number(value)) => {
                        let value = round_significant_digits(*value);
                        sheet.write_number(row, col, value).map_err(to_error)?;
                    }
                    Some(SheetValue::String(value)) => {
                        sheet.write_string(row, col, value).map_err(to_error)?;
                    }
                    None => {}
                }
            }
        }
    }

    book.save_to_buffer().map_err(to_error)
}

/// Python's xlsxwriter writes a number with 16 significant digits (`%.16G`), so the value
/// calamine reads back from Python's twin is rounded the same way.
fn round_significant_digits(value: f64) -> f64 {
    format!("{value:.15e}").parse().unwrap_or(value)
}

pub fn write_ods_buffer(sheets: &[SheetData]) -> Result<Vec<u8>> {
    let tables: String = sheets
        .iter()
        .map(|sheet| {
            let rows: String = sheet
                .rows
                .iter()
                .map(|row| {
                    let cells: String = row
                        .iter()
                        .map(|value| create_ods_cell(value.as_ref()))
                        .collect();
                    format!("<table:table-row>{cells}</table:table-row>")
                })
                .collect();
            format!(
                "<table:table table:name=\"{}\">{rows}</table:table>",
                escape_xml(&sheet.name)
            )
        })
        .collect();

    let content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
         <office:document-content xmlns:office=\"{OFFICE_NS}\" xmlns:table=\"{TABLE_NS}\" \
         xmlns:text=\"{TEXT_NS}\" office:version=\"1.2\"><office:body><office:spreadsheet>\
         {tables}</office:spreadsheet></office:body></office:document-content>"
    );

    let to_error = |error: zip::result::ZipError| Error::Load(error.to_string());
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    let deflated = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    archive.start_file("mimetype", stored).map_err(to_error)?;
    archive.write_all(ODS_MIMETYPE.as_bytes())?;
    archive
        .start_file("META-INF/manifest.xml", deflated)
        .map_err(to_error)?;
    archive.write_all(create_ods_manifest().as_bytes())?;
    archive
        .start_file("content.xml", deflated)
        .map_err(to_error)?;
    archive.write_all(content.as_bytes())?;

    Ok(archive.finish().map_err(to_error)?.into_inner())
}

const OFFICE_NS: &str = "urn:oasis:names:tc:opendocument:xmlns:office:1.0";
const TABLE_NS: &str = "urn:oasis:names:tc:opendocument:xmlns:table:1.0";
const TEXT_NS: &str = "urn:oasis:names:tc:opendocument:xmlns:text:1.0";
const ODS_MIMETYPE: &str = "application/vnd.oasis.opendocument.spreadsheet";

fn create_ods_manifest() -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
         <manifest:manifest xmlns:manifest=\"urn:oasis:names:tc:opendocument:xmlns:manifest:1.0\" \
         manifest:version=\"1.2\">\
         <manifest:file-entry manifest:full-path=\"/\" manifest:media-type=\"{ODS_MIMETYPE}\"/>\
         <manifest:file-entry manifest:full-path=\"content.xml\" manifest:media-type=\"text/xml\"/>\
         </manifest:manifest>"
    )
}

// calamine rejects whitespace between cells, so the document is written without any
fn create_ods_cell(value: Option<&SheetValue>) -> String {
    match value {
        None => "<table:table-cell/>".to_string(),
        Some(SheetValue::Boolean(value)) => {
            let text = value.to_string();
            format!(
                "<table:table-cell office:value-type=\"boolean\" office:boolean-value=\"{text}\">\
                 <text:p>{}</text:p></table:table-cell>",
                text.to_uppercase()
            )
        }
        Some(SheetValue::Number(value)) => {
            let text = format_sheet_cell(Some(&SheetCell::Number(*value))).unwrap_or_default();
            format!(
                "<table:table-cell office:value-type=\"float\" office:value=\"{text}\">\
                 <text:p>{text}</text:p></table:table-cell>"
            )
        }
        Some(SheetValue::String(value)) => format!(
            "<table:table-cell office:value-type=\"string\"><text:p>{}</text:p></table:table-cell>",
            escape_xml(value)
        ),
    }
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
