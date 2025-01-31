use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct FieldPath {
    alias: String,
    #[serde(rename = "langcode")]
    lang_code: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pid: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_field_path() {
        let json_str = mock_field_path();
        let field_path = serde_json::from_str::<FieldPath>(json_str).unwrap();

        assert_eq!(field_path.alias(), "test");
        assert_eq!(field_path.pid().clone(), 1);
        assert_eq!(field_path.lang_code(), "en");
    }

    fn mock_field_path() -> &'static str {
        r#"
        {
            "alias": "test",
            "langcode": "en",
            "pid": "1"
        }
        "#
    }
}
