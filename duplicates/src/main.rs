use std::collections::HashMap;

const MAX_OCCURRENCES: u32 = 1;

fn count_duplicates(text: &str) -> u32 {

    let mut chars_occurrences = HashMap::new();

    text.chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .for_each(|c| {
            *chars_occurrences.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        });

    chars_occurrences
        .iter()
        .filter(|(_, &value)| value > MAX_OCCURRENCES)
        .count() as u32
}

fn main() {
    let text = "Manchester United";
    println!("The word '{}' has {} duplicates", text, count_duplicates(text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_duplicates() {
        assert_eq!(count_duplicates("aA11"), 2);
        assert_eq!(count_duplicates("abcde"), 0);
        assert_eq!(count_duplicates("Indivisibilities"), 2);
        assert_eq!(count_duplicates("AaBbCcDdEeFf"), 6);
        assert_eq!(count_duplicates("aabBcde"), 2);
    }
}
