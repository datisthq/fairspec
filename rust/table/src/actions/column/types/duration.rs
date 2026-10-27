use std::sync::LazyLock;

use chrono::{Datelike, NaiveDate};
use fairspec_metadata::{CellError, Column, DurationColumn, Result};
use regex::{Captures, Regex};

use crate::actions::column::_shared::helpers::inspect_text_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;

pub fn inspect_duration_column(
    column: &DurationColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_text_column(
        &Column::Duration(column.clone()),
        table,
        &parse_duration,
        mapping,
    )
}

const MAX_TIMEDELTA_DAYS: f64 = 999_999_999.0;

static PERIOD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?P<sign>[+-])?P",
        r"(?P<years>[0-9]+(?:[,.][0-9]+)?Y)?",
        r"(?P<months>[0-9]+(?:[,.][0-9]+)?M)?",
        r"(?P<weeks>[0-9]+(?:[,.][0-9]+)?W)?",
        r"(?P<days>[0-9]+(?:[,.][0-9]+)?D)?",
        r"(?:T(?P<hours>[0-9]+(?:[,.][0-9]+)?H)?",
        r"(?P<minutes>[0-9]+(?:[,.][0-9]+)?M)?",
        r"(?P<seconds>[0-9]+(?:[,.][0-9]+)?S)?)?$",
    ))
    .expect("valid regex")
});

static DATE_REGEXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"^(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})(?P<month>[0-9]{2})(?P<day>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})-W(?P<week>[0-9]{2})-(?P<day>[0-9])$",
        r"^(?P<year>[0-9]{4})W(?P<week>[0-9]{2})(?P<day>[0-9])$",
        r"^(?P<year>[0-9]{4})-(?P<day>[0-9]{3})$",
        r"^(?P<year>[0-9]{4})(?P<day>[0-9]{3})$",
        r"^(?P<year>[0-9]{4})-W(?P<week>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})W(?P<week>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})-(?P<month>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})(?P<month>[0-9]{2})$",
        r"^(?P<year>[0-9]{4})$",
        r"^(?P<century>[0-9]{2})$",
    ]
    .iter()
    .map(|pattern| Regex::new(pattern).expect("valid regex"))
    .collect()
});

static TIME_REGEXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let time_zone = r"(?:Z|[+-][0-9]{2}(?::?[0-9]{2})?)?$";
    [
        r"^T?(?P<hour>[0-9]{2}):(?P<minute>[0-9]{2}):(?P<second>[0-9]{2}(?:[,.][0-9]+)?)",
        r"^T?(?P<hour>[0-9]{2})(?P<minute>[0-9]{2})(?P<second>[0-9]{2}(?:[,.][0-9]+)?)",
        r"^T?(?P<hour>[0-9]{2}):(?P<minute>[0-9]{2}(?:[,.][0-9]+)?)",
        r"^T?(?P<hour>[0-9]{2})(?P<minute>[0-9]{2}(?:[,.][0-9]+)?)",
        r"^T?(?P<hour>[0-9]{2}(?:[,.][0-9]+)?)",
    ]
    .iter()
    .map(|pattern| Regex::new(&format!("{pattern}{time_zone}")).expect("valid regex"))
    .collect()
});

/// Whether isodate's `parse_duration` accepts the text without raising.
fn parse_duration(text: &str) -> bool {
    // Python's `$` also matches before a final newline
    let period = text.strip_suffix('\n').unwrap_or(text);
    // isodate reads `PT` and `P1DT`, though ISO 8601 needs a component after the `T`
    if period.ends_with('T') {
        return false;
    }
    if let Some(captures) = PERIOD_REGEX.captures(period) {
        // isodate's `P(?!\b)` needs a word character right after the `P`
        let designator = captures.name("sign").map_or(0, |sign| sign.len());
        if period.len() > designator + 1 {
            return get_is_timedelta_valid(&captures);
        }
    }
    match text.strip_prefix('P') {
        Some(datetime) => parse_alternative_duration(datetime),
        None => false,
    }
}

fn get_is_timedelta_valid(captures: &Captures) -> bool {
    let read = |name: &str| {
        captures.name(name).map_or(Some(0.0), |value| {
            let text = &value.as_str()[..value.len() - 1];
            text.replace(',', ".").parse::<f64>().ok()
        })
    };
    let (Some(weeks), Some(days), Some(hours), Some(minutes), Some(seconds)) = (
        read("weeks"),
        read("days"),
        read("hours"),
        read("minutes"),
        read("seconds"),
    ) else {
        return false;
    };

    let total_days = weeks * 7.0 + days + hours / 24.0 + minutes / 1440.0 + seconds / 86400.0;
    if !total_days.is_finite() {
        return false;
    }
    let is_negative = captures.name("sign").map(|sign| sign.as_str()) == Some("-");
    let days = if is_negative {
        total_days.ceil()
    } else {
        total_days.floor()
    };
    days <= MAX_TIMEDELTA_DAYS
}

fn parse_alternative_duration(datetime: &str) -> bool {
    let mut parts = datetime.split('T');
    let (Some(date), Some(time), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    parse_date(date) && parse_time(time)
}

fn parse_date(text: &str) -> bool {
    let Some(captures) = DATE_REGEXES.iter().find_map(|regex| regex.captures(text)) else {
        return false;
    };
    let read = |name: &str| {
        captures
            .name(name)
            .map(|value| value.as_str().parse::<i64>())
    };
    let year = match read("century") {
        Some(Ok(century)) => return create_date(century * 100 + 1, 1, 1).is_some(),
        _ => match read("year") {
            Some(Ok(year)) => year,
            _ => return false,
        },
    };

    if let Some(Ok(month)) = read("month") {
        let day = match read("day") {
            Some(Ok(day)) => day,
            _ => 1,
        };
        let month = if month == 0 { 1 } else { month };
        return create_date(year, month, day).is_some();
    }

    let Some(first) = create_date(year, 1, 1) else {
        return false;
    };
    let offset = match (read("week"), read("day")) {
        (Some(Ok(week)), day) => {
            let iso_week = i64::from(first.iso_week().week());
            let iso_weekday = i64::from(first.weekday().number_from_monday());
            let day = match day {
                Some(Ok(day)) => day,
                _ => 1,
            };
            (week - i64::from(iso_week == 1)) * 7 - iso_weekday + day
        }
        (None, Some(Ok(day))) => day - 1,
        _ => 0,
    };
    first
        .checked_add_signed(chrono::Duration::days(offset))
        .is_some_and(|date| (1..=9999).contains(&date.year()))
}

fn create_date(year: i64, month: i64, day: i64) -> Option<NaiveDate> {
    if !(1..=9999).contains(&year) {
        return None;
    }
    NaiveDate::from_ymd_opt(
        year as i32,
        u32::try_from(month).ok()?,
        u32::try_from(day).ok()?,
    )
}

fn parse_time(text: &str) -> bool {
    let Some(captures) = TIME_REGEXES.iter().find_map(|regex| regex.captures(text)) else {
        return false;
    };
    let read = |name: &str| {
        captures.name(name).map_or(0, |value| {
            let integer = value.as_str().split(['.', ',']).next().unwrap_or_default();
            integer.parse::<u64>().unwrap_or(u64::MAX)
        })
    };
    read("hour") < 24 && read("minute") < 60 && read("second") < 60
}
