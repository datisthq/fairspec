use std::cell::Cell;
use std::collections::HashMap;

use super::potential_dialects::PotentialDialect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub field_counts: Vec<usize>,
    cached_modal_field_count: Cell<Option<usize>>,
}

impl Table {
    pub fn new(rows: Vec<Vec<String>>, field_counts: Vec<usize>) -> Self {
        Self {
            rows,
            field_counts,
            cached_modal_field_count: Cell::new(None),
        }
    }

    /// Splits lossily decoded text into rows; empty lines are skipped.
    pub fn parse(data: &[u8], dialect: &PotentialDialect) -> Self {
        let text = String::from_utf8_lossy(data);

        let delimiter = char::from_u32(dialect.delimiter).unwrap_or(',');
        let quote = dialect.quote.char.and_then(char::from_u32);

        let mut rows = Vec::new();
        let mut field_counts = Vec::new();

        for line in text.split(dialect.line_terminator.as_str()) {
            if line.is_empty() {
                continue;
            }

            let fields = match quote {
                Some(quote) => parse_quoted_line(line, delimiter, quote),
                None => line.split(delimiter).map(String::from).collect(),
            };

            field_counts.push(fields.len());
            rows.push(fields);
        }

        Self::new(rows, field_counts)
    }

    /// The most frequent field count; on a tie the smallest count wins.
    pub fn get_modal_field_count(&self) -> usize {
        if let Some(modal) = self.cached_modal_field_count.get() {
            return modal;
        }

        let Some(&max_field_count) = self.field_counts.iter().max() else {
            self.cached_modal_field_count.set(Some(0));
            return 0;
        };

        let modal = if max_field_count <= 256 {
            let mut frequency = vec![0; max_field_count + 1];
            for &count in &self.field_counts {
                frequency[count] += 1;
            }
            let mut max_frequency = 0;
            let mut modal = 0;
            for (count, &count_frequency) in frequency.iter().enumerate() {
                if count_frequency > max_frequency {
                    max_frequency = count_frequency;
                    modal = count;
                }
            }
            modal
        } else {
            // First-seen order breaks ties, as Python's insertion-ordered dict does
            let mut order = Vec::new();
            let mut frequency: HashMap<usize, usize> = HashMap::new();
            for &count in &self.field_counts {
                let entry = frequency.entry(count).or_insert_with(|| {
                    order.push(count);
                    0
                });
                *entry += 1;
            }
            let mut max_frequency = 0;
            let mut modal = 0;
            for count in order {
                if frequency[&count] > max_frequency {
                    max_frequency = frequency[&count];
                    modal = count;
                }
            }
            modal
        };

        self.cached_modal_field_count.set(Some(modal));
        modal
    }

    pub fn is_uniform(&self) -> bool {
        match self.field_counts.first() {
            None => true,
            Some(first) => self.field_counts.iter().all(|count| count == first),
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
}

fn parse_quoted_line(line: &str, delimiter: char, quote: char) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut i = 0;

    while i < chars.len() {
        let char = chars[i];
        if char == quote {
            if in_quotes && chars.get(i + 1) == Some(&quote) {
                current_field.push(quote);
                i += 2;
            } else {
                in_quotes = !in_quotes;
                i += 1;
            }
        } else if char == delimiter && !in_quotes {
            fields.push(std::mem::take(&mut current_field));
            i += 1;
        } else {
            current_field.push(char);
            i += 1;
        }
    }

    fields.push(current_field);
    fields
}
