pub struct Stats {
    primary_stat: u32,
    critical_score: u32,
    haste_score: u32,
    mastery_score: u32,
    versatility_score: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            primary_stat: 0,
            critical_score: 0,
            haste_score: 0,
            mastery_score: 0,
            versatility_score: 0,
        }
    }
}