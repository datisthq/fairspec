use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::{LazyLock, Mutex};

use encoding_rs::{Decoder, UTF_8, UTF_16BE, UTF_16LE};
use fairspec_metadata::{Error, Result};
use indexmap::IndexMap;
use regex::Regex;
use zip::{CompressionMethod, ZipArchive};

pub const MAX_SHEET_CELLS: i128 = 50_000_000;

/// Fails when the workbook's sheets span more cells than a load can hold: calamine
/// allocates every cell between the first and the last used one before returning, so a
/// tiny file naming a far cell would abort the process.
pub fn assert_sheet_size(path: &str) -> Result<()> {
    let metadata = std::fs::metadata(path)?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |duration| duration.as_nanos());
    let key = (path.to_string(), metadata.len(), modified);
    if CHECKED.lock().unwrap().contains_key(&key) {
        return Ok(());
    }

    // Not a zip archive: calamine then reads it as xls, which caps a sheet's size
    if get_is_zip_file(path)? {
        let file = BufReader::new(File::open(path)?);
        let mut archive = ZipArchive::new(file).map_err(create_zip_error)?;
        assert_archive_size(&mut archive, count_central_directory_entries(path)?)?;
    }

    let mut checked = CHECKED.lock().unwrap();
    checked.insert(key, ());
    while checked.len() > MAX_CHECKED {
        checked.shift_remove_index(0);
    }
    Ok(())
}

type CheckedKey = (String, u64, u128);

static CHECKED: LazyLock<Mutex<IndexMap<CheckedKey, ()>>> =
    LazyLock::new(|| Mutex::new(IndexMap::new()));
const MAX_CHECKED: usize = 64;
const MAX_TAG_LENGTH: usize = 1 << 24;
const CHUNK_SIZE: usize = 1 << 20;

// Quoted attribute values may hold ">", which must not end the tag early. The regex crate
// has no lookahead, so the attributes either are empty or open with a space or a slash
const ATTRS: &str = r#"((?:[\s/][^>"']*(?:(?:"[^"]*"|'[^']*')[^>"']*)*)?)"#;

static XLSX_SHEET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^xl/worksheets/[^/]+\.xml$").unwrap());
static XLSX_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"<(?:[\w.-]+:)?(c|row){ATTRS}>")).unwrap());
static ODS_TAG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"<(/?)(?:[\w.-]+:)?(table-row|table-cell|covered-table-cell|table){ATTRS}>"
    ))
    .unwrap()
});
static XLSX_FAST_TAG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<(?:[\w.-]+:)?c(?:\s+r="([A-Za-z]{1,3})([0-9]{1,7})"|[\s/>])"#).unwrap()
});
static RELATIONSHIP_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"<(?:[\w.-]+:)?(Relationship){ATTRS}>")).unwrap());
static ATTRIBUTES: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn assert_archive_size(
    archive: &mut ZipArchive<BufReader<File>>,
    entry_count: Option<usize>,
) -> Result<()> {
    if entry_count.is_some_and(|count| count != archive.len()) {
        return Err(create_duplicate_error());
    }

    let mut by_name: HashMap<String, usize> = HashMap::new();
    for index in 0..archive.len() {
        let file = archive.by_index_raw(index).map_err(create_zip_error)?;
        let name = file.name().map_err(create_zip_error)?.to_lowercase();
        if by_name.insert(name, index).is_some() {
            return Err(create_duplicate_error());
        }
    }

    if by_name.contains_key("xl/workbook.bin") {
        return Err(Error::Load(
            "Spreadsheet format xlsb is not supported".to_string(),
        ));
    }

    // calamine opens the sheets the workbook's relationships name, wherever they live
    let mut sheets: IndexMap<String, usize> = by_name
        .iter()
        .filter(|(name, _)| XLSX_SHEET.is_match(name))
        .map(|(name, index)| (name.clone(), *index))
        .collect();
    sheets.sort_keys();
    if let Some(&index) = by_name.get("xl/_rels/workbook.xml.rels") {
        let mut reader = RelationshipReader::default();
        scan_zip_entry(archive, index, &mut reader)?;
        for target in reader.targets {
            if let Some(&index) = by_name.get(&target.to_lowercase()) {
                sheets.insert(target.to_lowercase(), index);
            }
        }
    }

    let mut total = 0;
    for &index in sheets.values() {
        let mut fast_scanner = XlsxFastScanner::default();
        let mut fast = scan_zip_entry(archive, index, &mut fast_scanner)?;
        if fast.is_none_or(|fast| total + fast > MAX_SHEET_CELLS) {
            let mut scanner = XlsxScanner::new(Bounds::new(total));
            fast = scan_zip_entry(archive, index, &mut scanner)?;
        }
        total += fast.unwrap_or(0);
    }

    if let Some(&index) = by_name.get("content.xml") {
        scan_zip_entry(archive, index, &mut OdsScanner::new(total))?;
    }

    Ok(())
}

/// Python's `zipfile.is_zipfile`: an end of central directory record in the file's tail.
fn get_is_zip_file(path: &str) -> Result<bool> {
    Ok(find_end_record(path)?.is_some())
}

fn find_end_record(path: &str) -> Result<Option<Vec<u8>>> {
    let mut file = File::open(path)?;
    let length = file.metadata()?.len();
    let tail_length = length.min(22 + 65_535);
    file.seek(SeekFrom::Start(length - tail_length))?;
    let mut tail = Vec::new();
    file.read_to_end(&mut tail)?;
    Ok(tail
        .windows(4)
        .rposition(|window| window == b"PK\x05\x06")
        .filter(|&position| tail.len() - position >= 22)
        .map(|position| tail[position..].to_vec()))
}

/// The entry count the end record states; the zip crate silently keeps one entry of each
/// duplicated name, so a mismatch reveals duplicates. A zip64 archive states no count.
fn count_central_directory_entries(path: &str) -> Result<Option<usize>> {
    let Some(record) = find_end_record(path)? else {
        return Ok(None);
    };
    let count = u16::from_le_bytes([record[10], record[11]]);
    Ok((count != u16::MAX).then_some(count as usize))
}

fn scan_zip_entry(
    archive: &mut ZipArchive<BufReader<File>>,
    index: usize,
    scanner: &mut dyn TagScanner,
) -> Result<Option<i128>> {
    let mut file = archive.by_index(index).map_err(create_zip_error)?;
    if !matches!(
        file.compression(),
        CompressionMethod::Stored | CompressionMethod::Deflated
    ) {
        return Err(Error::Load(
            "Spreadsheet archive uses an unsupported compression method".to_string(),
        ));
    }

    let mut reader = TagReader::default();
    let mut decoder: Option<Decoder> = None;
    let mut chunk = vec![0; CHUNK_SIZE];
    loop {
        let size = read_chunk(&mut file, &mut chunk)?;
        if size == 0 {
            break;
        }
        let decoder = decoder.get_or_insert_with(|| {
            detect_encoding(&chunk[..size]).new_decoder_without_bom_handling()
        });
        let text = decode_chunk(decoder, &chunk[..size], false);
        reader.push(&text, false, scanner)?;
    }
    if let Some(decoder) = decoder.as_mut() {
        let text = decode_chunk(decoder, &[], true);
        reader.push(&text, true, scanner)?;
    }

    scanner.finish()
}

fn read_chunk(reader: &mut impl Read, chunk: &mut [u8]) -> Result<usize> {
    let mut size = 0;
    while size < chunk.len() {
        let read = reader.read(&mut chunk[size..])?;
        if read == 0 {
            break;
        }
        size += read;
    }
    Ok(size)
}

fn decode_chunk(decoder: &mut Decoder, bytes: &[u8], last: bool) -> String {
    let capacity = decoder
        .max_utf8_buffer_length(bytes.len())
        .unwrap_or(bytes.len() * 3 + 16);
    let mut text = String::with_capacity(capacity);
    let _ = decoder.decode_to_string(bytes, &mut text, last);
    text
}

// calamine decodes UTF-16 sheets too, which read as UTF-8 would show no tags
fn detect_encoding(data: &[u8]) -> &'static encoding_rs::Encoding {
    match data.get(..2) {
        Some(b"\xff\xfe") | Some(b"<\x00") => UTF_16LE,
        Some(b"\xfe\xff") | Some(b"\x00<") => UTF_16BE,
        _ => UTF_8,
    }
}

fn create_zip_error(error: zip::result::ZipError) -> Error {
    Error::Load(format!("Spreadsheet archive is not valid: {error}"))
}

fn create_duplicate_error() -> Error {
    Error::Load("Spreadsheet archive has duplicate entries".to_string())
}

fn create_size_error() -> Error {
    Error::Load(format!(
        "Sheet is too large to load: it spans more than {MAX_SHEET_CELLS} cells"
    ))
}

/// The used range of a sheet; every growth checks the cells it spans, plus `offset`.
struct Bounds {
    offset: i128,
    min_row: i128,
    max_row: i128,
    min_col: i128,
    max_col: i128,
}

impl Bounds {
    fn new(offset: i128) -> Self {
        Self {
            offset,
            min_row: i128::MAX,
            max_row: 0,
            min_col: i128::MAX,
            max_col: 0,
        }
    }

    fn count(&self) -> i128 {
        if self.max_row == 0 {
            return 0;
        }
        let rows = self.max_row - self.min_row + 1;
        rows.saturating_mul(self.max_col - self.min_col + 1)
    }

    fn get_is_inside(&self, row: i128, col: i128) -> bool {
        self.min_row <= row && row <= self.max_row && self.min_col <= col && col <= self.max_col
    }

    fn add(&mut self, from_row: i128, to_row: i128, from_col: i128, to_col: i128) -> Result<()> {
        if from_row >= self.min_row
            && to_row <= self.max_row
            && from_col >= self.min_col
            && to_col <= self.max_col
        {
            return Ok(());
        }
        self.min_row = self.min_row.min(from_row);
        self.max_row = self.max_row.max(to_row);
        self.min_col = self.min_col.min(from_col);
        self.max_col = self.max_col.max(to_col);
        if self.offset.saturating_add(self.count()) > MAX_SHEET_CELLS {
            return Err(create_size_error());
        }
        Ok(())
    }
}

trait TagScanner {
    fn get_pattern(&self) -> &'static Regex;
    fn on_tags(&mut self, tags: Vec<Vec<String>>) -> Result<()>;
    fn finish(&self) -> Result<Option<i128>>;
}

#[derive(Default)]
struct TagReader {
    rest: String,
}

impl TagReader {
    fn push(&mut self, text: &str, last: bool, scanner: &mut dyn TagScanner) -> Result<()> {
        let data = std::mem::take(&mut self.rest) + text;

        // A tag cannot hold "<", so only the text from the last one can be incomplete
        let cut = if last {
            data.len()
        } else {
            data.rfind('<').unwrap_or(0)
        };
        self.rest = data[cut..].to_string();
        if self.rest.len() > MAX_TAG_LENGTH {
            return Err(Error::Load("Spreadsheet markup is malformed".to_string()));
        }

        let tags = scanner
            .get_pattern()
            .captures_iter(&data[..cut])
            .map(|captures| {
                captures
                    .iter()
                    .skip(1)
                    .map(|group| group.map_or_else(String::new, |group| group.as_str().to_string()))
                    .collect()
            })
            .collect();
        scanner.on_tags(tags)
    }
}

#[derive(Default)]
struct RelationshipReader {
    targets: Vec<String>,
}

impl TagScanner for RelationshipReader {
    fn get_pattern(&self) -> &'static Regex {
        &RELATIONSHIP_TAG
    }

    fn on_tags(&mut self, tags: Vec<Vec<String>>) -> Result<()> {
        for tag in tags {
            let Some(target) = get_attribute(&tag[1], "Target").filter(|target| !target.is_empty())
            else {
                continue;
            };
            match target.strip_prefix('/') {
                Some(target) => self.targets.push(target.to_string()),
                None => {
                    self.targets
                        .push(normalize_posix_path(&format!("xl/{target}")));
                    self.targets.push(format!("xl/{target}"));
                }
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<Option<i128>> {
        Ok(None)
    }
}

/// Writers put r="A1" first on every cell, so the used range comes from a single regex.
/// It counts empty cells too, an upper bound: on no answer the exact scan runs.
struct XlsxFastScanner {
    is_exact: bool,
    letters: std::collections::HashSet<String>,
    min_row: i128,
    max_row: i128,
}

impl Default for XlsxFastScanner {
    fn default() -> Self {
        Self {
            is_exact: true,
            letters: Default::default(),
            min_row: 0,
            max_row: 0,
        }
    }
}

impl TagScanner for XlsxFastScanner {
    fn get_pattern(&self) -> &'static Regex {
        &XLSX_FAST_TAG
    }

    fn on_tags(&mut self, tags: Vec<Vec<String>>) -> Result<()> {
        if tags.is_empty() || !self.is_exact {
            return Ok(());
        }
        if tags.iter().any(|tag| tag[0].is_empty()) {
            self.is_exact = false;
            return Ok(());
        }
        for tag in tags {
            let row: i128 = tag[1].parse().unwrap_or_default();
            self.letters.insert(tag[0].clone());
            self.min_row = if self.min_row == 0 {
                row
            } else {
                self.min_row.min(row)
            };
            self.max_row = self.max_row.max(row);
        }
        Ok(())
    }

    fn finish(&self) -> Result<Option<i128>> {
        if !self.is_exact {
            return Ok(None);
        }
        let cols: Vec<i128> = self
            .letters
            .iter()
            .map(|letters| parse_column_letters(letters))
            .collect();
        let (Some(min_col), Some(max_col)) = (cols.iter().min(), cols.iter().max()) else {
            return Ok(Some(0));
        };
        Ok(Some(
            (self.max_row - self.min_row + 1) * (max_col - min_col + 1),
        ))
    }
}

struct XlsxScanner {
    bounds: Bounds,
    columns: HashMap<String, i128>,
    row: i128,
    col: i128,
}

impl XlsxScanner {
    fn new(bounds: Bounds) -> Self {
        Self {
            bounds,
            columns: HashMap::new(),
            row: 0,
            col: 0,
        }
    }
}

impl TagScanner for XlsxScanner {
    fn get_pattern(&self) -> &'static Regex {
        &XLSX_TAG
    }

    fn on_tags(&mut self, tags: Vec<Vec<String>>) -> Result<()> {
        for tag in tags {
            let (name, attrs) = (&tag[0], &tag[1]);
            let reference = get_attribute(attrs, "r");

            if name == "row" {
                let number = to_number(reference.as_deref());
                self.row = if number != 0 { number } else { self.row + 1 };
                self.col = 0;
                continue;
            }

            let reference = reference.unwrap_or_default();
            let letters = reference.trim_end_matches(|char: char| char.is_ascii_digit());
            let digits = &reference[letters.len()..];
            if !letters.is_empty()
                && letters.chars().all(|char| char.is_ascii_alphabetic())
                && !digits.is_empty()
            {
                let col = match self.columns.get(letters) {
                    Some(&col) => col,
                    None => parse_column_letters(letters),
                };
                self.columns.insert(letters.to_string(), col);
                self.col = col;
                self.row = parse_saturating(digits);
            } else {
                self.col += 1;
            }

            if !attrs.ends_with('/') && !self.bounds.get_is_inside(self.row, self.col) {
                self.bounds.add(self.row, self.row, self.col, self.col)?;
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<Option<i128>> {
        Ok(Some(self.bounds.count()))
    }
}

struct OdsScanner {
    offset: i128,
    previous: i128,
    bounds: Bounds,
    row: i128,
    col: i128,
    row_repeat: i128,
    row_min_col: i128,
    row_max_col: i128,
}

impl OdsScanner {
    fn new(offset: i128) -> Self {
        Self {
            offset,
            previous: 0,
            bounds: Bounds::new(offset),
            row: 0,
            col: 0,
            row_repeat: 1,
            row_min_col: i128::MAX,
            row_max_col: 0,
        }
    }

    fn on_tag(&mut self, is_closing: bool, name: &str, attrs: &str) -> Result<()> {
        let is_empty = attrs.ends_with('/');

        if name == "table" && !is_closing {
            self.previous += self.bounds.count();
            self.bounds = Bounds::new(self.offset + self.previous);
            self.row = 0;
            return Ok(());
        }

        if name == "table-row" {
            if !is_closing {
                self.row_repeat = get_repeat(attrs, "number-rows-repeated");
                self.col = 0;
            }
            if is_closing || is_empty {
                if self.row_max_col != 0 {
                    self.bounds.add(
                        self.row + 1,
                        self.row.saturating_add(self.row_repeat),
                        self.row_min_col,
                        self.row_max_col,
                    )?;
                }
                self.row = self.row.saturating_add(self.row_repeat);
                self.row_min_col = i128::MAX;
                self.row_max_col = 0;
            }
            return Ok(());
        }

        if name != "table" && !is_closing {
            let start = self.col + 1;
            self.col = self
                .col
                .saturating_add(get_repeat(attrs, "number-columns-repeated"));
            if !is_empty || get_attribute(attrs, "value-type").is_some() {
                self.row_min_col = self.row_min_col.min(start);
                self.row_max_col = self.row_max_col.max(self.col);
            }
        }
        Ok(())
    }
}

impl TagScanner for OdsScanner {
    fn get_pattern(&self) -> &'static Regex {
        &ODS_TAG
    }

    fn on_tags(&mut self, tags: Vec<Vec<String>>) -> Result<()> {
        for tag in tags {
            self.on_tag(tag[0] == "/", &tag[1], &tag[2])?;
        }
        Ok(())
    }

    fn finish(&self) -> Result<Option<i128>> {
        Ok(Some(self.previous + self.bounds.count()))
    }
}

fn get_attribute(attrs: &str, name: &str) -> Option<String> {
    let quick = format!(" {name}=\"");
    if let Some(rest) = attrs.strip_prefix(&quick) {
        return rest.find('"').map(|end| rest[..end].to_string());
    }

    let mut patterns = ATTRIBUTES.lock().unwrap();
    let pattern = patterns.entry(name.to_string()).or_insert_with(|| {
        Regex::new(&format!(
            r#"(?:^|\s)(?:[\w.-]+:)?{name}\s*=\s*(?:"([^"]*)"|'([^']*)')"#
        ))
        .unwrap()
    });
    let captures = pattern.captures(attrs)?;
    captures
        .get(1)
        .or_else(|| captures.get(2))
        .map(|value| value.as_str().to_string())
}

fn get_repeat(attrs: &str, name: &str) -> i128 {
    to_number(get_attribute(attrs, name).as_deref()).max(1)
}

fn to_number(value: Option<&str>) -> i128 {
    match value {
        Some(value) if !value.is_empty() && value.chars().all(|char| char.is_ascii_digit()) => {
            parse_saturating(value)
        }
        _ => 0,
    }
}

/// Digits beyond any real sheet size saturate: the size check rejects them either way.
fn parse_saturating(digits: &str) -> i128 {
    digits.parse().unwrap_or(i128::MAX / 4)
}

fn parse_column_letters(letters: &str) -> i128 {
    letters
        .to_ascii_uppercase()
        .bytes()
        .fold(0, |col, letter| col * 26 + letter as i128 - 64)
}

/// Python's `posixpath.normpath` for a relative path.
fn normalize_posix_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|last| *last != "..") => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    if parts.is_empty() {
        return ".".to_string();
    }
    parts.join("/")
}
