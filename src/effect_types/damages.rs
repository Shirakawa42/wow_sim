use crate::{damages_type::DamagesType, effect::Effect, unit::Unit, behavior::{self, Behavior}};

#[derive(Clone)]
pub struct DamageEffect {
    pub damage: f32,
    pub damage_type: DamagesType,
}

impl Effect for DamageEffect {
    fn apply(&mut self, target: &mut Unit, caster: &mut Unit) -> f32 {
        let damage = self.damage;
        return damage;
    }

    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}
