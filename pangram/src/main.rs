use std::collections::HashSet;

fn is_pangram(a_string: &str) -> bool {
    let mut letters = HashSet::new();

    a_string.chars()
        .filter (|c| c.is_alphabetic())
        .for_each (|c| {
            letters.insert(c.to_ascii_lowercase());
        } );

    letters.len() == 26
}

fn main() {
    let a_string = "The quick brown fox jumps over the lazy dog";
    println!("Is {} a pangram? {}", a_string, is_pangram(a_string));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pangram() {
        assert_eq!(is_pangram("The quick brown fox jumps over the lazy dog"), true);
        assert_eq!(is_pangram("Hello, world!"), false);
        assert_eq!(is_pangram("Sphinx of black quartz, judge my vow"), true);
        assert_eq!(is_pangram("abcdefghijklmnopqrstuvwxyz"), true);
    }
}
