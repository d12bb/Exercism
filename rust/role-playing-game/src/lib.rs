// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match (self.health, self.mana) {
            (0, None) => Some(Player {
                health: 100,
                mana: self.mana,
                level: self.level,
            }),
            (0, Some(_)) => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana < mana_cost => 0,
            Some(_) => {
                self.mana = Some(self.mana.unwrap() - mana_cost);
                2 * mana_cost
            },
            None if self.health < mana_cost => {
                self.health = 0;
                0
            },
            None => {
                self.health -= mana_cost;
                0
            },
        }
    }
}
