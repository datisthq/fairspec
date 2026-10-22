use crate::utils::sniffer::uniformity::{calculate_tau0, calculate_tau1};

#[test]
fn test_tau0_uniform_field_counts() {
    assert_eq!(calculate_tau0(&[3, 3, 3, 3, 3]), 1.0);
}

#[test]
fn test_tau0_varying_field_counts() {
    let tau0 = calculate_tau0(&[3, 4, 3, 5, 3]);
    assert!(tau0 < 1.0 && tau0 > 0.0);
}

#[test]
fn test_tau0_empty_array() {
    assert_eq!(calculate_tau0(&[]), 0.0);
}

#[test]
fn test_tau0_penalize_high_variance() {
    assert!(calculate_tau0(&[3, 3, 3, 4, 3]) > calculate_tau0(&[1, 5, 2, 8, 3]));
}

#[test]
fn test_tau1_consistent_field_counts() {
    assert!(calculate_tau1(&[3, 3, 3, 3, 3], 3) > 0.9);
}

#[test]
fn test_tau1_varying_field_counts() {
    let tau1 = calculate_tau1(&[3, 4, 5, 3, 4], 3);
    assert!(tau1 < 1.0 && tau1 > 0.0);
}

#[test]
fn test_tau1_empty_array() {
    assert_eq!(calculate_tau1(&[], 0), 0.0);
}

#[test]
fn test_tau1_penalize_frequent_transitions() {
    assert!(calculate_tau1(&[3, 3, 3, 4, 4, 4], 3) > calculate_tau1(&[3, 4, 3, 4, 3, 4], 3));
}

#[test]
fn test_tau1_favor_higher_mode_dominance() {
    assert!(calculate_tau1(&[3, 3, 3, 3, 4], 3) > calculate_tau1(&[3, 3, 4, 4, 5], 3));
}

#[test]
fn test_tau1_penalize_wide_range() {
    assert!(calculate_tau1(&[3, 3, 4, 4, 3], 3) > calculate_tau1(&[1, 3, 7, 3, 2], 3));
}

#[test]
fn test_uniform_data_scores_highly() {
    let field_counts = [5, 5, 5, 5, 5];
    assert!(calculate_tau0(&field_counts) > 0.9);
    assert!(calculate_tau1(&field_counts, 5) > 0.9);
}

#[test]
fn test_chaotic_data_scores_lowly() {
    let field_counts = [1, 5, 2, 8, 3, 9, 1, 7];
    assert!(calculate_tau0(&field_counts) < 0.5);
    assert!(calculate_tau1(&field_counts, 1) < 0.5);
}
