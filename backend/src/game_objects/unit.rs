use rand::Rng;
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

    pub fn random(
        hp_min_max: (u8, u8),
        damage_min_max: (u8, u8),
        damage_interval: (u8, u8),
        speed_min_max: (u8, u8),
        player: u32,
    ) -> Unit {
        let mut rng = rand::thread_rng();

        let hp = rng.gen_range(hp_min_max.0, hp_min_max.1 + 1) as u32;
        let damage_min = rng.gen_range(damage_min_max.0, damage_min_max.1) as u32;
        let damage_max =
            damage_min + rng.gen_range(damage_interval.0, damage_interval.1 + 1) as u32;
        let speed = rng.gen_range(speed_min_max.0, speed_min_max.1 + 1) as u32;

        Unit::new(player, hp, [damage_min, damage_max], speed)
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

    pub fn has_no_moves(self) -> bool {
        self.movements == 0
    }

    pub fn is_my(self, player: u32) -> bool {
        self.player == player
    }
}

#[cfg(test)]
mod test {}
