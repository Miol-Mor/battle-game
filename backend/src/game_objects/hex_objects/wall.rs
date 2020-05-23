use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum WallKind {
    Default,
}

#[derive(Clone, Serialize)]
pub struct Wall {
    pub kind: WallKind,
}

#[cfg(test)]
mod test {
    use super::{Wall, WallKind};

    #[test]
    fn serialize() {
        let wall = Wall {
            kind: WallKind::Default,
        };
        let wall_string = serde_json::to_string(&wall).unwrap();

        assert_eq!(wall_string, "{\"kind\":\"Default\"}");
    }
}
