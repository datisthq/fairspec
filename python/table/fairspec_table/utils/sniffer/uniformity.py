from __future__ import annotations

import math


def calculate_tau0(field_counts: list[int]) -> float:
    if len(field_counts) == 0:
        return 0

    std_dev = _calculate_std_dev(field_counts)
    return 1 / (1 + 2 * std_dev)


def calculate_tau1(
    field_counts: list[int],
    modal_count: int,
) -> float:
    if len(field_counts) == 0:
        return 0

    range_score = _calculate_range_score(field_counts, modal_count)
    transition_score = _calculate_transition_score(field_counts)
    mode_score = _calculate_mode_score(field_counts, modal_count)

    return (range_score + transition_score + mode_score) / 3


def _calculate_std_dev(counts: list[int]) -> float:
    if len(counts) == 0:
        return 0

    mean = sum(counts) / len(counts)
    variance = sum((count - mean) ** 2 for count in counts) / len(counts)

    return math.sqrt(variance)


def _calculate_range_score(counts: list[int], modal_count: int) -> float:
    if len(counts) == 0:
        return 0

    min_val = min(counts)
    max_val = max(counts)
    range_val = max_val - min_val

    if range_val == 0:
        return 1
    if modal_count <= 0:
        return 0

    normalized_range = range_val / modal_count
    return 1 / (1 + normalized_range)


def _calculate_transition_score(counts: list[int]) -> float:
    if len(counts) <= 1:
        return 1

    transitions = 0
    for i in range(1, len(counts)):
        if counts[i] != counts[i - 1]:
            transitions += 1

    max_transitions = len(counts) - 1
    transition_rate = transitions / max_transitions

    return 1 - transition_rate


def _calculate_mode_score(counts: list[int], modal_count: int) -> float:
    if len(counts) == 0:
        return 0

    modal_frequency = sum(1 for count in counts if count == modal_count)
    mode_ratio = modal_frequency / len(counts)

    return mode_ratio
