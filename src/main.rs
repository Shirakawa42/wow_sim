mod abilities;
mod ability;
mod behavior;
mod computed_stats;
mod damage_multipliers;
mod effect;
mod effect_types;
mod stats;
mod unit;

const FIGHT_DURATION: f32 = 300.0;

fn main() {
    let current_time = 0.0;
    let total_damages = 0.0;
    while current_time < FIGHT_DURATION {}
    println!("Total damages: {}", total_damages);
    println!("DPS: {}", total_damages / current_time);
}
