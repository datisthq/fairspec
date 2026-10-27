use fairspec_metadata::{CellError, Column, Result, WkbColumn};

use super::wkt::get_is_line_valid;
use crate::actions::column::_shared::helpers::inspect_text_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;

pub fn inspect_wkb_column(
    column: &WkbColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_text_column(&Column::Wkb(column.clone()), table, &parse_wkb, mapping)
}

/// Whether GEOS (shapely's `from_wkb(bytes.fromhex(text))`) reads a linear geometry;
/// bytes after the geometry are ignored, as GEOS does. A byte order other than 0 or 1,
/// which GEOS reads as the machine's, is refused.
fn parse_wkb(text: &str) -> bool {
    let Some(bytes) = decode_hex(text) else {
        return false;
    };
    let mut reader = WkbReader {
        bytes: &bytes,
        position: 0,
    };
    reader.read_geometry(None).is_some()
}

/// Python's `bytes.fromhex`: ASCII whitespace may separate the digit pairs.
fn decode_hex(text: &str) -> Option<Vec<u8>> {
    let mut bytes = Vec::with_capacity(text.len() / 2);
    let mut chars = text.chars();
    loop {
        let high = loop {
            match chars.next() {
                None => return Some(bytes),
                Some(' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}') => {}
                Some(char) => break char.to_digit(16)?,
            }
        };
        let low = chars.next()?.to_digit(16)?;
        bytes.push((high * 16 + low) as u8);
    }
}

struct WkbReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl WkbReader<'_> {
    fn read_geometry(&mut self, expected_type: Option<u32>) -> Option<()> {
        let is_little_endian = match self.read_byte()? {
            0 => false,
            1 => true,
            _ => return None,
        };
        let type_int = self.read_u32(is_little_endian)?;
        let geometry_type = (type_int & 0xffff) % 1000;
        let iso_type_range = (type_int & 0xffff) / 1000;
        let has_z = type_int & 0x8000_0000 != 0 || matches!(iso_type_range, 1 | 3);
        let has_m = type_int & 0x4000_0000 != 0 || matches!(iso_type_range, 2 | 3);
        let dimension = 2 + usize::from(has_z) + usize::from(has_m);
        if type_int & 0x2000_0000 != 0 {
            self.read_u32(is_little_endian)?;
        }
        if expected_type.is_some_and(|expected_type| expected_type != geometry_type) {
            return None;
        }

        match geometry_type {
            1 => self.read_points(1, dimension, is_little_endian).map(|_| ()),
            2 => self.read_line(dimension, is_little_endian, false),
            3 => self.read_polygon(dimension, is_little_endian),
            4..=7 => {
                let count = self.read_u32(is_little_endian)?;
                let member_type = (geometry_type != 7).then(|| geometry_type - 3);
                for _ in 0..count {
                    self.read_geometry(member_type)?;
                }
                Some(())
            }
            _ => None,
        }
    }

    fn read_polygon(&mut self, dimension: usize, is_little_endian: bool) -> Option<()> {
        let count = self.read_u32(is_little_endian)?;
        for _ in 0..count {
            self.read_line(dimension, is_little_endian, true)?;
        }
        Some(())
    }

    fn read_line(&mut self, dimension: usize, is_little_endian: bool, is_ring: bool) -> Option<()> {
        let count = self.read_u32(is_little_endian)? as usize;
        let points = self.read_points(count, dimension, is_little_endian)?;
        get_is_line_valid(&points, is_ring).then_some(())
    }

    fn read_points(
        &mut self,
        count: usize,
        dimension: usize,
        is_little_endian: bool,
    ) -> Option<Vec<(f64, f64)>> {
        let size = count.checked_mul(dimension * 8)?;
        if size > self.bytes.len() - self.position {
            return None;
        }
        let mut points = Vec::with_capacity(count);
        for _ in 0..count {
            let x = self.read_f64(is_little_endian)?;
            let y = self.read_f64(is_little_endian)?;
            for _ in 2..dimension {
                self.read_f64(is_little_endian)?;
            }
            points.push((x, y));
        }
        Some(points)
    }

    fn read_byte(&mut self) -> Option<u8> {
        let byte = *self.bytes.get(self.position)?;
        self.position += 1;
        Some(byte)
    }

    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        let bytes = self.bytes.get(self.position..self.position + N)?;
        self.position += N;
        bytes.try_into().ok()
    }

    fn read_u32(&mut self, is_little_endian: bool) -> Option<u32> {
        let bytes = self.read_array()?;
        Some(if is_little_endian {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        })
    }

    fn read_f64(&mut self, is_little_endian: bool) -> Option<f64> {
        let bytes = self.read_array()?;
        Some(if is_little_endian {
            f64::from_le_bytes(bytes)
        } else {
            f64::from_be_bytes(bytes)
        })
    }
}
