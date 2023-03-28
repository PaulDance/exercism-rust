pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Self> {
        if self.health != 0 {
            None
        } else {
            Some(Self {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                ..*self
            })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(m) if m < mana_cost => 0,
            Some(m) => {
                self.mana = Some(m - mana_cost);
                2 * mana_cost
            }
        }
    }
}
