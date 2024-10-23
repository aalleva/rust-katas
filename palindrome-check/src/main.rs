
fn is_palindrome(a_string : &str) -> bool {

    let ordered : String = a_string.chars()
        .filter( |c| c.is_alphanumeric() )
        .map ( |c| c.to_ascii_lowercase() )
        .collect();

    ordered == ordered.chars().rev().collect::<String>()
}

fn main() {
    
    let word = "level";
    println!("Is \'{}\' a polindrome? {}", word, is_palindrome(word));

}

// Write Unit Test for a is_polindrome function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("12321"), true);
        assert_eq!(is_palindrome("12345"), false);
        assert_eq!(is_palindrome("!"), true);
        assert_eq!(is_palindrome("This is not a palindrome"), false);
    }
}