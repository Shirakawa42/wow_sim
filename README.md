Effect -> different types of effect
    - damages: deal a type of damages to target
    - apply behavior: add a behavior to the target

Ability -> apply effects on [target] from [caster]
struct:
{
    cooldown: f32
    current_cooldown: f32
    target: Unit
    effects: Effect[]
}

Behavior -> apply effect on [self] from [caster]
struct:
{
    remaining_duration: f32
    period: f32
    start_effects: Effect[]
    periodic_effects: Effect[]
    expired_effects: Effect[]
    caster: Unit // damages are calculated with the caster stats
}

Unit -> contains all infos of a unit
struct:
{
    stats: Stats
    computed_stats: ComputedStats
    damage_multipliers: DamageMultipliers
    abilities: Ability[]
    behaviors: Behavior[]
}

Stats -> contains not computed stats of a unit
struct:
{
    primary_stat: u32
    critical_score: u32
    haste_score: u32
    mastery_score: u32
    versatility_score: u32
}

ComputedStats -> contains computed stats of a unit
struct:
{
    attack_power: u32
    spell_power: u32
    critical_chances: f32
    haste: f32
    mastery_points: u32
    versatility: f32
}

DamageMultipliers -> contains damages modifiers of a unit, all values are 1.0 by default
struct:
{
    critical_damages: f32
    global_damage: f32
    physical_damages: f32
    elemental_damages: f32
    fire_damages: f32
    frost_damages: f32
    arcane_damages: f32
    nature_damages: f32
    chaos_damages: f32
}