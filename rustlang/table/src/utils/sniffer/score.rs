use super::error::SnifferError;
use super::metadata::Quote;
use super::potential_dialects::PotentialDialect;
use super::table::Table;
use super::uniformity::{calculate_tau0, calculate_tau1};

const COMMON_DELIMITERS: [u32; 4] = [44, 9, 59, 124];

#[derive(Debug, Clone, PartialEq)]
pub struct DialectScore {
    pub dialect: PotentialDialect,
    pub gamma: f64,
    pub tau0: f64,
    pub tau1: f64,
    pub num_fields: usize,
    pub is_uniform: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindBestDialectPreferences {
    pub prefer_common_delimiters: bool,
    pub prefer_double_quote: bool,
}

pub fn score_dialect(data: &[u8], dialect: &PotentialDialect) -> DialectScore {
    let table = Table::parse(data, dialect);

    let tau0 = calculate_tau0(&table.field_counts);
    let tau1 = calculate_tau1(&table.field_counts, table.get_modal_field_count());
    let num_fields = table.get_modal_field_count();
    let is_uniform = table.is_uniform();
    let num_rows = table.num_rows();

    let quote_evidence = analyze_quote_evidence(data, dialect);

    let gamma = calculate_gamma(
        tau0,
        tau1,
        num_rows,
        num_fields,
        is_uniform,
        dialect.delimiter,
        dialect.quote,
        &quote_evidence,
    );

    DialectScore {
        dialect: *dialect,
        gamma,
        tau0,
        tau1,
        num_fields,
        is_uniform,
    }
}

/// The highest gamma wins; an equal gamma keeps the earlier score.
pub fn find_best_dialect<'a>(
    scores: &'a [DialectScore],
    preferences: &FindBestDialectPreferences,
) -> Result<&'a DialectScore, SnifferError> {
    let mut best_score = scores
        .first()
        .ok_or_else(|| SnifferError::Sniffer("No dialect scores provided".to_string()))?;

    for score in scores {
        let mut current_gamma = score.gamma;
        let mut best_gamma = best_score.gamma;

        if preferences.prefer_common_delimiters {
            if COMMON_DELIMITERS.contains(&score.dialect.delimiter) {
                current_gamma += 0.05;
            }
            if COMMON_DELIMITERS.contains(&best_score.dialect.delimiter) {
                best_gamma += 0.05;
            }
        }

        if preferences.prefer_double_quote {
            if score.dialect.quote.char == Some(34) {
                current_gamma += 0.05;
            }
            if best_score.dialect.quote.char == Some(34) {
                best_gamma += 0.05;
            }
        }

        if current_gamma > best_gamma {
            best_score = score;
        }
    }

    Ok(best_score)
}

struct QuoteEvidence {
    quote_density: f64,
    boundary_matches: usize,
    internal_matches: usize,
}

#[allow(clippy::too_many_arguments)]
fn calculate_gamma(
    tau0: f64,
    tau1: f64,
    num_rows: usize,
    num_fields: usize,
    is_uniform: bool,
    delimiter: u32,
    quote: Quote,
    quote_evidence: &QuoteEvidence,
) -> f64 {
    let mut gamma = tau0 * 0.4 + tau1 * 0.6;

    if is_uniform {
        gamma += 0.2;
    }

    if (2..=50).contains(&num_fields) {
        gamma += 0.3;
    } else if num_fields == 1 {
        gamma -= 1.0;
    }

    if num_rows >= 2 {
        gamma += 0.05;
    }

    if COMMON_DELIMITERS.contains(&delimiter) {
        gamma += 0.15;
    }

    match quote.char {
        Some(char) => {
            let quote_score = quote_evidence.boundary_matches as f64 * 0.5
                + quote_evidence.quote_density * 0.3
                - quote_evidence.internal_matches as f64 * 0.2;
            gamma += quote_score.clamp(0.0, 0.2);

            if char == 34 {
                gamma += 0.05;
            }
        }
        None => {
            if quote_evidence.quote_density < 0.01 {
                gamma += 0.1;
            }
        }
    }

    gamma.clamp(0.0, 2.0)
}

fn analyze_quote_evidence(data: &[u8], dialect: &PotentialDialect) -> QuoteEvidence {
    let Some(quote_char) = dialect.quote.char else {
        return QuoteEvidence {
            quote_density: 0.0,
            boundary_matches: 0,
            internal_matches: 0,
        };
    };
    let delimiter = dialect.delimiter;
    let get_is_boundary = |byte: Option<&u8>| match byte {
        None => true,
        Some(&byte) => byte as u32 == delimiter || byte == 10 || byte == 13,
    };

    let mut quote_count = 0;
    let mut boundary_matches = 0;
    let mut internal_matches = 0;

    for (i, &byte) in data.iter().enumerate() {
        if byte as u32 != quote_char {
            continue;
        }
        quote_count += 1;

        let previous = if i > 0 { data.get(i - 1) } else { None };
        if get_is_boundary(previous) || get_is_boundary(data.get(i + 1)) {
            boundary_matches += 1;
        } else {
            internal_matches += 1;
        }
    }

    let quote_density = if data.is_empty() {
        0.0
    } else {
        quote_count as f64 / data.len() as f64
    };

    QuoteEvidence {
        quote_density,
        boundary_matches,
        internal_matches,
    }
}
