use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::thread;

use fairspec_dataset::get_temp_file_path;
use fairspec_metadata::{FileDialect, Result};
use memchr::memmem;
use memmap2::Mmap;

use crate::helpers::file_dialect::get_header_rows;
use crate::plugins::csv::helpers::delimiter::{CHUNK_SIZE, open_file};

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const BOM: &[u8] = b"\xef\xbb\xbf";
// Below this one search is faster than starting the threads
const PARALLEL_SEARCH_BYTES: usize = 1 << 24;

pub struct CleanedRows {
    pub paths: Vec<String>,
    pub file_dialect: FileDialect,
}

struct RowScanner {
    eol: u8,
    delimiter: u8,
    quote: Option<u8>,
    null_sequence: Option<Vec<u8>>,
    comment_prefix: Option<Vec<u8>>,
    last_header_row: usize,
}

/// Polars reads a blank line as a row of nulls and a quoted `nullSequence` as null, so
/// files holding a blank line, or read with a `nullSequence`, are copied with blank lines
/// dropped and unquoted sequences emptied; the dialect returned has no `nullSequence`.
pub fn clean_csv_rows(
    paths: &[String],
    file_dialect: &FileDialect,
    max_bytes: Option<usize>,
) -> Result<Option<CleanedRows>> {
    let (line_terminator, null_sequence, comment_prefix, delimiter, quote) = match file_dialect {
        FileDialect::Csv(dialect) => (
            dialect.line_terminator.clone(),
            dialect.null_sequence.clone(),
            dialect.comment_prefix.clone(),
            dialect.delimiter.clone().unwrap_or_else(|| ",".to_string()),
            Some(
                dialect
                    .quote_char
                    .clone()
                    .unwrap_or_else(|| "\"".to_string()),
            ),
        ),
        FileDialect::Tsv(dialect) => (
            dialect.line_terminator.clone(),
            dialect.null_sequence.clone(),
            dialect.comment_prefix.clone(),
            "\t".to_string(),
            None,
        ),
        _ => return Ok(None),
    };
    let eol = line_terminator
        .and_then(|terminator| terminator.bytes().last())
        .unwrap_or(LF);

    if null_sequence.is_none() {
        let mut is_blank = false;
        for path in paths {
            if get_has_blank_line(path, eol, max_bytes)? {
                is_blank = true;
                break;
            }
        }
        if !is_blank {
            return Ok(None);
        }
    }

    let scanner = RowScanner {
        eol,
        delimiter: delimiter.bytes().next().unwrap_or(b','),
        quote: quote.and_then(|quote| quote.bytes().next()),
        null_sequence: null_sequence.map(String::into_bytes),
        comment_prefix: comment_prefix
            .filter(|prefix| !prefix.is_empty())
            .map(String::into_bytes),
        last_header_row: get_header_rows(Some(file_dialect))
            .into_iter()
            .max()
            .unwrap_or(0),
    };
    let paths = paths
        .iter()
        .map(|path| clean_file_rows(path, &scanner, max_bytes))
        .collect::<Result<Vec<_>>>()?;

    let mut file_dialect = file_dialect.clone();
    match &mut file_dialect {
        FileDialect::Csv(dialect) => dialect.null_sequence = None,
        FileDialect::Tsv(dialect) => dialect.null_sequence = None,
        _ => {}
    }
    Ok(Some(CleanedRows {
        paths,
        file_dialect,
    }))
}

// A blank line is two line ends with nothing (or only a CR) between them, or one at
// the start of the file after any BOM; the copy drops it unless inside a quoted cell
fn get_has_blank_line(path: &str, eol: u8, max_bytes: Option<usize>) -> Result<bool> {
    let file = File::open(path)?;
    // SAFETY: the mapping is read-only and dropped before this returns; a file truncated
    // meanwhile raises SIGBUS here, as it would in polars, which maps the same file to read it
    match unsafe { Mmap::map(&file) } {
        Ok(data) => {
            let end = max_bytes
                .filter(|max_bytes| *max_bytes > 0)
                .map_or(data.len(), |max_bytes| max_bytes.min(data.len()));
            Ok(find_blank_line(&data[..end], eol, PARALLEL_SEARCH_BYTES))
        }
        Err(_) => read_has_blank_line(path, eol, max_bytes),
    }
}

pub(crate) fn find_blank_line(data: &[u8], eol: u8, parallel_bytes: usize) -> bool {
    let head = &data[get_bom_length(data, true)..];
    if head.first() == Some(&eol) || (eol == LF && head.starts_with(&[CR, LF])) {
        return true;
    }
    let mut needles = vec![[eol, eol].to_vec()];
    if eol == LF {
        needles.push(vec![LF, CR, LF]);
    }
    let finders: Vec<memmem::Finder> = needles.iter().map(memmem::Finder::new).collect();
    let is_found = |slice: &[u8]| finders.iter().any(|finder| finder.find(slice).is_some());
    if data.len() < parallel_bytes.max(1) {
        return is_found(data);
    }

    // One slice per core, each overlapping the next by the longest needle less one
    let workers = thread::available_parallelism().map_or(1, |count| count.get());
    let step = data.len().div_ceil(workers);
    let overlap = needles.iter().map(Vec::len).max().unwrap_or(1) - 1;
    thread::scope(|scope| {
        let searches: Vec<_> = (0..data.len())
            .step_by(step)
            .map(|start| {
                let slice = &data[start..data.len().min(start + step + overlap)];
                scope.spawn(move || is_found(slice))
            })
            .collect();
        searches
            .into_iter()
            .any(|search| search.join().unwrap_or(true))
    })
}

// The byte loop for a file that cannot be mapped
fn read_has_blank_line(path: &str, eol: u8, max_bytes: Option<usize>) -> Result<bool> {
    let mut file = open_file(path, max_bytes)?;
    let mut chunk = vec![0; CHUNK_SIZE];
    // The file starts as if a line had just ended
    let mut previous = [eol, eol];
    let mut is_start = true;
    loop {
        let size = file.read(&mut chunk)?;
        if size == 0 {
            return Ok(false);
        }
        let start = get_bom_length(&chunk[..size], is_start);
        is_start = false;
        for &byte in &chunk[start..size] {
            let is_after_cr = eol == LF && previous[1] == CR && previous[0] == eol;
            if byte == eol && (previous[1] == eol || is_after_cr) {
                return Ok(true);
            }
            previous = [previous[1], byte];
        }
    }
}

fn clean_file_rows(path: &str, scanner: &RowScanner, max_bytes: Option<usize>) -> Result<String> {
    let target = get_temp_file_path(false, None, Some("csv"))?;
    let mut source = open_file(path, max_bytes)?;
    let mut output = BufWriter::new(File::create(&target)?);
    let mut parser = RowParser::new(scanner);
    let mut chunk = vec![0; CHUNK_SIZE];
    let mut is_start = true;
    loop {
        let size = source.read(&mut chunk)?;
        if size == 0 {
            break;
        }
        // The copy drops a BOM, so a quote or a line end right after it reads as at the start
        let start = get_bom_length(&chunk[..size], is_start);
        is_start = false;
        for &byte in &chunk[start..size] {
            parser.push(byte, &mut output)?;
        }
    }
    parser.flush(&mut output)?;
    output.flush()?;
    Ok(target)
}

#[derive(Default, Clone, Copy)]
struct CellState {
    is_quoted: bool,
    is_closed: bool,
    is_started: bool,
}

impl CellState {
    // Returns true when the byte ends the cell (an unquoted delimiter). A quote opens a
    // cell only at its start, and a doubled quote inside one is a quote
    fn step(&mut self, byte: u8, scanner: &RowScanner) -> bool {
        if self.is_quoted {
            if Some(byte) == scanner.quote {
                self.is_quoted = false;
                self.is_closed = true;
            }
            return false;
        }
        if byte == scanner.delimiter {
            *self = CellState::default();
            return true;
        }
        if Some(byte) == scanner.quote && (!self.is_started || self.is_closed) {
            self.is_quoted = true;
        }
        self.is_started = true;
        self.is_closed = false;
        false
    }
}

// A row is a record of the file: a line end inside a quoted cell does not end it
struct RowParser<'a> {
    scanner: &'a RowScanner,
    row: Vec<u8>,
    row_number: usize,
    cell: CellState,
    is_comment: bool,
    is_comment_candidate: bool,
}

impl<'a> RowParser<'a> {
    fn new(scanner: &'a RowScanner) -> Self {
        RowParser {
            scanner,
            row: Vec::new(),
            row_number: 0,
            cell: CellState::default(),
            is_comment: false,
            is_comment_candidate: scanner.comment_prefix.is_some(),
        }
    }

    fn push(&mut self, byte: u8, output: &mut impl Write) -> Result<()> {
        if byte == self.scanner.eol && !self.cell.is_quoted {
            return self.emit_row(output, true);
        }
        self.row.push(byte);
        if let Some(prefix) = &self.scanner.comment_prefix
            && self.is_comment_candidate
        {
            if prefix.get(self.row.len() - 1) != Some(&byte) {
                self.is_comment_candidate = false;
            } else if self.row.len() == prefix.len() {
                self.is_comment = true;
            }
        }
        if !self.is_comment {
            self.cell.step(byte, self.scanner);
        }
        Ok(())
    }

    fn flush(&mut self, output: &mut impl Write) -> Result<()> {
        if !self.row.is_empty() {
            self.emit_row(output, false)?;
        }
        Ok(())
    }

    fn emit_row(&mut self, output: &mut impl Write, is_terminated: bool) -> Result<()> {
        let scanner = self.scanner;
        let content = strip_cr(&self.row, scanner.eol);
        if !content.is_empty() {
            self.row_number += 1;
            let is_data = self.row_number > scanner.last_header_row && !self.is_comment;
            if scanner.null_sequence.is_some() && is_data {
                output.write_all(&empty_null_cells(&self.row, scanner))?;
            } else {
                output.write_all(&self.row)?;
            }
            if is_terminated {
                output.write_all(&[scanner.eol])?;
            }
        }
        self.row.clear();
        self.cell = CellState::default();
        self.is_comment = false;
        self.is_comment_candidate = scanner.comment_prefix.is_some();
        Ok(())
    }
}

fn strip_cr(bytes: &[u8], eol: u8) -> &[u8] {
    if eol == LF && bytes.last() == Some(&CR) {
        &bytes[..bytes.len() - 1]
    } else {
        bytes
    }
}

fn empty_null_cells(row: &[u8], scanner: &RowScanner) -> Vec<u8> {
    let mut cells: Vec<&[u8]> = Vec::new();
    let mut state = CellState::default();
    let mut start = 0;
    for (index, &byte) in row.iter().enumerate() {
        if state.step(byte, scanner) {
            cells.push(&row[start..index]);
            start = index + 1;
        }
    }
    cells.push(&row[start..]);

    let mut output = Vec::with_capacity(row.len());
    for (index, cell) in cells.into_iter().enumerate() {
        if index > 0 {
            output.push(scanner.delimiter);
        }
        let text = strip_cr(cell, scanner.eol);
        if scanner.null_sequence.as_deref() == Some(text) {
            output.extend_from_slice(&cell[text.len()..]);
        } else {
            output.extend_from_slice(cell);
        }
    }
    output
}

fn get_bom_length(data: &[u8], is_start: bool) -> usize {
    if is_start && data.starts_with(BOM) {
        BOM.len()
    } else {
        0
    }
}
