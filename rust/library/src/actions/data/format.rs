use std::sync::LazyLock;

use fairspec_metadata::FormatCheck;
use fairspec_table::{DURATION_REGEX, EMAIL_REGEX};
use regex::Regex;

static DATE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap());
static TIME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^([0-9]{2}):([0-9]{2}):([0-9]{2})(?:\.[0-9]+)?(?:[Zz]|([+-])([0-9]{2}):([0-9]{2}))$",
    )
    .unwrap()
});
const DEC_OCTET: &str = "(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])";
static IPV4_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^{DEC_OCTET}(?:\.{DEC_OCTET}){{3}}$")).unwrap());
static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}$")
        .unwrap()
});
static EMAIL_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(&EMAIL_REGEX).unwrap());
static DURATION_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(&DURATION_REGEX).unwrap());

/// The `format` values a Data Schema asserts; any other format is an annotation.
pub static DATA_SCHEMA_FORMATS: [FormatCheck; 7] = [
    ("date", get_is_date),
    ("time", get_is_time),
    ("date-time", get_is_date_time),
    ("duration", |value| DURATION_PATTERN.is_match(value)),
    ("email", |value| EMAIL_PATTERN.is_match(value)),
    ("ipv4", |value| IPV4_REGEX.is_match(value)),
    ("uuid", |value| UUID_REGEX.is_match(value)),
];

fn get_is_date(value: &str) -> bool {
    let Some(captures) = DATE_REGEX.captures(value) else {
        return false;
    };
    let number = |index: usize| captures[index].parse::<u32>().unwrap_or(0);
    let (year, month, day) = (number(1), number(2), number(3));

    let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    (1..=12).contains(&month) && day >= 1 && day <= days[month as usize - 1]
}

// RFC 3339 allows a leap second only at the last minute of a UTC day
fn get_is_time(value: &str) -> bool {
    let Some(captures) = TIME_REGEX.captures(value) else {
        return false;
    };
    let number = |index: usize| {
        captures
            .get(index)
            .map_or(0, |found| found.as_str().parse::<i64>().unwrap_or(0))
    };
    let (hour, minute, second) = (number(1), number(2), number(3));
    let sign = if captures.get(4).map(|found| found.as_str()) == Some("-") {
        -1
    } else {
        1
    };
    let (offset_hour, offset_minute) = (number(5), number(6));
    if hour > 23 || minute > 59 || second > 60 || offset_hour > 23 || offset_minute > 59 {
        return false;
    }
    if second < 60 {
        return true;
    }

    let utc_minutes = hour * 60 + minute - sign * (offset_hour * 60 + offset_minute);
    utc_minutes.rem_euclid(1440) == 23 * 60 + 59
}

fn get_is_date_time(value: &str) -> bool {
    matches!(value.get(10..11), Some("T" | "t"))
        && value.get(..10).is_some_and(get_is_date)
        && value.get(11..).is_some_and(get_is_time)
}
