use fairspec_library::fairspec_metadata::Report;
use serde::Serialize;

pub const DEFAULT_LIMIT: usize = 100;
pub const MAX_LIMIT: usize = 1000;
pub const MAX_RESPONSE_CHARS: usize = 50_000;

// room for the object a trimmed list is wrapped in
const MAX_ITEMS_CHARS: usize = MAX_RESPONSE_CHARS - 1_000;

/// Cuts a list to `limit` items, then to as many as fit the response budget. Pass one
/// item more than the limit to learn whether the list was truncated.
pub fn limit_items<T: Serialize>(mut items: Vec<T>, limit: usize) -> (Vec<T>, bool) {
    let mut chars = 0;
    let mut count = 0;
    for item in items.iter().take(limit) {
        chars += count_json_chars(item) + 1;
        if chars > MAX_ITEMS_CHARS {
            break;
        }
        count += 1;
    }

    let truncated = count < items.len();
    items.truncate(count);
    (items, truncated)
}

/// Cuts a report's errors to `max_errors`, then to the response budget.
pub fn limit_report(report: Report, max_errors: Option<usize>) -> (Report, bool) {
    let limit = max_errors.unwrap_or(DEFAULT_LIMIT);
    let (errors, truncated) = limit_items(report.errors, limit);
    (
        Report {
            valid: report.valid,
            errors,
        },
        truncated,
    )
}

/// The length Python counts: characters, not bytes, of the compact JSON.
pub fn count_json_chars(value: &impl Serialize) -> usize {
    serde_json::to_string(value)
        .map(|text| text.chars().count())
        .unwrap_or_default()
}
