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

/// Slugifies `s`: lowercases ASCII letters, keeps ASCII letters and digits,
/// and collapses every other run of characters into a single `-`. The
/// result never starts or ends with `-`.
pub fn slugify_16d50d(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_sep = true;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            result.push('-');
            last_was_sep = true;
        }
    }
    if result.ends_with('-') {
        result.pop();
    }
    result
}

/// Slugifies `s`: lowercases ASCII letters, keeps ASCII letters and digits,
/// and collapses every other run of characters into a single `-`. The
/// result never starts or ends with `-`.
pub fn slugify_d88da5(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_sep = true;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            result.push('-');
            last_was_sep = true;
        }
    }
    if result.ends_with('-') {
        result.pop();
    }
    result
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

    #[test]
    fn slugify_lowercases_and_collapses_separators() {
        assert_eq!(slugify_16d50d("Hello, World!"), "hello-world");
        assert_eq!(slugify_16d50d("  foo---bar_BAZ99  "), "foo-bar-baz99");
    }

    #[test]
    fn slugify_empty_input_is_empty() {
        assert_eq!(slugify_16d50d(""), "");
    }

    #[test]
    fn slugify_only_punctuation_is_empty() {
        assert_eq!(slugify_16d50d("!!!---???"), "");
    }

    #[test]
    fn slugify_never_starts_or_ends_with_dash() {
        assert_eq!(
            slugify_16d50d("-leading and trailing-"),
            "leading-and-trailing"
        );
    }

    #[test]
    fn slugify_d88da5_lowercases_and_collapses_separators() {
        assert_eq!(slugify_d88da5("Hello, World!"), "hello-world");
        assert_eq!(slugify_d88da5("  foo---bar_BAZ99  "), "foo-bar-baz99");
    }

    #[test]
    fn slugify_d88da5_empty_input_is_empty() {
        assert_eq!(slugify_d88da5(""), "");
    }

    #[test]
    fn slugify_d88da5_only_punctuation_is_empty() {
        assert_eq!(slugify_d88da5("!!!---???"), "");
    }

    #[test]
    fn slugify_d88da5_never_starts_or_ends_with_dash() {
        assert_eq!(
            slugify_d88da5("-leading and trailing-"),
            "leading-and-trailing"
        );
    }
}
