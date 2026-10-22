pub fn calculate_tau0(field_counts: &[usize]) -> f64 {
    if field_counts.is_empty() {
        return 0.0;
    }

    let std_dev = calculate_std_dev(field_counts);
    1.0 / (1.0 + 2.0 * std_dev)
}

pub fn calculate_tau1(field_counts: &[usize], modal_count: usize) -> f64 {
    if field_counts.is_empty() {
        return 0.0;
    }

    let range_score = calculate_range_score(field_counts, modal_count);
    let transition_score = calculate_transition_score(field_counts);
    let mode_score = calculate_mode_score(field_counts, modal_count);

    (range_score + transition_score + mode_score) / 3.0
}

fn calculate_std_dev(counts: &[usize]) -> f64 {
    let length = counts.len() as f64;
    let mean = counts.iter().sum::<usize>() as f64 / length;
    let variance = sum_python_floats(counts.iter().map(|&count| {
        let delta = count as f64 - mean;
        delta * delta
    })) / length;

    variance.sqrt()
}

fn calculate_range_score(counts: &[usize], modal_count: usize) -> f64 {
    let min = counts.iter().min().copied().unwrap_or_default();
    let max = counts.iter().max().copied().unwrap_or_default();
    let range = max - min;

    if range == 0 {
        return 1.0;
    }
    if modal_count == 0 {
        return 0.0;
    }

    let normalized_range = range as f64 / modal_count as f64;
    1.0 / (1.0 + normalized_range)
}

fn calculate_transition_score(counts: &[usize]) -> f64 {
    if counts.len() <= 1 {
        return 1.0;
    }

    let transitions = counts.windows(2).filter(|pair| pair[0] != pair[1]).count();
    let max_transitions = counts.len() - 1;
    1.0 - transitions as f64 / max_transitions as f64
}

fn calculate_mode_score(counts: &[usize], modal_count: usize) -> f64 {
    let modal_frequency = counts.iter().filter(|&&count| count == modal_count).count();
    modal_frequency as f64 / counts.len() as f64
}

/// Python's `sum()` of floats, which compensates with Neumaier's algorithm since 3.12.
fn sum_python_floats(values: impl Iterator<Item = f64>) -> f64 {
    let mut total = 0.0;
    let mut compensation = 0.0;
    for value in values {
        let next = total + value;
        if f64::abs(total) >= f64::abs(value) {
            compensation += (total - next) + value;
        } else {
            compensation += (value - next) + total;
        }
        total = next;
    }
    if compensation != 0.0 && compensation.is_finite() {
        total += compensation;
    }
    total
}
