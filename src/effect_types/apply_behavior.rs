use std::mem;

use crate::{behavior::{Behavior}, effect::Effect, unit::Unit};

#[derive(Clone)]
pub struct ApplyBehaviorEffect {
    pub behavior: Behavior,
}

impl Effect for ApplyBehaviorEffect {
    fn apply(&mut self, target: &mut Unit, caster: &mut Unit) -> f32 {
        target.behaviors.push(self.behavior.clone());
        return 0.0;
    }

    fn box_clone(&self) -> Box<dyn Effect> {
        Box::new(self.clone())
    }
}
