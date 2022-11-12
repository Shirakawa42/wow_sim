pub struct ComputedStats {
    pub attack_power: u32,
    pub spell_power: u32,
    pub critical_chances: f32,
    pub haste: f32,
    pub mastery_points: u32,
    pub versatility: f32,
}

impl ComputedStats {
    pub fn new() -> Self {
        Self {
            attack_power: 0,
            spell_power: 0,
            critical_chances: 0.0,
            haste: 0.0,
            mastery_points: 0,
            versatility: 0.0,
        }
    }
}