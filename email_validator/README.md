# Kata 7: Email Validator with Option and Result

## Description
Write a function to validate email addresses in a basic format. The function should check if a string has the basic format of an email (`user@domain.ext`). If the email is valid, it returns `Some(String)` with the email; if not, it returns `None`. Then, create a second function that validates and returns a `Result`, indicating whether the address is valid or not, with custom error messages.

## Instructions

1. **First Function**: Create a function `validate_email` that takes a string and returns an `Option<String>`.
   - Return `Some(email)` if the email is valid.
   - Return `None` if the email is invalid.

2. **Second Function**: Create a function `validate_email_with_result` that takes a string and returns a `Result<String, String>`.
   - Return `Ok(email)` if the email is valid.
   - Return `Err(message)` with a custom error message if the email is invalid.

3. Validation Requirements:
   - Must contain exactly one `@` character.
   - Must contain a domain (e.g., `gmail.com`).
   - The domain must have at least one extension (e.g., `.com`).

## Examples
```rust
assert_eq!(validate_email("user@example.com"), Some("user@example.com".to_string()));
assert_eq!(validate_email("user@com"), None);

assert_eq!(validate_email_with_result("user@example.com"), Ok("user@example.com".to_string()));
assert_eq!(validate_email_with_result("user@com"), Err("Invalid email format".to_string()));
