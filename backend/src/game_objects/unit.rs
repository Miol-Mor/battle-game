use serde::Serialize;

#[derive(Clone, Serialize, Debug, Copy)]
pub struct Unit {
    pub player: u32,
    pub hp: u32,
    pub damage: [u32; 2],
    pub speed: u32,
    pub movements: u32,
}

impl Unit {
    pub fn new(player: u32, hp: u32, damage: [u32; 2], speed: u32) -> Unit {
        assert!(damage[0] <= damage[1]);
        Unit {
            player,
            hp,
            damage,
            speed,
            movements: speed,
        }
    }

    pub fn change_hp(&mut self, diff: i32) {
        let hp = (self.hp as i32) + diff;
        self.hp = if hp >= 0 { hp as u32 } else { 0 };
    }

    pub fn change_movements(&mut self, diff: u32) {
        assert!(self.movements >= diff);

        self.movements -= diff;
    }

    pub fn restore_movements(&mut self) {
        self.movements = self.speed;
    }

    pub fn has_moved(self) -> bool {
        self.movements != self.speed
    }
}

#[cfg(test)]
mod test {}
