use super::hex_objects::content::Content;
use super::unit::Unit;
use crate::api::common::Point;
use serde::Serialize;

#[derive(Clone, Serialize, Debug, Copy)]
pub struct Hex {
    pub x: u32,
    pub y: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
}

impl Hex {
    pub fn get_unit(&self) -> Option<Unit> {
        self.unit.clone()
    }

    pub fn get_unit_mut(&mut self) -> Option<&mut Unit> {
        self.unit.as_mut()
    }

    pub fn set_unit(&mut self, unit: Option<Unit>) {
        self.unit = unit;
    }

    pub fn get_content(&self) -> Option<Content> {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: Option<Content>) {
        self.content = content;
    }

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::hex_objects::content::Content;
    use super::super::hex_objects::wall::Wall;
    use super::super::unit::Unit;
    use super::Hex;

    #[test]
    fn serialize_without_anything() {
        let cell = Hex {
            x: 1,
            y: 2,
            unit: None,
            content: None,
        };
        let cell_string = serde_json::to_string(&cell).unwrap();

        assert_eq!(cell_string, "{\"x\":1,\"y\":2}",);
    }

    #[test]
    fn serialize_with_unit_and_content_wall() {
        let unit = Unit::new(1, 10, [2, 4], 4);
        let content = Content::Wall(Wall {});
        let hex = Hex {
            x: 1,
            y: 2,
            unit: Some(unit.clone()),
            content: Some(content),
        };
        let hex_string = serde_json::to_string(&hex).unwrap();
        let unit_string = serde_json::to_string(&unit).unwrap();
        let content_string = serde_json::to_string(&content).unwrap();

        assert_eq!(
            hex_string,
            format!(
                "{{\"x\":1,\"y\":2,\"unit\":{},\"content\":{}}}",
                unit_string, content_string,
            ),
        );
    }
}
