# Kata 4: Detect the Pangram

## Description:
A **pangram** is a sentence that contains every letter of the alphabet at least once. Write a function in Rust that checks if a given string is a pangram or not. Return `true` if it is a pangram, and `false` otherwise. Ignore numbers, spaces, and symbols, and treat uppercase and lowercase letters as equal.

## Instructions:
- Create a function `is_pangram` that takes a string as a parameter.
- The function should return `true` if the string is a pangram, and `false` otherwise.
- Ignore numbers, spaces, and symbols; only consider alphabetic characters.

## Examples:
```rust
assert_eq!(is_pangram("The quick brown fox jumps over the lazy dog"), true);
assert_eq!(is_pangram("Hello, world!"), false);
