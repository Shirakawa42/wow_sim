use crate::{effect::Effect, unit::Unit};

pub struct Behavior {
    cooldown: f32,
    current_cooldown: f32,
    target: Unit,
    effects: Vec<Effect>,
}
