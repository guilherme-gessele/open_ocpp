#[derive(Debug, PartialEq)]
struct IdToken {
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
}
