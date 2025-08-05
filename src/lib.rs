/// Password validation rules with optional custom error messages.
#[derive(Debug)]
pub enum Rule<'a> {
    MinLength(usize, Option<&'a str>),
    RequireUpperLower(Option<&'a str>),
    RequireNumber(Option<&'a str>),
    RequireSpecialChar(Option<&'a str>),
}

/// PasswordChecker holds the rules and validates passwords.
pub struct PasswordChecker<'a> {
    rules: Vec<Rule<'a>>,
}

impl<'a> PasswordChecker<'a> {
    /// Creates a new empty PasswordChecker.
    pub fn new() -> Self {
        PasswordChecker { rules: vec![] }
    }

    /// Adds a minimum length rule with an optional custom message.
    ///
    /// # Arguments
    ///
    /// * `len` - Minimum required length.
    /// * `msg` - Optional custom error message.
    pub fn min_length(mut self, len: usize, msg: Option<&'a str>) -> Self {
        self.rules.push(Rule::MinLength(len, msg));
        self
    }

    /// Adds a rule requiring both uppercase and lowercase letters.
    pub fn require_upper_lower(mut self, msg: Option<&'a str>) -> Self {
        self.rules.push(Rule::RequireUpperLower(msg));
        self
    }

    /// Adds a rule requiring at least one digit.
    pub fn require_number(mut self, msg: Option<&'a str>) -> Self {
        self.rules.push(Rule::RequireNumber(msg));
        self
    }

    /// Adds a rule requiring at least one special character.
    pub fn require_special_char(mut self, msg: Option<&'a str>) -> Self {
        self.rules.push(Rule::RequireSpecialChar(msg));
        self
    }

    /// Validates the given password against all configured rules.
    ///
    /// Returns:
    /// - `Ok(())` if all rules pass.
    /// - `Err(Vec<String>)` with all error messages if validation fails.
    pub fn validate(&self, password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Allowed special characters for validation.
        const SPECIAL_CHARS: [char; 30] = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
            '-', '_', '=', '+', '[', ']', '{', '}', '\\', '|',
            ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?',
        ];

        for rule in &self.rules {
            match rule {
                Rule::MinLength(len, maybe_msg) => {
                    if password.len() < *len {
                        let msg = maybe_msg.unwrap_or_else(|| {
                            Box::leak(format!("Password must be at least {} characters long.", len).into_boxed_str())
                        });
                        errors.push(msg.to_string());
                    }
                }
                Rule::RequireUpperLower(maybe_msg) => {
                    if !password.chars().any(|c| c.is_ascii_uppercase()) ||
                       !password.chars().any(|c| c.is_ascii_lowercase()) {
                        let msg = maybe_msg.unwrap_or("Password must include both uppercase and lowercase letters.");
                        errors.push(msg.to_string());
                    }
                }
                Rule::RequireNumber(maybe_msg) => {
                    if !password.chars().any(|c| c.is_ascii_digit()) {
                        let msg = maybe_msg.unwrap_or("Password must include at least one number.");
                        errors.push(msg.to_string());
                    }
                }
                Rule::RequireSpecialChar(maybe_msg) => {
                    if !password.chars().any(|c| SPECIAL_CHARS.contains(&c)) {
                        let msg = maybe_msg.unwrap_or("Password must include at least one special character.");
                        errors.push(msg.to_string());
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_password_all_rules() {
        let checker = PasswordChecker::new()
            .min_length(8, None)
            .require_upper_lower(None)
            .require_number(None)
            .require_special_char(None);

        let result = checker.validate("Passw0rd!");
        assert!(result.is_ok());
    }

    #[test]
    fn password_too_short() {
        let checker = PasswordChecker::new().min_length(10, None);
        let result = checker.validate("short");
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("at least 10 characters")));
    }

    #[test]
    fn missing_uppercase() {
        let checker = PasswordChecker::new().require_upper_lower(None);
        let result = checker.validate("alllowercase");
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("uppercase and lowercase")));
    }

    #[test]
    fn missing_number() {
        let checker = PasswordChecker::new().require_number(None);
        let result = checker.validate("NoNumbersHere");
        assert!(result.is_err());
        assert!(result.unwrap_err().iter().any(|e| e.contains("at least one number")));
    }

    #[test]
    fn special_char_rule_passes() {
        let checker = PasswordChecker::new().require_special_char(None);
        let result = checker.validate("abc@");
        assert!(result.is_ok());
    }

    #[test]
    fn custom_error_messages_work() {
        let checker = PasswordChecker::new()
            .min_length(8, Some("Password length must be 8 or more."))
            .require_upper_lower(Some("Must have uppercase & lowercase letters."))
            .require_number(Some("Must include a number."))
            .require_special_char(Some("Must include a special character."));

        let result = checker.validate("abc");
        assert!(result.is_err());
        let errors = result.unwrap_err();

        assert!(errors.contains(&"Password length must be 8 or more.".to_string()));
        assert!(errors.contains(&"Must have uppercase & lowercase letters.".to_string()));
        assert!(errors.contains(&"Must include a number.".to_string()));
        assert!(errors.contains(&"Must include a special character.".to_string()));
    }
}
