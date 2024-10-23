# Kata 2: Sum the Digits of a Number

## Description:
Write a function in Rust that takes a positive integer and returns the sum of its digits. If the number has more than one digit, continue summing the digits until you get a single digit.

## Instructions:
- Create a function `sum_digits` that takes a positive integer.
- The function should return the sum of the digits of the number.
- If the sum of the digits results in a number with more than one digit, continue summing the digits until you get a single digit.

## Examples:
```rust
assert_eq!(sum_digits(942), 6); // 9 + 4 + 2 = 15, 1 + 5 = 6
assert_eq!(sum_digits(123), 6); // 1 + 2 + 3 = 6
assert_eq!(sum_digits(9999), 9); // 9 + 9 + 9 + 9 = 36, 3 + 6 = 9
assert_eq!(sum_digits(5), 5); // Already a single digit
