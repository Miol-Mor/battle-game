use super::wall::Wall;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "wall")]
    Wall(Wall),
}

#[cfg(test)]
mod test {
    use super::super::wall::Wall;
    use super::Content;

    #[test]
    fn serialize() {
        let wall = Wall {};
        let content = Content::Wall(wall);

        let content_string = serde_json::to_string(&content).unwrap();
        assert_eq!(content_string, "{\"type\":\"wall\"}");
    }
}
