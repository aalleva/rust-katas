fn validate_email(email_address: &str) -> Option<String> {
    const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    let email_regex = regex::Regex::new(EMAIL_REGEX).ok()?;

    if email_regex.is_match( email_address ) {
        return Some ( email_address.to_string() );
    } else {
        None
    }
}

fn validate_email_with_result(email_address: &str) -> Result<String, String> {
    const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    let email_regex = regex::Regex::new(EMAIL_REGEX).map_err(|_| "Invalid regex".to_string())?;

    if email_regex.is_match( email_address ) {
        Ok( email_address.to_string()) 
    } else {
        Err("Invalid email format".to_string())
    }
}

fn main() {
    assert_eq!(validate_email("user@example.com"), Some("user@example.com".to_string()));
    assert_eq!(validate_email("user@com"), None);

    assert_eq!(validate_email_with_result("user@example.com"), Ok("user@example.com".to_string()));
    assert_eq!(validate_email_with_result("user@com"), Err("Invalid email format".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert_eq!(validate_email("user@example.com"), Some("user@example.com".to_string()));
        assert_eq!(validate_email("user@com"), None);
        assert_eq!(validate_email("invalid-email"), None);
    }
 
    #[test]
    fn test_validate_email_with_result() {
        assert_eq!(validate_email_with_result("user@example.com"), Ok("user@example.com".to_string()));
        assert_eq!(validate_email_with_result("user@com"), Err("Invalid email format".to_string()));
        assert_eq!(validate_email_with_result("invalid-email"), Err("Invalid email format".to_string()));
    } 

}
