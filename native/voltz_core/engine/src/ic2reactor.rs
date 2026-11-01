// The public build leaves IndustrialCraft 2's reactor out (IC2 is all rights
// reserved; it stays private until the IC2 team gives permission). This stub
// keeps the same names so the rest compiles; no block uses it.

pub const SLOTS: usize = 54;
pub const TICKS: i32 = 20;
pub const JOULES_PER_EU: i64 = 6;
pub const REACTOR: &str = "voltz:nuclear_reactor";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {}

pub fn part(_id: &str) -> Option<Part> {
    None
}

pub fn depleted(_id: &str) -> Option<&'static str> {
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub part: Part,
    pub heat: i32,
    pub used: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    pub eu_per_tick: i64,
    pub destroyed: Vec<usize>,
    pub spent: Vec<usize>,
    pub meltdown: bool,
}

pub fn step(_grid: &mut [Option<Cell>], _hull: &mut i32) -> Outcome {
    Outcome::default()
}
