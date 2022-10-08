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
        match self.health { 
            0 => {
                let nana = if self.mana == Some(0) {
                    Some(100)
                } else {
                    None
                };
                Some(Player {
                    health: 100,
                    mana: nana,
                    level: self.level,
                })
            },
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health<mana_cost {
                    self.health = 0
                } else {
                    self.health -= mana_cost;
                }
                0
            }
            Some(x) => {
                if x < mana_cost {
                    0
                } else {
                    x - mana_cost
                }
            }
            _ => 0
        }
    }
}
