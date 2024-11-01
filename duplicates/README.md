# Kata 6: Counting Duplicates

## Description
Write a function that takes a string and returns the number of characters that appear more than once in the string. Treat uppercase and lowercase letters as the same.

## Instructions
- Create a function `count_duplicates` that takes a string as a parameter.
- The function should return the number of characters that appear more than once, ignoring case differences.
- Only consider alphabetic characters and numbers; ignore spaces, punctuation, and other special characters.

## Examples
```rust
assert_eq!(count_duplicates("aA11"), 2); // 'a' and '1' are repeated
assert_eq!(count_duplicates("abcde"), 0); // No characters are repeated
assert_eq!(count_duplicates("Indivisibilities"), 2); // 'i' and 's' are repeated
