use std::{mem, collections::HashMap};

use crate::{all_abilities::AbilityIds, unit::Unit};

impl Unit {
    pub fn enhancement_shaman_rotation_1(&mut self, target: &mut Unit) -> f32 {
        let mut damages_done = 0.0;
        let mut abilities = mem::take(&mut self.abilities);

        let stormstrike = abilities.get_mut(&(AbilityIds::EnhStormstrike as u32)).unwrap();
        if !stormstrike.on_cooldown(self.remaining_gcd) {
            damages_done += stormstrike.cast(target, self);
        }

        self.abilities = abilities;
        return damages_done;
    }
}
