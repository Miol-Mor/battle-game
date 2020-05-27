use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum WallKind {
    Normal,
}

#[derive(Clone, Serialize)]
pub struct Wall {
    pub wall: WallKind,
}

#[cfg(test)]
mod test {
    use super::{Wall, WallKind};

    #[test]
    fn sertialize() {
        let wall = Wall {
            wall: WallKind::Normal,
        };
        let wall_string = serde_json::to_string(&wall).unwrap();

        assert_eq!(wall_string, "{\"wall\":\"Normal\"}");
    }
}
