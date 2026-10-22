use crate::utils::sniffer::metadata::{LineTerminator, Quote};
use crate::utils::sniffer::potential_dialects::PotentialDialect;
use crate::utils::sniffer::score::{
    DialectScore, FindBestDialectPreferences, find_best_dialect, score_dialect,
};

fn create_dialect(delimiter: u32, quote: Option<u32>) -> PotentialDialect {
    PotentialDialect {
        delimiter,
        quote: Quote { char: quote },
        line_terminator: LineTerminator::Lf,
    }
}

fn create_preferences(common: bool, double_quote: bool) -> FindBestDialectPreferences {
    FindBestDialectPreferences {
        prefer_common_delimiters: common,
        prefer_double_quote: double_quote,
    }
}

#[test]
fn test_comma_delimited_csv_scores_highly() {
    let score = score_dialect(b"a,b,c\n1,2,3\n4,5,6", &create_dialect(44, None));
    assert!(score.gamma > 0.5);
    assert!(score.is_uniform);
    assert_eq!(score.num_fields, 3);
}

#[test]
fn test_incorrect_delimiter_scores_poorly() {
    let score = score_dialect(b"a,b,c\n1,2,3\n4,5,6", &create_dialect(9, None));
    assert!(score.gamma < 0.5);
    assert_eq!(score.num_fields, 1);
}

#[test]
fn test_detect_non_uniform() {
    let score = score_dialect(b"a,b\n1,2\n3,4,5", &create_dialect(44, None));
    assert!(!score.is_uniform);
}

#[test]
fn test_calculates_tau0_and_tau1() {
    let score = score_dialect(b"a,b,c\n1,2,3\n4,5,6", &create_dialect(44, None));
    assert!(score.tau0 > 0.0);
    assert!(score.tau1 > 0.0);
}

#[test]
fn test_handle_quoted_fields() {
    let score = score_dialect(
        b"\"a\",\"b,c\",\"d\"\n\"1\",\"2,3\",\"4\"",
        &create_dialect(44, Some(34)),
    );
    assert_eq!(score.num_fields, 3);
    assert!(score.is_uniform);
}

#[test]
fn test_select_highest_scoring() {
    let data = b"a,b,c\n1,2,3\n4,5,6";
    let scores: Vec<_> = [44, 9, 59]
        .into_iter()
        .map(|delimiter| score_dialect(data, &create_dialect(delimiter, None)))
        .collect();
    let best = find_best_dialect(&scores, &create_preferences(true, true)).unwrap();
    assert_eq!(best.dialect.delimiter, 44);
}

#[test]
fn test_prefer_common_delimiters() {
    let data = b"a,b\n1,2\n3,4";
    let mut scores: Vec<_> = [44, 94]
        .into_iter()
        .map(|delimiter| score_dialect(data, &create_dialect(delimiter, None)))
        .collect();
    scores[0].gamma = 0.7;
    scores[1].gamma = 0.71;

    let best = find_best_dialect(&scores, &create_preferences(true, false)).unwrap();
    assert_eq!(best.dialect.delimiter, 44);
}

#[test]
fn test_prefer_double_quote() {
    let data = b"\"a\",\"b\"\n\"1\",\"2\"";
    let scores: Vec<_> = [34, 39]
        .into_iter()
        .map(|quote| score_dialect(data, &create_dialect(44, Some(quote))))
        .collect();

    let best = find_best_dialect(&scores, &create_preferences(false, true)).unwrap();
    if best.dialect.quote.char.is_some() {
        assert_eq!(best.dialect.quote.char, Some(34));
    }
}

#[test]
fn test_empty_scores_throws() {
    let scores: Vec<DialectScore> = vec![];
    assert!(find_best_dialect(&scores, &create_preferences(true, true)).is_err());
}

#[test]
fn test_single_score() {
    let scores = vec![score_dialect(b"a,b\n1,2", &create_dialect(44, None))];
    let best = find_best_dialect(&scores, &create_preferences(true, true)).unwrap();
    assert!(std::ptr::eq(best, &scores[0]));
}

#[test]
fn test_reward_uniform_tables() {
    let dialect = create_dialect(44, None);
    let uniform = score_dialect(b"a,b,c\n1,2,3\n4,5,6\n7,8,9", &dialect);
    let non_uniform = score_dialect(b"a,b\n1,2,3\n4,5\n6,7,8,9", &dialect);
    assert!(uniform.gamma > non_uniform.gamma);
}

#[test]
fn test_reward_reasonable_field_counts() {
    let dialect = create_dialect(44, None);
    let csv_100 = format!(
        "a,{}z\n1,{}3\n4,{}6",
        "b,".repeat(98),
        "2,".repeat(98),
        "5,".repeat(98)
    );
    let score_2 = score_dialect(b"a,b\n1,2\n3,4", &dialect);
    let score_100 = score_dialect(csv_100.as_bytes(), &dialect);
    assert_eq!(score_2.num_fields, 2);
    assert!(score_100.num_fields > 50);
}

#[test]
fn test_cap_gamma_at_2() {
    let score = score_dialect(
        b"a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12",
        &create_dialect(44, None),
    );
    assert!(score.gamma <= 2.0);
}

#[test]
fn test_gamma_non_negative() {
    let score = score_dialect(b"random text without structure", &create_dialect(44, None));
    assert!(score.gamma >= 0.0);
}
