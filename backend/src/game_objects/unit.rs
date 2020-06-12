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
    use crate::fixtures;

    #[test]
    fn serialize() {
        let unit = fixtures::unit::unit();
        let unit_string = serde_json::to_string(&unit).unwrap();

        assert_eq!(
            unit_string,
            format!(
                "{{\"player\":{},\"hp\":{},\"attack\":{},\"speed\":{}}}",
                fixtures::unit::player(),
                fixtures::unit::hp(),
                serde_json::to_string(&fixtures::unit::attack()).unwrap(),
                fixtures::unit::speed(),
            ),
        );
    }
}
