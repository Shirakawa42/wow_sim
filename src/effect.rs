use crate::{unit::Unit, damages_type::DamagesType, behavior::{Behavior, self}};

pub trait Effect {
    fn apply(&mut self, target: &mut Unit, caster: &mut Unit) -> f32;
    fn box_clone(&self) -> Box<dyn Effect>;
}

impl Clone for Box<dyn Effect> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}