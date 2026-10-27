use fairspec_metadata::Result;
use polars::sql::SQLContext;

use crate::helpers::error::PolarsResultExt;
use crate::models::table::Table;

/// Runs a SQL query against a table, exposed as `self`.
pub fn query_table(table: Table, query: &str) -> Result<Table> {
    query_tables([("self".to_string(), table)], query)
}

/// Runs a SQL query against several tables, each exposed under its name.
pub fn query_tables(
    tables: impl IntoIterator<Item = (String, Table)>,
    query: &str,
) -> Result<Table> {
    let mut context = SQLContext::new();
    for (name, table) in tables {
        context.register(&name, table);
    }
    context.execute(&set_nulls_last(query)).into_result()
}

// A keyword that ends an ORDER BY list at the depth it opened
const ORDER_END_KEYWORDS: [&str; 9] = [
    "LIMIT",
    "OFFSET",
    "FETCH",
    "UNION",
    "EXCEPT",
    "INTERSECT",
    "ROWS",
    "RANGE",
    "GROUPS",
];

/// Adds `NULLS LAST` to every ORDER BY item that names no null order.
///
/// Polars sorts nulls first in descending order, while the spec places them last
/// in both directions.
pub fn set_nulls_last(query: &str) -> String {
    let chars: Vec<char> = query.chars().collect();
    let size = chars.len();
    let mut positions: Vec<usize> = Vec::new();
    let mut frames = vec![OrderFrame::default()];
    let mut index = 0;

    while index < size {
        let char = chars[index];
        let starts_with = |prefix: &str| {
            prefix
                .chars()
                .enumerate()
                .all(|(offset, expected)| chars.get(index + offset) == Some(&expected))
        };

        if matches!(char, '\'' | '"' | '`') {
            index = skip_quoted(&chars, index);
            if let Some(frame) = frames.last_mut() {
                frame.mark(index, None);
            }
        } else if starts_with("--") {
            index = find_from(&chars, index, &['\n']).unwrap_or(size);
        } else if starts_with("/*") {
            index = find_from(&chars, index + 2, &['*', '/']).map_or(size, |end| end + 2);
        } else if char.is_whitespace() {
            index += 1;
        } else if char == '(' {
            frames.push(OrderFrame::default());
            index += 1;
        } else if char == ')' {
            if frames.len() > 1
                && let Some(mut closed) = frames.pop()
            {
                closed.finish_item(&mut positions);
                if let Some(parent) = frames.last_mut() {
                    parent.mark(index + 1, None);
                }
            }
            index += 1;
        } else if char == ',' || char == ';' {
            if let Some(frame) = frames.last_mut() {
                frame.finish_item(&mut positions);
                if char == ';' {
                    frame.is_ordering = false;
                }
                frame.last_word = None;
            }
            index += 1;
        } else if is_word_char(char) {
            let mut end = index;
            while end < size && (is_word_char(chars[end]) || chars[end] == '$') {
                end += 1;
            }
            let word: String = chars[index..end].iter().collect::<String>().to_uppercase();
            if let Some(frame) = frames.last_mut() {
                if frame.is_ordering && ORDER_END_KEYWORDS.contains(&word.as_str()) {
                    frame.finish_item(&mut positions);
                    frame.is_ordering = false;
                }
                if word == "BY" && frame.last_word.as_deref() == Some("ORDER") {
                    frame.is_ordering = true;
                    frame.item_end = None;
                    frame.has_nulls = false;
                } else {
                    frame.has_nulls = frame.has_nulls || word == "NULLS";
                    frame.mark(end, Some(word));
                }
            }
            index = end;
        } else {
            if let Some(frame) = frames.last_mut() {
                frame.mark(index + 1, None);
            }
            index += 1;
        }
    }

    for frame in &mut frames {
        frame.finish_item(&mut positions);
    }

    positions.sort_unstable();
    let mut result = String::with_capacity(query.len() + positions.len() * 11);
    let mut positions = positions.into_iter().peekable();
    for (index, char) in chars.iter().enumerate() {
        while positions.next_if_eq(&index).is_some() {
            result.push_str(" NULLS LAST");
        }
        result.push(*char);
    }
    for _ in positions {
        result.push_str(" NULLS LAST");
    }
    result
}

#[derive(Default)]
struct OrderFrame {
    is_ordering: bool,
    has_nulls: bool,
    item_end: Option<usize>,
    last_word: Option<String>,
}

impl OrderFrame {
    fn mark(&mut self, end: usize, word: Option<String>) {
        if self.is_ordering {
            self.item_end = Some(end);
        }
        self.last_word = word;
    }

    fn finish_item(&mut self, positions: &mut Vec<usize>) {
        if self.is_ordering
            && !self.has_nulls
            && let Some(end) = self.item_end
        {
            positions.push(end);
        }
        self.item_end = None;
        self.has_nulls = false;
    }
}

fn is_word_char(char: char) -> bool {
    char.is_alphanumeric() || char == '_'
}

fn find_from(chars: &[char], start: usize, needle: &[char]) -> Option<usize> {
    (start..chars.len()).find(|&index| chars[index..].starts_with(needle))
}

fn skip_quoted(chars: &[char], mut index: usize) -> usize {
    let quote = chars[index];
    index += 1;
    while index < chars.len() {
        if chars[index] == quote {
            if chars.get(index + 1) == Some(&quote) {
                index += 2;
                continue;
            }
            return index + 1;
        }
        index += 1;
    }
    index
}
