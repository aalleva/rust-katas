use std::collections::HashMap;

fn are_anagrams(first_word: &str, second_word: &str) -> bool {

    let mut first_word_chars = HashMap::new();
    let mut second_word_chars = HashMap::new();

    first_word.chars()
        .filter(|c| c.is_alphabetic() )
        .for_each(|c| {
            *first_word_chars.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        });

    second_word.chars()
        .filter(|c| c.is_alphabetic() )
        .for_each(|c| {
            *second_word_chars.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        });

    first_word_chars ==  second_word_chars
}

fn main() {
    let first_word = "Astronomer";
    let second_word = "Moon starer";
    println!("{} and {} are anagrams: {}", first_word, second_word, are_anagrams(first_word, second_word));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_anagrams() {
        assert_eq!(are_anagrams("Listen", "Silent"), true);
        assert_eq!(are_anagrams("Hello, World!", "Droll, he woL!"), true);
        assert_eq!(are_anagrams("Dormitory", "Dirty room"), true);
        assert_eq!(are_anagrams("Astronomer", "Moon starer"), true);
        assert_eq!(are_anagrams("Hello, Word!", "Droll, he woL!"), false);
    }
}
