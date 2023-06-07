pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|char| forbidden_characters.contains(&char));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!(
                "`{}` is not a valid subscriber name. Subscriber name cannot be empty, more than 256 characters long, or contain the following characters: {:?}",
                s,
                forbidden_characters,
            )
        } else {
            Self(s)
        }
    }
}