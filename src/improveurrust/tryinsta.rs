#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_serialization() {
        let user = User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            username: "johndoe".to_string(),
        };

        insta::assert_json_snapshot!(&user);
    }
}
