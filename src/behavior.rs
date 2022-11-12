use crate::{effect::Effect, unit::Unit, TICK};

#[derive(Clone)]
pub struct Behavior {
    remaining_duration: f32,
    period: f32,
    start_effects: Vec<Box<dyn Effect>>,
    periodic_effects: Vec<Box<dyn Effect>>,
    expired_effects: Vec<Box<dyn Effect>>,
    current_stacks: u32,
    max_stacks: u32,
}

impl Behavior {
    pub fn tick(&mut self, target: &mut Unit, caster: &mut Unit) {
        if self.remaining_duration > 0.0 {
            self.remaining_duration -= TICK;
        }
        if self.remaining_duration <= 0.0 {
            for effect in self.expired_effects.iter() {
                // apply expiration effect
            }
            //caster.behaviors.remove(self);
        }
    }
}