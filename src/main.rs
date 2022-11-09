use crate::stats::Stats;

mod stats;

const FIGHT_DURATION: f32 = 300.0;

fn main() {
    let stats = Stats {
        agility: 2195,
        crit_score: 32,          // 0.91% (+10% base)
        haste_score: 961,        // 28.76%
        mastery_score: 278,      // 15.89% (+ 16% base)
        versatility_score: 1295, // 32.30%
    };

    let crit_chance = stats.get_crit_chance();
    let haste = stats.get_haste();
    let versatility = stats.get_versatility();
    let mastery = stats.get_mastery();
    let stormbringer_chance = stats.get_stormbringer_chance();
    let windfury_chance = stats.get_windfury_chance();
    let gcd = stats.get_gcd();

    let current_time = 0.0;
    let total_damages = 0.0;
    while current_time < FIGHT_DURATION {

    }
    println!("Total damages: {}", total_damages);
    println!("DPS: {}", total_damages / FIGHT_DURATION);
}
