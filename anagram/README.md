# Kata 5: Anagram Detection

## Description
Write a function that determines if two given strings are **anagrams** of each other. An **anagram** is a word or phrase formed by rearranging the letters of another word or phrase, using all the original letters exactly once. Ignore spaces, punctuation, and case differences.

## Instructions
- Create a function `are_anagrams` that takes two strings as parameters.
- The function should return `true` if the strings are anagrams, and `false` otherwise.
- Ignore spaces, punctuation, and treat uppercase and lowercase letters as equal.

## Examples
```rust
assert_eq!(are_anagrams("Listen", "Silent"), true);
assert_eq!(are_anagrams("Hello, World!", "Droll, he woL!"), false);
assert_eq!(are_anagrams("Dormitory", "Dirty room"), true);
