pub mod validators {
    pub fn validate_email(email: &str) -> Result<(), String> {
        if email.contains('@') && email.contains('.') && email.len() > 5 {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        if !password.chars().any(|c| c.is_digit(10)) {
            return Err("Password must contain at least one number".to_string());
        }
        Ok(())
    }
}
