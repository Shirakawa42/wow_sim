use crate::{effect::Effect, unit::Unit, TICK};

pub struct Ability {
    pub default_cooldown: f32,
    pub cooldown: f32,
    pub current_cooldown: f32,
    pub haste_cooldown: bool,
    pub effects: Vec<Box<dyn Effect>>,
    pub in_gcd: bool,
    pub cast_count: u32,
}

impl Ability {
    pub fn recalculate_cooldown(&mut self, caster: &Unit) {
        if self.haste_cooldown {
            self.cooldown = self.default_cooldown * (100.0 - caster.computed_stats.haste) / 100.0;
        }
    }

    pub fn cast(&mut self, target: &mut Unit, caster: &mut Unit) -> f32 {
        let mut damages = 0.0;
        self.cast_count += 1;
        if self.in_gcd {
            caster.remaining_gcd = caster.gcd;
        }
        self.current_cooldown = self.cooldown;
        for effect in self.effects.iter_mut() {
            damages += effect.apply(target, caster);
        }
        return damages;
    }

    pub fn on_cooldown(&self, remaining_gcd: f32) -> bool {
        self.current_cooldown > 0.0 || remaining_gcd > 0.0
    }

    pub fn tick(&mut self) {
        if self.current_cooldown > 0.0 {
            self.current_cooldown -= TICK;
        }
    }
}
