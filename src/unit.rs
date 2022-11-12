use std::collections::{HashSet, HashMap};

use crate::{
    behavior::Behavior, computed_stats::ComputedStats,
    damage_multipliers::DamageMultipliers, stats::Stats, ability::Ability, all_abilities, TICK,
};

pub struct Unit {
    pub stats: Stats,
    pub computed_stats: ComputedStats,
    pub damage_multipliers: DamageMultipliers,
    pub abilities: HashMap<u32, Ability>,
    pub behaviors: Vec<Behavior>,
    pub gcd: f32,
    pub remaining_gcd: f32,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            stats: Stats::new(),
            computed_stats: ComputedStats::new(),
            damage_multipliers: DamageMultipliers::new(),
            abilities: HashMap::new(),
            behaviors: vec![],
            gcd: 1.5,
            remaining_gcd: 0.0,
        }
    }

    pub fn add_ability(&mut self, id: u32, all_abilities: &HashMap<u32, fn(&Unit) -> Ability>) {
        self.abilities.insert(id, all_abilities.get(&id).unwrap()(&self));
    }

    pub fn recalculate_gcd(&mut self) {
        self.gcd = 1.5 * (100.0 - self.computed_stats.haste) / 100.0;
    }

    pub fn tick(&mut self) {
        if self.remaining_gcd > 0.0 {
            self.remaining_gcd -= TICK;
        }
        for (_, ability) in self.abilities.iter_mut() {
            ability.tick();
        }
        // tick behaviors
    }
}