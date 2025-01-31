use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Getters, Debug)]
pub struct FieldLink {
    uri: String,
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_field_link() {
        let json_str = mock_field_link();
        let field_link = serde_json::from_str::<FieldLink>(json_str).unwrap();

        assert_eq!(field_link.uri(), "https://example.com/test");
        assert_eq!(field_link.title(), "test");
    }

    fn mock_field_link() -> &'static str {
        r#"
        {
            "uri": "https://example.com/test",
            "title": "test"
        }
        "#
    }
}
