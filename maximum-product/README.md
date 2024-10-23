# Kata 3: Maximum Product in a Series of Numbers

## Description:
Given a string of numbers, write a function in Rust that finds the maximum product of N consecutive digits.

## Instructions:
- Create a function `max_product` that takes a string of numbers and an integer N.
- The function should return the largest product that can be obtained by multiplying N consecutive digits in the string.
- If the length of the string is less than N, the function should return 0.

## Examples:
```rust
assert_eq!(max_product("123456789", 2), 72); // 8 * 9 = 72
assert_eq!(max_product("73167176531330624919225119674426574742355349194934", 6), 23520);
assert_eq!(max_product("12345", 3), 60); // 3 * 4 * 5 = 60
assert_eq!(max_product("123", 5), 0); // String is shorter than N
