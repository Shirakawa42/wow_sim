pub struct Stats {
    pub agility: u32,
    pub crit_score: u32,
    pub haste_score: u32,
    pub mastery_score: u32,
    pub versatility_score: u32,
}

impl Stats {
    pub fn get_crit_chance(&self) -> f32 {
        self.crit_score as f32 * 1.0 / 35.0 + 10.0
    }

    pub fn get_haste(&self) -> f32 {
        self.haste_score as f32 * 1.0 / 33.0
    }

    pub fn get_versatility(&self) -> f32 {
        self.versatility_score as f32 * 1.0 / 40.0
    }

    pub fn get_mastery(&self) -> f32 {
        self.mastery_score as f32 * 1.0 / 35.0 * 2.0 + 16.0
    }

    pub fn get_stormbringer_chance(&self) -> f32 {
        self.mastery_score as f32 * 1.0 / 35.0 * 0.08 + 0.64 + 5.0
    }

    pub fn get_windfury_chance(&self) -> f32 {
        self.mastery_score as f32 * 1.0 / 35.0 * 0.08 + 0.64 + 25.0
    }

    pub fn get_gcd(&self) -> f32 {
        1.5 / (1.0 + self.get_haste() / 100.0)
    }
}
