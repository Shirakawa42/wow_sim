use std::collections::HashMap;

use crate::ability::Ability;
use crate::unit::Unit;

pub enum AbilityIds {
    EnhStormstrike = 17364,
}

pub fn init_ability_list() -> HashMap<u32, fn(&Unit) -> Ability> {
    let mut ability_list: HashMap<u32, fn(&Unit) -> Ability> = HashMap::new();

    ability_list.insert(AbilityIds::EnhStormstrike as u32, Ability::enh_stormstrike);

    return ability_list;
}