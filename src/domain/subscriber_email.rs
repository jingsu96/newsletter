//! src/domain/subscriber_email.rs

use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        // TODO: add validation!
        if !validate_email(&s) {
            return Err(format!("Invalid email address: {}", s));
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]

mod tests {
    use super::SubscriberEmail;
    use claims::{assert_err, assert_ok};

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursula.com".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@example.com".to_string();
        let result = SubscriberEmail::parse(email);
        assert_err!(result);
    }
}
