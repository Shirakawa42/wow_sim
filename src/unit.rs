use crate::{
    ability::Ability, behavior::Behavior, computed_stats::ComputedStats,
    damage_multipliers::DamageMultipliers, stats::Stats,
};

pub struct Unit {
    stats: Stats,
    computed_stats: ComputedStats,
    damage_multipliers: DamageMultipliers,
    abilities: Vec<Ability>,
    behaviors: Vec<Behavior>,
}
