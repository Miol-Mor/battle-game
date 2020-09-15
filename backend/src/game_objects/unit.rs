use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Unit {
    pub player: u32,
    pub hp: u32,
    pub damage: [u32; 2],
    pub speed: u32,
}

impl Unit {
    pub fn change_hp(&mut self, diff: i32) {
        let hp = (self.hp as i32) + diff;
        self.hp = if hp >= 0 { hp as u32 } else { 0 };
    }
}

#[cfg(test)]
mod test {
    use super::Unit;

    #[test]
    fn serialize() {
        let unit = Unit {
            player: 1,
            hp: 10,
            damage: [2, 3],
            speed: 3,
        };
        let unit_string = serde_json::to_string(&unit).unwrap();

        assert_eq!(
            unit_string,
            "{\"player\":1,\"hp\":10,\"damage\":[2,3],\"speed\":3}",
        );
    }
}
