use chrono::{DateTime, Utc};

use super::enumerations::AuthorizationStatus;

pub struct AuthorizationData {
    /// The identifier to with this authorization applies
    id_tag: IdToken,
    /// This contains information about authorization status,
    /// expiry and parent id. For a differential update the
    /// following applies: If this element is present, then
    /// this entry SHALL be added or updated in the Local
    /// Authorization List. If this element is absent, than
    /// the Local Authorization List SHALL be deleted.
    id_tag_info: Option<IdTagInfo>,
}

pub struct IdTagInfo {
    expiry_date: Option<DateTime<Utc>>,
    parent_id_tag: Option<IdToken>,
    status: AuthorizationStatus,
}

#[derive(Debug)]
pub struct IdToken {
    id_token: String,
}

impl IdToken {
    pub fn new(id_token: String) -> IdToken {
        if id_token.len() > 20 {
            panic!("Max size of 20 characters")
        }

        IdToken { id_token }
    }
}

impl PartialEq for IdToken {
    fn eq(&self, other: &Self) -> bool {
        self.id_token.to_lowercase() == other.id_token.to_lowercase()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::iter;

    #[test]
    fn test_with_valid_id_token() {
        let token = String::from("abcd-01234");
        let id_token = IdToken::new(token);

        assert_eq!(
            id_token,
            IdToken {
                id_token: String::from("abcd-01234")
            }
        )
    }

    #[test]
    #[should_panic]
    fn test_with_invalid_id_token() {
        let token = iter::repeat("a").take(21).collect();
        let _id_token = IdToken::new(token);
    }

    #[test]
    fn test_id_token_partial_equals_with_same_token_case() {
        let id_token_a = IdToken::new(String::from("abcd-01234"));
        let id_token_b = IdToken::new(String::from("abcd-01234"));

        assert_eq!(true, id_token_a == id_token_b)
    }

    #[test]
    fn test_id_token_partial_equals_with_distinct_token_case() {
        let id_token_a = IdToken::new(String::from("abcd-01234"));
        let id_token_b = IdToken::new(String::from("ABCD-01234"));

        assert_eq!(true, id_token_a == id_token_b)
    }

    #[test]
    fn test_id_token_partial_equals_with_distict_tokens() {
        let id_token_a = IdToken::new(String::from("abcd-01234"));
        let id_token_b = IdToken::new(String::from("efgh-01234"));

        assert_eq!(false, id_token_a == id_token_b)
    }
}
