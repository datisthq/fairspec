use std::io::{Cursor, Write};

use fairspec_dataset::write_temp_file;
use zip::write::{SimpleFileOptions, ZipWriter};

use crate::plugins::xlsx::helpers::guard::assert_sheet_size;

const TOO_LARGE: &str = "Sheet is too large to load";
const FAR_CELLS: &str = "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
    <row r=\"1048576\"><c r=\"XFD1048576\"><v>1</v></c></row>";

fn create_sheet(sheet_data: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><worksheet \
         xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">\
         <sheetData>{sheet_data}</sheetData></worksheet>"
    )
}

fn create_zip(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
    for (name, content) in files {
        archive
            .start_file(*name, SimpleFileOptions::default())
            .unwrap();
        archive.write_all(content).unwrap();
    }
    archive.finish().unwrap().into_inner()
}

fn write_file(content: impl AsRef<[u8]>) -> String {
    write_temp_file(content, false, None, None).unwrap()
}

fn write_xlsx(sheet_data: &str) -> String {
    write_file(create_zip(&[(
        "xl/worksheets/sheet1.xml",
        create_sheet(sheet_data).as_bytes(),
    )]))
}

fn write_ods(rows: &str) -> String {
    let content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><office:document-content \
         xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\" \
         xmlns:table=\"urn:oasis:names:tc:opendocument:xmlns:table:1.0\" \
         xmlns:text=\"urn:oasis:names:tc:opendocument:xmlns:text:1.0\">\
         <office:body><office:spreadsheet>\
         <table:table table:name=\"Sheet1\">{rows}</table:table>\
         </office:spreadsheet></office:body></office:document-content>"
    );
    write_file(create_zip(&[("content.xml", content.as_bytes())]))
}

fn assert_error(path: &str, message: &str) {
    let error = assert_sheet_size(path).unwrap_err().to_string();
    assert!(error.contains(message), "{error}");
}

#[test]
fn test_should_accept_a_small_sheet() {
    let path = write_xlsx("<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\"><v>2</v></c></row>");
    assert_sheet_size(&path).unwrap();
}

#[test]
fn test_should_reject_a_sheet_naming_a_far_cell() {
    assert_error(&write_xlsx(FAR_CELLS), TOO_LARGE);
}

#[test]
fn test_should_accept_a_far_cell_without_a_value() {
    let path = write_xlsx(
        "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
         <row r=\"1048576\"><c r=\"XFD1048576\" s=\"1\"/></row>",
    );
    assert_sheet_size(&path).unwrap();
}

#[test]
fn test_should_read_a_reference_after_a_quoted_gt() {
    let path = write_xlsx(
        "<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>\
         <row><c t=\"n\" x=\">\" r=\"XFD1048576\"><v>1</v></c></row>",
    );
    assert_error(&path, TOO_LARGE);
}

#[test]
fn test_should_place_cells_without_a_reference_one_after_another() {
    let wide = "<c><v>1</v></c>".repeat(10_001);
    let path = write_xlsx(&format!(
        "<row r=\"1\">{wide}</row><row r=\"5001\"><c r=\"A5001\"><v>1</v></c></row>"
    ));
    assert_error(&path, TOO_LARGE);
}

#[test]
fn test_should_reject_ods_cells_repeated_over_a_large_range() {
    let path = write_ods(
        "<table:table-row table:number-rows-repeated=\"1048576\">\
         <table:table-cell table:number-columns-repeated=\"16384\" \
         office:value-type=\"float\" office:value=\"1\"><text:p>1</text:p>\
         </table:table-cell></table:table-row>",
    );
    assert_error(&path, TOO_LARGE);
}

#[test]
fn test_should_accept_empty_ods_cells_repeated_to_the_sheet_edge() {
    let path = write_ods(
        "<table:table-row><table:table-cell office:value-type=\"float\" \
         office:value=\"1\"><text:p>1</text:p></table:table-cell>\
         <table:table-cell table:number-columns-repeated=\"16383\"/></table:table-row>\
         <table:table-row table:number-rows-repeated=\"1048575\">\
         <table:table-cell table:number-columns-repeated=\"16384\"/></table:table-row>",
    );
    assert_sheet_size(&path).unwrap();
}

#[test]
fn test_should_reject_an_xlsb_workbook() {
    let path = write_file(create_zip(&[("xl/workbook.bin", b"")]));
    assert_error(&path, "xlsb is not supported");
}

#[test]
fn test_should_skip_a_file_that_is_not_a_zip_archive() {
    assert_sheet_size(&write_file("id,name\n1,a\n")).unwrap();
}

#[test]
fn test_should_find_a_zip_archive_behind_prepended_data() {
    let archive = create_zip(&[(
        "xl/worksheets/sheet1.xml",
        create_sheet(FAR_CELLS).as_bytes(),
    )]);
    let path = write_file([b"junk".as_slice(), &archive].concat());
    assert_error(&path, TOO_LARGE);
}

#[test]
fn test_should_scan_a_sheet_the_workbook_relationships_place_elsewhere() {
    let relationships =
        "<Relationships><Relationship Id=\"rId1\" Target=\"data/s.xml\"/></Relationships>";
    let path = write_file(create_zip(&[
        ("xl/_rels/workbook.xml.rels", relationships.as_bytes()),
        ("xl/data/s.xml", create_sheet(FAR_CELLS).as_bytes()),
    ]));
    assert_error(&path, TOO_LARGE);
}

#[test]
fn test_should_reject_an_archive_with_duplicate_entries() {
    // The zip writer refuses a duplicate name, so a second name of the same length is renamed
    let small = create_sheet("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>");
    let mut archive = create_zip(&[
        ("xl/worksheets/sheet1.xml", small.as_bytes()),
        (
            "xl/worksheets/sheet2.xml",
            create_sheet(FAR_CELLS).as_bytes(),
        ),
    ]);
    let (from, to) = (b"xl/worksheets/sheet2.xml", b"xl/worksheets/sheet1.xml");
    for start in 0..archive.len() - from.len() {
        if &archive[start..start + from.len()] == from {
            archive[start..start + to.len()].copy_from_slice(to);
        }
    }
    assert_error(&write_file(archive), "duplicate entries");
}

#[test]
fn test_should_decode_a_utf_16_sheet() {
    let sheet: Vec<u8> = format!("\u{feff}{}", create_sheet(FAR_CELLS))
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    let path = write_file(create_zip(&[("xl/worksheets/sheet1.xml", &sheet)]));
    assert_error(&path, TOO_LARGE);
}
