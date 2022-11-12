use crate::damages_type::DamagesType;
use crate::effect_types::damages::DamageEffect;
use crate::{ability::Ability, unit::Unit};

impl Ability {
    pub fn enh_stormstrike(caster: &Unit) -> Self {
        let damages_effect = DamageEffect {
            damage: 2000.0,
            damage_type: DamagesType::Physical,
        };
        Self {
            default_cooldown: 7.5,
            cooldown: 7.5 * (100.0 - caster.computed_stats.haste) / 100.0,
            current_cooldown: 0.0,
            haste_cooldown: true,
            effects: vec![Box::new(damages_effect)],
            in_gcd: true,
            cast_count: 0,
        }
    }
}
