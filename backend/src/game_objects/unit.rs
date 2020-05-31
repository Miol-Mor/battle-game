use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Unit {
    pub player: u32,
    pub hp: u32,
    pub attack: [u32; 2],
    pub speed: u32,
}

#[cfg(test)]
mod test {
    use super::Unit;

    #[test]
    fn serialize() {
        let unit = Unit {
            player: 1,
            hp: 10,
            attack: [2, 3],
            speed: 3,
        };
        let unit_string = serde_json::to_string(&unit).unwrap();

        assert_eq!(
            unit_string,
            "{\"player\":1,\"hp\":10,\"attack\":[2,3],\"speed\":3}",
        );
    }
}
