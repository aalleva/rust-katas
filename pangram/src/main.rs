fn is_pangram(a_string: &str) -> bool {
    let mut alphabet = [false; 26];
    
    a_string.chars()
        .filter (|c| c.is_alphabetic())
        .map (|c| c.to_ascii_lowercase())
        .for_each (|c| {
            let index = c.to_ascii_lowercase() as usize - 'a' as usize;
            alphabet[index] = true;
        } );

    for b in alphabet.iter() {
        if !b {
            return false;
        }
    }

    return true;
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
