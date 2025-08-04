#[derive(Debug)]
pub enum Rule {
    MinLength(usize),
    RequireUpperLower,
    RequireNumber,
    RequireSpecialChar,
}

pub struct PasswordChecker {
    rules: Vec<Rule>,
}

impl PasswordChecker {
    pub fn new() -> Self {
        PasswordChecker { rules: vec![] }
    }

    pub fn min_length(mut self, len: usize) -> Self {
        self.rules.push(Rule::MinLength(len));
        self
    }

    pub fn require_upper_lower(mut self) -> Self {
        self.rules.push(Rule::RequireUpperLower);
        self
    }

    pub fn require_number(mut self) -> Self {
        self.rules.push(Rule::RequireNumber);
        self
    }

    pub fn require_special_char(mut self) -> Self {
        self.rules.push(Rule::RequireSpecialChar);
        self
    }

    pub fn validate(&self, password: &str) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        const SPECIAL_CHARS: [char; 30] = [
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
            '-', '_', '=', '+', '[', ']', '{', '}', '\\', '|',
            ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?',
        ];

        for rule in &self.rules {
            match rule {
                Rule::MinLength(len) => {
                    if password.len() < *len {
                        errors.push(format!("Password must be at least {} characters", len));
                    }
                }
                Rule::RequireUpperLower => {
                    if !password.chars().any(|c| c.is_ascii_uppercase()) ||
                       !password.chars().any(|c| c.is_ascii_lowercase()) {
                        errors.push("Password must include both uppercase and lowercase letters".to_string());
                    }
                }
                Rule::RequireNumber => {
                    if !password.chars().any(|c| c.is_ascii_digit()) {
                        errors.push("Password must include at least one number".to_string());
                    }
                }
                Rule::RequireSpecialChar => {
                    if !password.chars().any(|c| SPECIAL_CHARS.contains(&c)) {
                        errors.push("Password must include at least one special character".to_string());
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
    fn test_valid_password_all_rules() {
        let checker = PasswordChecker::new()
            .min_length(8)
            .require_upper_lower()
            .require_number()
            .require_special_char();

        let result = checker.validate("Passw0rd!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_short_password() {
        let checker = PasswordChecker::new().min_length(10);
        let result = checker.validate("short");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_uppercase() {
        let checker = PasswordChecker::new().require_upper_lower();
        let result = checker.validate("alllowercase");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_number() {
        let checker = PasswordChecker::new().require_number();
        let result = checker.validate("NoNumbersHere");
        assert!(result.is_err());
    }

    #[test]
    fn test_only_special_char_rule_passes() {
        let checker = PasswordChecker::new().require_special_char();
        let result = checker.validate("abc@");
        assert!(result.is_ok());
    }
}
