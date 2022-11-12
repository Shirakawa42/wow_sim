pub struct DamageMultipliers {
    critical_damages: f32,
    global_damage: f32,
    damages_type: [f32; 7],
}

impl DamageMultipliers {
    pub fn new() -> Self {
        Self {
            critical_damages: 1.0,
            global_damage: 1.0,
            damages_type: [1.0; 7],
        }
    }
}