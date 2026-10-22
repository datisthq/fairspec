use std::sync::LazyLock;

use fairspec_metadata::{CellError, Column, Result, WktColumn};
use regex::Regex;

use crate::actions::column::_shared::helpers::inspect_text_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;

pub fn inspect_wkt_column(
    column: &WktColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_text_column(&Column::Wkt(column.clone()), table, &parse_wkt, mapping)
}

/// A C `strtod` that consumes the whole token, as GEOS reads a WKT number.
static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^[+-]?(?:",
        r"(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:[eE][+-]?[0-9]+)?",
        r"|0[xX](?:[0-9a-fA-F]+\.?[0-9a-fA-F]*|\.[0-9a-fA-F]+)(?:[pP][+-]?[0-9]+)?",
        r"|(?i:inf|infinity|nan(?:\([0-9A-Za-z_]*\))?)",
        r")$",
    ))
    .expect("valid regex")
});

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Open,
    Close,
    Comma,
    Number(f64),
    Word(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ordinates {
    Xy,
    Xyz,
    Xym,
    Xyzm,
}

impl Ordinates {
    fn count(self) -> usize {
        match self {
            Ordinates::Xy => 2,
            Ordinates::Xyz | Ordinates::Xym => 3,
            Ordinates::Xyzm => 4,
        }
    }
}

/// Whether GEOS (shapely's `from_wkt`) reads the text as a linear geometry.
fn parse_wkt(text: &str) -> bool {
    let text = text.split('\0').next().unwrap_or_default();
    let mut reader = WktReader {
        tokens: tokenize(text),
        position: 0,
    };
    reader.read_geometry(None).is_some() && reader.position == reader.tokens.len()
}

fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut rest = text;
    loop {
        rest = rest.trim_start_matches(|char: char| char.is_ascii_whitespace() || char == '\u{b}');
        let Some(char) = rest.chars().next() else {
            return tokens;
        };
        let token = match char {
            '(' => Token::Open,
            ')' => Token::Close,
            ',' => Token::Comma,
            _ => {
                let end = rest
                    .find(['\n', '\r', '\t', '(', ')', ' ', ','])
                    .unwrap_or(rest.len());
                let word = &rest[..end];
                rest = &rest[end..];
                tokens.push(match parse_number(word) {
                    Some(number) => Token::Number(number),
                    None => Token::Word(word.to_uppercase()),
                });
                continue;
            }
        };
        tokens.push(token);
        rest = &rest[1..];
    }
}

fn parse_number(word: &str) -> Option<f64> {
    if !NUMBER_REGEX.is_match(word) {
        return None;
    }
    let lower = word.to_ascii_lowercase();
    let unsigned = lower.trim_start_matches(['+', '-']);
    let sign = if lower.starts_with('-') { -1.0 } else { 1.0 };
    if unsigned.starts_with("nan") {
        return Some(f64::NAN);
    }
    if unsigned.starts_with("inf") {
        return Some(sign * f64::INFINITY);
    }
    if let Some(hex) = unsigned.strip_prefix("0x") {
        let (mantissa, exponent) = hex.split_once('p').unwrap_or((hex, "0"));
        let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let mut value = 0.0;
        for digit in integer.chars().chain(fraction.chars()) {
            value = value * 16.0 + f64::from(digit.to_digit(16)?);
        }
        let exponent: i32 = exponent.parse().unwrap_or(0);
        let scale = exponent - 4 * fraction.len() as i32;
        return Some(sign * value * 2f64.powi(scale));
    }
    lower.parse().ok()
}

struct WktReader {
    tokens: Vec<Token>,
    position: usize,
}

impl WktReader {
    fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn read_geometry(&mut self, parent: Option<Ordinates>) -> Option<Ordinates> {
        let Some(Token::Word(word)) = self.next_token() else {
            return None;
        };
        let (kind, mut ordinates) = split_geometry_type(&word)?;
        if ordinates.is_none()
            && let Some(Token::Word(word)) = self.peek_token()
        {
            ordinates = match word.as_str() {
                "Z" => Some(Ordinates::Xyz),
                "M" => Some(Ordinates::Xym),
                "ZM" => Some(Ordinates::Xyzm),
                _ => None,
            };
            if ordinates.is_some() {
                self.position += 1;
            }
        }

        let mut dimension = ordinates;
        let is_empty = match self.next_token()? {
            Token::Word(word) if word == "EMPTY" => true,
            Token::Open => false,
            _ => return None,
        };
        let actual = if is_empty {
            ordinates.unwrap_or(Ordinates::Xy)
        } else {
            match kind {
                "POINT" => {
                    self.read_coordinate(&mut dimension)?;
                    self.expect(Token::Close)?;
                }
                "LINESTRING" => self.read_line(&mut dimension, false)?,
                "LINEARRING" => self.read_line(&mut dimension, true)?,
                "POLYGON" => self.read_polygon(&mut dimension)?,
                "MULTIPOINT" => self.read_multi_point(&mut dimension)?,
                "MULTILINESTRING" => self.read_list(|reader| {
                    reader.read_empty_or(|reader| reader.read_line(&mut dimension, false))
                })?,
                "MULTIPOLYGON" => self.read_list(|reader| {
                    reader.read_empty_or(|reader| reader.read_polygon(&mut dimension))
                })?,
                _ => {
                    let mut children = Vec::new();
                    self.read_list(|reader| {
                        children.push(reader.read_geometry(ordinates)?);
                        Some(())
                    })?;
                    dimension = ordinates.or_else(|| children.into_iter().reduce(merge_ordinates));
                }
            }
            dimension.unwrap_or(Ordinates::Xy)
        };
        match parent {
            Some(parent) if parent != actual => None,
            _ => Some(actual),
        }
    }

    fn read_multi_point(&mut self, dimension: &mut Option<Ordinates>) -> Option<()> {
        let is_nested = matches!(self.peek_token(), Some(Token::Open))
            || matches!(self.peek_token(), Some(Token::Word(word)) if word == "EMPTY");
        self.read_list(|reader| {
            if !is_nested {
                return reader.read_coordinate(dimension).map(|_| ());
            }
            reader.read_empty_or(|reader| {
                reader.read_coordinate(dimension)?;
                reader.expect(Token::Close)
            })
        })
    }

    fn read_polygon(&mut self, dimension: &mut Option<Ordinates>) -> Option<()> {
        self.read_list(|reader| reader.read_empty_or(|reader| reader.read_line(dimension, true)))
    }

    /// Reads `EMPTY` or an opening parenthesis followed by `read`.
    fn read_empty_or(&mut self, read: impl FnOnce(&mut Self) -> Option<()>) -> Option<()> {
        match self.next_token()? {
            Token::Word(word) if word == "EMPTY" => Some(()),
            Token::Open => read(self),
            _ => None,
        }
    }

    /// Reads items separated by commas up to the closing parenthesis.
    fn read_list(&mut self, mut read: impl FnMut(&mut Self) -> Option<()>) -> Option<()> {
        loop {
            read(self)?;
            match self.next_token()? {
                Token::Comma => continue,
                Token::Close => return Some(()),
                _ => return None,
            }
        }
    }

    fn read_line(&mut self, dimension: &mut Option<Ordinates>, is_ring: bool) -> Option<()> {
        let mut points = Vec::new();
        self.read_list(|reader| {
            points.push(reader.read_coordinate(dimension)?);
            Some(())
        })?;
        get_is_line_valid(&points, is_ring).then_some(())
    }

    fn read_coordinate(&mut self, dimension: &mut Option<Ordinates>) -> Option<(f64, f64)> {
        let mut values = Vec::new();
        while let Some(Token::Number(value)) = self.peek_token() {
            values.push(*value);
            self.position += 1;
            if values.len() == dimension.map_or(4, Ordinates::count) {
                break;
            }
        }
        let expected = match *dimension {
            Some(dimension) => dimension,
            None => {
                let ordinates = match values.len() {
                    2 => Ordinates::Xy,
                    3 => Ordinates::Xyz,
                    4 => Ordinates::Xyzm,
                    _ => return None,
                };
                *dimension = Some(ordinates);
                ordinates
            }
        };
        (values.len() == expected.count()).then(|| (values[0], values[1]))
    }

    fn expect(&mut self, token: Token) -> Option<()> {
        (self.next_token()? == token).then_some(())
    }
}

/// The base type and any `Z`/`M`/`ZM` suffix glued to it (`POINTZ`).
fn split_geometry_type(word: &str) -> Option<(&'static str, Option<Ordinates>)> {
    let (base, ordinates) = if let Some(base) = word.strip_suffix("ZM") {
        (base, Some(Ordinates::Xyzm))
    } else if let Some(base) = word.strip_suffix('Z') {
        (base, Some(Ordinates::Xyz))
    } else if let Some(base) = word.strip_suffix('M') {
        (base, Some(Ordinates::Xym))
    } else {
        (word, None)
    };
    let kind = [
        "POINT",
        "LINESTRING",
        "LINEARRING",
        "POLYGON",
        "MULTIPOINT",
        "MULTILINESTRING",
        "MULTIPOLYGON",
        "GEOMETRYCOLLECTION",
    ]
    .into_iter()
    .find(|kind| *kind == base)?;
    Some((kind, ordinates))
}

fn merge_ordinates(left: Ordinates, right: Ordinates) -> Ordinates {
    let has_z = matches!(left, Ordinates::Xyz | Ordinates::Xyzm)
        || matches!(right, Ordinates::Xyz | Ordinates::Xyzm);
    let has_m = matches!(left, Ordinates::Xym | Ordinates::Xyzm)
        || matches!(right, Ordinates::Xym | Ordinates::Xyzm);
    match (has_z, has_m) {
        (true, true) => Ordinates::Xyzm,
        (true, false) => Ordinates::Xyz,
        (false, true) => Ordinates::Xym,
        (false, false) => Ordinates::Xy,
    }
}

/// GEOS rejects a line of one point, and a ring of one or two points or not closed in 2D.
pub(crate) fn get_is_line_valid(points: &[(f64, f64)], is_ring: bool) -> bool {
    if points.is_empty() {
        return true;
    }
    if !is_ring {
        return points.len() >= 2;
    }
    points.len() >= 3 && points.first() == points.last()
}
