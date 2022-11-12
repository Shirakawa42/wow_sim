use std::collections::HashMap;

use unit::Unit;

use crate::{ability::Ability, all_abilities::AbilityIds};

mod abilities;
mod ability;
mod all_abilities;
mod behavior;
mod computed_stats;
mod damage_multipliers;
mod effect;
mod effect_types;
mod rotation;
mod stats;
mod unit;
mod damages_type;

const FIGHT_DURATION: f32 = 300.0;
const TICK: f32 = 0.1;

fn main() {
    let all_abilities: HashMap<u32, fn(&Unit) -> Ability> = all_abilities::init_ability_list();
    let mut player = Unit::new();
    let mut target = Unit::new();

    player.add_ability(AbilityIds::EnhStormstrike as u32, &all_abilities);

    let mut duration = 0.0;
    let mut total_damages = 0.0;

    while duration < FIGHT_DURATION {
        total_damages += player.enhancement_shaman_rotation_1(&mut target);
        player.tick();
        target.tick();
        duration += TICK;
    }

    println!("Total stormstrike casted: {}", player.abilities.get(&(AbilityIds::EnhStormstrike as u32)).unwrap().cast_count);
    println!("Total damages done: {}", total_damages);
    println!("DPS: {}", total_damages / FIGHT_DURATION);
}
