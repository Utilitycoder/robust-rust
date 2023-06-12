#[derive(Debug)]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let name = s.trim().to_string();
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.len() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name
            .chars()
            .any(|char| forbidden_characters.contains(&char));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!(
                "`{}` is not a valid subscriber name. Subscriber name cannot be empty, more than 256 characters long, or contain the following characters: {:?}",
                name,
                forbidden_characters,
            ))
        } else {
            Ok(Self(name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_graheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_containing_a_invalid_character_is_rejected() {
        let invalid_characters = vec!['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        for character in invalid_characters {
            let name = format!("name{}", character);
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_name_containing_a_valid_character_is_valid() {
        let valid_characters = vec!['a', 'A', '1', '-', '.', '_'];
        for character in valid_characters {
            let name = format!("name{}", character);
            assert_ok!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_trimmed() {
        let name = "  John Doe  ".to_string();
        let parsed_name = SubscriberName::parse(name).unwrap();
        assert_eq!(parsed_name.as_ref(), "John Doe");
    }

    #[test]
    fn a_name_with_valid_characters_is_valid() {
        let name = "John Doe".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
