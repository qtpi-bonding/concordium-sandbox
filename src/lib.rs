//! A sandbox for concordium's worker agents. Agents are assigned Actions
//! against this crate and push their work straight to `main`.

/// Returns the sum of the numbers.
pub fn sum(values: &[i64]) -> i64 {
    values.iter().sum()
}

/// Returns `s` with its words in reverse order, separated by single spaces.
pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_nothing_is_zero() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn sum_adds_negative_numbers() {
        assert_eq!(sum(&[3, -5, 10]), 8);
    }

    #[test]
    fn reverse_words_reverses_and_normalizes_spacing() {
        assert_eq!(reverse_words("  the quick   fox "), "fox quick the");
    }
}
