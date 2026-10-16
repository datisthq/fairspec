from __future__ import annotations

from ..uniformity import calculate_tau0, calculate_tau1


class TestCalculateTau0:
    def test_uniform_field_counts(self):
        assert calculate_tau0([3, 3, 3, 3, 3]) == 1

    def test_varying_field_counts(self):
        tau0 = calculate_tau0([3, 4, 3, 5, 3])
        assert tau0 < 1
        assert tau0 > 0

    def test_empty_array(self):
        assert calculate_tau0([]) == 0

    def test_penalize_high_variance(self):
        tau0_low = calculate_tau0([3, 3, 3, 4, 3])
        tau0_high = calculate_tau0([1, 5, 2, 8, 3])
        assert tau0_low > tau0_high


class TestCalculateTau1:
    def test_consistent_field_counts(self):
        tau1 = calculate_tau1([3, 3, 3, 3, 3], 3)
        assert tau1 > 0.9

    def test_varying_field_counts(self):
        tau1 = calculate_tau1([3, 4, 5, 3, 4], 3)
        assert tau1 < 1
        assert tau1 > 0

    def test_empty_array(self):
        assert calculate_tau1([], 0) == 0

    def test_penalize_frequent_transitions(self):
        tau1_few = calculate_tau1([3, 3, 3, 4, 4, 4], 3)
        tau1_many = calculate_tau1([3, 4, 3, 4, 3, 4], 3)
        assert tau1_few > tau1_many

    def test_favor_higher_mode_dominance(self):
        tau1_high = calculate_tau1([3, 3, 3, 3, 4], 3)
        tau1_low = calculate_tau1([3, 3, 4, 4, 5], 3)
        assert tau1_high > tau1_low

    def test_penalize_wide_range(self):
        tau1_narrow = calculate_tau1([3, 3, 4, 4, 3], 3)
        tau1_wide = calculate_tau1([1, 3, 7, 3, 2], 3)
        assert tau1_narrow > tau1_wide


class TestCombinedTau:
    def test_uniform_data_scores_highly(self):
        field_counts = [5, 5, 5, 5, 5]
        tau0 = calculate_tau0(field_counts)
        tau1 = calculate_tau1(field_counts, 5)
        assert tau0 > 0.9
        assert tau1 > 0.9

    def test_chaotic_data_scores_lowly(self):
        field_counts = [1, 5, 2, 8, 3, 9, 1, 7]
        tau0 = calculate_tau0(field_counts)
        tau1 = calculate_tau1(field_counts, 1)
        assert tau0 < 0.5
        assert tau1 < 0.5
