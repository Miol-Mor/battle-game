use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Unit {
    pub hp: u32,
    pub attack: u32,
}

#[cfg(test)]
mod test {
    use super::Unit;

    #[test]
    fn sertialize() {
        let unit = Unit { hp: 10, attack: 2 };
        let unit_string = serde_json::to_string(&unit).unwrap();

        assert_eq!(unit_string, "{\"hp\":10,\"attack\":2}");
    }
}
