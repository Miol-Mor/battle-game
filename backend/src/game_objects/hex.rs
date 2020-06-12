use super::hex_objects::content::Content;
use super::unit::Unit;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Hex {
    pub x: u32,
    pub y: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

#[cfg(test)]
mod test {
    use crate::fixtures;

    #[test]
    fn serialize_without_anything() {
        let hex = fixtures::hex::hex_empty();
        let hex_string = serde_json::to_string(&hex).unwrap();

        assert_eq!(
            hex_string,
            format!(
                "{{\"x\":{},\"y\":{}}}",
                fixtures::hex::x(),
                fixtures::hex::y()
            )
        );
    }

    #[test]
    fn serialize_with_unit_and_content_wall() {
        let hex = fixtures::hex::hex_with_unit_and_wall();
        let hex_string = serde_json::to_string(&hex).unwrap();

        let unit_string = serde_json::to_string(&fixtures::unit::unit()).unwrap();
        let content_string = serde_json::to_string(&fixtures::content::content_wall()).unwrap();

        assert_eq!(
            hex_string,
            format!(
                "{{\"x\":{},\"y\":{},\"unit\":{},\"content\":{}}}",
                fixtures::hex::x(),
                fixtures::hex::y(),
                unit_string,
                content_string,
            ),
        );
    }
}
