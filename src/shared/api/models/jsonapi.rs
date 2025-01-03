use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters)]
pub struct Resource<T> {
    jsonapi: JsonApi,
    data: T,
    links: Links,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
struct JsonApi {
    version: String,
    parsed: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
struct Links {
    #[serde(rename = "self")]
    current: Link,
    #[serde(rename = "next")]
    next: Option<Link>,
    #[serde(rename = "prev")]
    prev: Option<Link>,
}

#[derive(Deserialize, Serialize, Debug, Getters)]
struct Link {
    href: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_jsonapi() {
        let json_str = mock_jsonapi();

        #[derive(Deserialize, Serialize, Debug, Getters)]
        struct NodeExample {
            id: String,
            #[serde(rename = "type")]
            entity_type: String,
        }

        let resource = serde_json::from_str::<Resource<NodeExample>>(json_str).unwrap();

        assert_eq!(resource.data().id(), "1");
        assert_eq!(resource.data().entity_type(), "node--example");
        assert_eq!(resource.jsonapi().version(), "1.0");
        assert_eq!(*resource.jsonapi().parsed(), Some(true));
        assert!(resource.links().next().is_none());
        assert!(resource.links().prev().is_none());
        assert_eq!(
            resource.links().current().href(),
            "https://example.com/test"
        );
    }

    fn mock_jsonapi() -> &'static str {
        r#"
        {
            "jsonapi": {
                "version": "1.0",
                "parsed": true
            },
            "data": {
                "type": "node--example",
                "id": "1",
                "attributes": {}
            },
            "links": {
                "self": {
                    "href": "https://example.com/test"
                }
            }
        }
        "#
    }
}
