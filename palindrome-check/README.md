# Kata 1: Palindrome Check

### Description:
Write a function in Rust that checks if a word or phrase is a palindrome, meaning it reads the same forwards and backwards, ignoring spaces, punctuation, and case differences.

### Instructions:
- Create a function `is_palindrome` that takes a string as a parameter.
- The function should return `true` if the string is a palindrome and `false` otherwise.
- Ignore spaces, punctuation, and case differences when comparing.

### Examples:
```rust
assert_eq!(is_palindrome("A man, a plan, a canal, Panama"), true);
assert_eq!(is_palindrome("Hello, World!"), false);
assert_eq!(is_palindrome("racecar"), true);
assert_eq!(is_palindrome("Was it a car or a cat I saw"), true);
