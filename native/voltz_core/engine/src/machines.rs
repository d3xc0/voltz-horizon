// The machines' state and their tick, free of game types. Each keeps its
// energy and progress as plain integers that the glue saves in the block
// entity's compound under the same names (`fields` / `load`).
//
// Slots follow vanilla's furnace menu, which every Voltz machine opens:
// 0 input, 1 fuel (or battery), 2 output. Its four data slots draw the flame
// (`lit`, `lit_total`) and the arrow (`progress`, `progress_total`).

use crate::recipes::{self, Recipe, Stack};

pub const INPUT: usize = 0;
pub const FUEL: usize = 1;
pub const OUTPUT: usize = 2;

/// Joules per tick a coal generator makes while it burns.
pub const GENERATOR_OUTPUT: i64 = 60;
pub const GENERATOR_BUFFER: i64 = 10_000;
/// A geothermal generator: a lava bucket runs it this long at this rate.
pub const GEOTHERMAL_OUTPUT: i64 = 100;
pub const GEOTHERMAL_TICKS: i32 = 1_000;
/// A solar panel in open daylight.
pub const SOLAR_OUTPUT: i64 = 10;
pub const BATTERY_CAPACITY: i64 = 1_000_000;
/// The most a battery box takes in or gives out in one tick.
pub const BATTERY_RATE: i64 = 400;
pub const MFE_CAPACITY: i64 = 10_000_000;
pub const MFE_RATE: i64 = 2_000;
pub const MFSU_CAPACITY: i64 = 100_000_000;
pub const MFSU_RATE: i64 = 8_000;
pub const FURNACE_USE: i64 = 50;
pub const FURNACE_TICKS: i32 = 100;
pub const CRUSHER_USE: i64 = 80;
pub const CRUSHER_TICKS: i32 = 200;
pub const MACHINE_BUFFER: i64 = 8_000;
/// Mekanism's heat generator: furnace fuels or lava, a bit more than coal.
pub const HEAT_OUTPUT: i64 = 80;
/// A wind generator's most, high up under open sky.
pub const WIND_MAX_OUTPUT: i64 = 40;
/// Energy cubes: capacity and rate per tier (basic, advanced, elite, ultimate).
pub const CUBES: [(i64, i64); 4] = [(2_000_000, 800), (8_000_000, 3_200), (32_000_000, 12_800), (128_000_000, 51_200)];

/// Table-driven machines (Ender IO on): a processing machine's id, energy
/// per tick, ticks per item and recipe type.
pub const PROCESSORS: [(&str, i64, i32, &str); 23] = [
    ("voltz:alloy_smelter", 80, 100, "voltz:alloy_smelting"),
    ("voltz:sag_mill", 80, 120, "voltz:sag_milling"),
    ("voltz:slice_and_splice", 120, 200, "voltz:slicing"),
    ("voltz:soul_binder", 150, 200, "voltz:soul_binding"),
    // Atomic Science
    ("voltz:chemical_extractor", 200, 200, "voltz:chemical_extracting"),
    ("voltz:centrifuge", 300, 300, "voltz:centrifuging"),
    // Tinkers' Construct
    ("voltz:smeltery", 60, 80, "voltz:smeltery_melting"),
    // Railcraft
    ("voltz:coke_oven", 30, 300, "voltz:coking"),
    ("voltz:blast_furnace", 100, 200, "voltz:blast_smelting"),
    // IndustrialCraft 2
    ("voltz:induction_furnace", 80, 40, "voltz:electric_smelting"),
    ("voltz:recycler", 40, 45, "voltz:recycling"),
    ("voltz:ore_washing_plant", 60, 200, "voltz:ore_washing"),
    ("voltz:thermal_centrifuge", 100, 300, "voltz:thermal_centrifuging"),
    ("voltz:metal_former", 60, 100, "voltz:metal_forming"),
    ("voltz:block_cutter", 40, 100, "voltz:sawing"),
    ("voltz:mass_fabricator", 500, 400, "voltz:fabricating"),
    // Mekanism
    ("voltz:purification_chamber", 120, 100, "voltz:purifying"),
    ("voltz:chemical_injection_chamber", 160, 100, "voltz:injecting"),
    ("voltz:precision_sawmill", 60, 100, "voltz:sawing"),
    // Applied Energistics 2
    ("voltz:inscriber", 80, 100, "voltz:inscribing"),
    ("voltz:charger", 60, 100, "voltz:charging"),
    ("voltz:grindstone", 20, 80, "voltz:grinding"),
    // Railcraft
    ("voltz:rolling_machine", 40, 100, "voltz:rolling"),
];
/// Storage blocks: id, capacity, rate.
pub const STORAGE: [(&str, i64, i64); 3] = [
    ("voltz:basic_capacitor_bank", 5_000_000, 1_000),
    ("voltz:capacitor_bank", 25_000_000, 5_000),
    ("voltz:vibrant_capacitor_bank", 100_000_000, 25_000),
];
/// What a fuel generator burns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fuel {
    Furnace,
    Lava,
    /// Fissile fuel cells: a reactor, which needs water beside it.
    Fissile,
}
/// Fuel generators: id, joules per tick, fuel.
pub const FUEL_GENERATORS: [(&str, i64, Fuel); 2] =
    [("voltz:stirling_generator", 40, Fuel::Furnace), ("voltz:fission_reactor", 600, Fuel::Fissile)];
/// A fuel cell runs a reactor this long (10 minutes).
pub const FISSILE_TICKS: i32 = 12_000;
/// Heat a reactor gains per tick without water beside it, and loses with it;
/// at `MELTDOWN_HEAT` it goes off as a nuclear blast.
pub const HEAT_GAIN: i32 = 2;
pub const MELTDOWN_HEAT: i32 = 1_200;
pub const FISSILE_CELL: &str = "voltz:fissile_fuel_cell";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `PROCESSORS[i]`.
    Proc(u8),
    /// `STORAGE[i]`.
    Store(u8),
    /// `FUEL_GENERATORS[i]`.
    FuelGen(u8),
    CoalGenerator,
    GeothermalGenerator,
    SolarPanel,
    BatteryBox,
    Mfe,
    Mfsu,
    ElectricFurnace,
    Crusher,
    Compressor,
    Extractor,
    HeatGenerator,
    WindGenerator,
    /// Energy cube tier 0..4.
    EnergyCube(u8),
    EnrichmentChamber,
    MekanismCrusher,
    EnergizedSmelter,
    OsmiumCompressor,
    MetallurgicInfuser,
    /// MFFS force field projector: buffers energy; the glue raises the field.
    Projector,
    /// BuildCraft quarry: buffers energy; the glue digs.
    Quarry,
    /// AE2 controller: draws its upkeep to keep the network online.
    MeController,
    /// IC2 nuclear reactor: the glue runs its grid and fills its buffer.
    Ic2Reactor,
    /// A turret: buffers energy; the glue shoots.
    Turret,
}

impl Kind {
    pub fn of(block: &str) -> Option<Kind> {
        use crate::ids::*;
        Some(match block {
            COAL_GENERATOR => Kind::CoalGenerator,
            GEOTHERMAL_GENERATOR => Kind::GeothermalGenerator,
            SOLAR_PANEL => Kind::SolarPanel,
            BATTERY_BOX => Kind::BatteryBox,
            MFE => Kind::Mfe,
            MFSU => Kind::Mfsu,
            ELECTRIC_FURNACE => Kind::ElectricFurnace,
            CRUSHER => Kind::Crusher,
            COMPRESSOR => Kind::Compressor,
            EXTRACTOR => Kind::Extractor,
            HEAT_GENERATOR => Kind::HeatGenerator,
            WIND_GENERATOR => Kind::WindGenerator,
            BASIC_ENERGY_CUBE => Kind::EnergyCube(0),
            ADVANCED_ENERGY_CUBE => Kind::EnergyCube(1),
            ELITE_ENERGY_CUBE => Kind::EnergyCube(2),
            ULTIMATE_ENERGY_CUBE => Kind::EnergyCube(3),
            ENRICHMENT_CHAMBER => Kind::EnrichmentChamber,
            MEKANISM_CRUSHER => Kind::MekanismCrusher,
            ENERGIZED_SMELTER => Kind::EnergizedSmelter,
            OSMIUM_COMPRESSOR => Kind::OsmiumCompressor,
            METALLURGIC_INFUSER => Kind::MetallurgicInfuser,
            crate::forcefield::PROJECTOR => Kind::Projector,
            b if crate::buildcraft::is_quarry(b) => Kind::Quarry,
            crate::ae2::CONTROLLER => Kind::MeController,
            crate::ic2reactor::REACTOR => Kind::Ic2Reactor,
            b if crate::weapons::turret(b).is_some() => Kind::Turret,
            b => {
                if let Some(i) = PROCESSORS.iter().position(|p| p.0 == b) {
                    Kind::Proc(i as u8)
                } else if let Some(i) = STORAGE.iter().position(|p| p.0 == b) {
                    Kind::Store(i as u8)
                } else if let Some(i) = FUEL_GENERATORS.iter().position(|p| p.0 == b) {
                    Kind::FuelGen(i as u8)
                } else {
                    return None;
                }
            }
        })
    }
    pub fn is_generator(self) -> bool {
        matches!(
            self,
            Kind::CoalGenerator
                | Kind::GeothermalGenerator
                | Kind::SolarPanel
                | Kind::HeatGenerator
                | Kind::WindGenerator
                | Kind::FuelGen(_)
                | Kind::Ic2Reactor
        )
    }
    pub fn is_reactor(self) -> bool {
        matches!(self, Kind::FuelGen(i) if FUEL_GENERATORS[i as usize].2 == Fuel::Fissile)
    }
    pub fn is_storage(self) -> bool {
        matches!(self, Kind::BatteryBox | Kind::Mfe | Kind::Mfsu | Kind::EnergyCube(_) | Kind::Store(_))
    }
    /// Whether it has the furnace menu's three slots (solar and wind have none).
    pub fn has_slots(self) -> bool {
        !matches!(self, Kind::SolarPanel | Kind::WindGenerator | Kind::Quarry | Kind::MeController | Kind::Ic2Reactor | Kind::Turret)
    }
    pub fn capacity(self) -> i64 {
        match self {
            Kind::CoalGenerator | Kind::GeothermalGenerator | Kind::HeatGenerator | Kind::FuelGen(_) => GENERATOR_BUFFER,
            Kind::Store(i) => STORAGE[i as usize].1,
            Kind::Projector => 50_000,
            Kind::Quarry => 20_000,
            Kind::MeController => 10_000,
            Kind::Ic2Reactor => 1_000_000,
            Kind::Turret => 50_000,
            Kind::SolarPanel | Kind::WindGenerator => 1_000,
            Kind::EnergyCube(t) => CUBES[t.min(3) as usize].0,
            Kind::BatteryBox => BATTERY_CAPACITY,
            Kind::Mfe => MFE_CAPACITY,
            Kind::Mfsu => MFSU_CAPACITY,
            _ => MACHINE_BUFFER,
        }
    }
    /// What a storage block moves in or out in one tick.
    pub fn rate(self) -> i64 {
        match self {
            Kind::BatteryBox => BATTERY_RATE,
            Kind::Mfe => MFE_RATE,
            Kind::Mfsu => MFSU_RATE,
            Kind::EnergyCube(t) => CUBES[t.min(3) as usize].1,
            Kind::Store(i) => STORAGE[i as usize].2,
            _ => i64::MAX,
        }
    }
    /// Energy per tick and ticks per item of a processing machine.
    pub fn process(self) -> Option<(i64, i32)> {
        match self {
            Kind::ElectricFurnace => Some((FURNACE_USE, FURNACE_TICKS)),
            Kind::Crusher | Kind::Compressor | Kind::Extractor => Some((CRUSHER_USE, CRUSHER_TICKS)),
            // Mekanism's machines: quicker, a little hungrier.
            Kind::EnrichmentChamber | Kind::MekanismCrusher | Kind::OsmiumCompressor => Some((100, 100)),
            Kind::EnergizedSmelter => Some((120, 50)),
            Kind::MetallurgicInfuser => Some((100, 100)),
            Kind::Proc(i) => Some((PROCESSORS[i as usize].1, PROCESSORS[i as usize].2)),
            _ => None,
        }
    }
    /// The recipe type a processing machine reads.
    pub fn recipe_type(self) -> Option<&'static str> {
        match self {
            Kind::ElectricFurnace => Some("voltz:electric_smelting"),
            Kind::Crusher => Some("voltz:crushing"),
            Kind::Compressor => Some("voltz:compressing"),
            Kind::Extractor => Some("voltz:extracting"),
            Kind::EnrichmentChamber => Some("voltz:enriching"),
            Kind::MekanismCrusher => Some("voltz:mekanism_crushing"),
            Kind::EnergizedSmelter => Some("voltz:electric_smelting"),
            Kind::OsmiumCompressor => Some("voltz:osmium_compressing"),
            Kind::MetallurgicInfuser => Some("voltz:infusing"),
            Kind::Proc(i) => Some(PROCESSORS[i as usize].3),
            _ => None,
        }
    }
    /// What it may take from the network in one tick.
    pub fn accepts(self, s: &State) -> i64 {
        if self.is_generator() {
            return 0;
        }
        (self.capacity() - s.energy).max(0).min(self.rate())
    }
    /// What it offers the network this tick: generators all they hold,
    /// storage up to its rate.
    pub fn offers(self, s: &State) -> i64 {
        if self.is_generator() {
            s.energy
        } else if self.is_storage() {
            s.energy.min(self.rate())
        } else {
            0
        }
    }
    /// Whether a source of this kind gives to `other` (storage never feeds
    /// storage, or two would pass the same joules back and forth).
    pub fn feeds(self, other: Kind) -> bool {
        !(other.is_generator() || (self.is_storage() && other.is_storage()))
    }
}

/// A machine's saved state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub energy: i64,
    /// Ticks of fuel left (generator) or of work done (processors).
    pub progress: i32,
    pub progress_total: i32,
    /// A reactor's heat (0 for everything else).
    pub heat: i32,
}

impl State {
    pub fn fields(&self) -> [(&'static str, i64); 4] {
        [
            ("energy", self.energy),
            ("progress", self.progress as i64),
            ("progress_total", self.progress_total as i64),
            ("heat", self.heat as i64),
        ]
    }
    pub fn load(get: impl Fn(&str) -> Option<i64>) -> State {
        State {
            energy: get("energy").unwrap_or(0).max(0),
            progress: get("progress").unwrap_or(0).clamp(0, i32::MAX as i64) as i32,
            progress_total: get("progress_total").unwrap_or(0).clamp(0, i32::MAX as i64) as i32,
            heat: get("heat").unwrap_or(0).clamp(0, i32::MAX as i64) as i32,
        }
    }
}

/// Burn time in ticks of a generator fuel, as vanilla's furnace burns it.
pub fn burn_ticks(item: &str) -> i32 {
    match item {
        "minecraft:coal" | "minecraft:charcoal" => 1600,
        "minecraft:coal_block" => 16000,
        "minecraft:blaze_rod" => 2400,
        "minecraft:lava_bucket" => 20000,
        "minecraft:dried_kelp_block" => 4001,
        _ if item.ends_with("_log") || item.ends_with("_planks") || item.ends_with("_wood") => 300,
        "minecraft:stick" => 100,
        _ => 0,
    }
}

/// What the tick asks the glue to do besides saving the state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Effects {
    /// Take one item from these slots.
    pub consume: Vec<usize>,
    /// Put this into the output slot (merging with what is there).
    pub produce: Option<(String, u32)>,
    /// The block's `lit` property (furnace templates have it).
    pub lit: bool,
    /// Give back a bucket in the fuel slot (a lava bucket burned).
    pub bucket: bool,
    /// A reactor overheated: it goes off as a nuclear blast.
    pub meltdown: bool,
}

/// What the world around a machine offers it this tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Surroundings {
    /// Open sky above and daylight, no rain (solar panels).
    pub sunlit: bool,
    /// Open sky above (wind generators).
    pub sky: bool,
    /// The block's height (wind generators: more wind higher up).
    pub height: i32,
    /// Water touches it (a reactor stays cool).
    pub cooled: bool,
}

/// A wind generator's output at `height` under open sky: nothing at sea
/// level, the most from y 200.
pub fn wind_output(height: i32, sky: bool) -> i64 {
    if !sky {
        return 0;
    }
    ((height - 64).clamp(0, 136) as i64 * WIND_MAX_OUTPUT) / 136
}

/// One tick of a machine. `slots` are its three furnace-menu slots;
/// `recipes` the ones of its type; `max_stack` the output item's stack size.
pub fn tick(kind: Kind, s: &mut State, slots: &[Stack; 3], recipes: &[Recipe], max_stack: impl Fn(&str) -> u32) -> Effects {
    tick_in(kind, s, slots, recipes, max_stack, Surroundings::default())
}

pub fn tick_in(
    kind: Kind,
    s: &mut State,
    slots: &[Stack; 3],
    recipes: &[Recipe],
    max_stack: impl Fn(&str) -> u32,
    around: Surroundings,
) -> Effects {
    let mut fx = Effects::default();
    match kind {
        Kind::CoalGenerator | Kind::GeothermalGenerator | Kind::HeatGenerator | Kind::FuelGen(_) => {
            let (output, fuel_ticks): (i64, fn(&str) -> i32) = match kind {
                Kind::CoalGenerator => (GENERATOR_OUTPUT, burn_ticks),
                Kind::FuelGen(i) => match FUEL_GENERATORS[i as usize].2 {
                    Fuel::Lava => {
                        (FUEL_GENERATORS[i as usize].1, |i| if i == "minecraft:lava_bucket" { GEOTHERMAL_TICKS } else { 0 })
                    }
                    Fuel::Fissile => (FUEL_GENERATORS[i as usize].1, |i| if i == FISSILE_CELL { FISSILE_TICKS } else { 0 }),
                    Fuel::Furnace => (FUEL_GENERATORS[i as usize].1, burn_ticks),
                },
                Kind::HeatGenerator => (HEAT_OUTPUT, |i| if i == "minecraft:lava_bucket" { GEOTHERMAL_TICKS } else { burn_ticks(i) }),
                _ => (GEOTHERMAL_OUTPUT, |i| if i == "minecraft:lava_bucket" { GEOTHERMAL_TICKS } else { 0 }),
            };
            if s.progress == 0 && s.energy + output <= kind.capacity() {
                if let Some((fuel, n)) = &slots[FUEL] {
                    let t = fuel_ticks(fuel);
                    if t > 0 && *n > 0 {
                        s.progress = t;
                        s.progress_total = t;
                        fx.consume.push(FUEL);
                        fx.bucket = fuel == "minecraft:lava_bucket";
                    }
                }
            }
            if s.progress > 0 {
                s.progress -= 1;
                s.energy = (s.energy + output).min(kind.capacity());
                fx.lit = true;
                if s.progress == 0 && kind.is_reactor() {
                    fx.produce = Some(("voltz:toxic_waste".to_string(), 1));
                }
            }
            if kind.is_reactor() {
                let burning = s.progress > 0;
                s.heat = if burning && !around.cooled {
                    s.heat + HEAT_GAIN
                } else {
                    (s.heat - HEAT_GAIN * 2).max(0)
                };
                fx.meltdown = s.heat >= MELTDOWN_HEAT;
            }
        }
        Kind::SolarPanel => {
            if around.sunlit {
                s.energy = (s.energy + SOLAR_OUTPUT).min(kind.capacity());
                fx.lit = true;
            }
        }
        Kind::WindGenerator => {
            let out = wind_output(around.height, around.sky);
            if out > 0 {
                s.energy = (s.energy + out).min(kind.capacity());
                fx.lit = true;
            }
        }
        k if k.is_storage() => {
            fx.lit = s.energy > 0;
        }
        Kind::Projector | Kind::Quarry | Kind::Ic2Reactor | Kind::Turret => {}
        Kind::MeController => {
            if s.energy >= crate::ae2::CONTROLLER_UPKEEP {
                s.energy -= crate::ae2::CONTROLLER_UPKEEP;
                fx.lit = true;
            }
        }
        _ => {
            let (use_per_tick, total) = kind.process().unwrap_or((CRUSHER_USE, CRUSHER_TICKS));
            let recipe = recipes::find_with(recipes, &slots[INPUT], &slots[FUEL])
                .filter(|r| recipes::fits(&slots[OUTPUT], r, max_stack(&r.output)));
            match recipe {
                None => {
                    s.progress = 0;
                    s.progress_total = 0;
                }
                Some(r) => {
                    s.progress_total = total;
                    if s.energy >= use_per_tick {
                        s.energy -= use_per_tick;
                        s.progress += 1;
                        fx.lit = true;
                        if s.progress >= total {
                            s.progress = 0;
                            fx.consume.push(INPUT);
                            if r.secondary.is_some() {
                                fx.consume.push(FUEL);
                            }
                            fx.produce = Some((r.output.clone(), r.count));
                        }
                    }
                }
            }
        }
    }
    fx
}

/// The furnace menu's four data slots for this machine:
/// lit time, lit duration, cooking progress, cooking total.
pub fn menu_data(kind: Kind, s: &State) -> [i32; 4] {
    // The flame shows energy (or fuel left for a generator) out of 1000.
    let charge = |e: i64, cap: i64| ((e * 1000) / cap.max(1)) as i32;
    match kind {
        Kind::CoalGenerator | Kind::GeothermalGenerator | Kind::HeatGenerator | Kind::FuelGen(_) => {
            [s.progress, s.progress_total.max(1), charge(s.energy, kind.capacity()), 1000]
        }
        k if k.is_storage() || !k.has_slots() || matches!(k, Kind::Projector | Kind::Quarry | Kind::MeController) => [charge(s.energy, k.capacity()), 1000, 0, 0],
        _ => [charge(s.energy, kind.capacity()), 1000, s.progress, s.progress_total],
    }
}

/// "12.5 kJ" for a tooltip or the action bar.
pub fn joules(e: i64) -> String {
    match e {
        e if e >= 1_000_000 => format!("{:.2} MJ", e as f64 / 1e6),
        e if e >= 1_000 => format!("{:.1} kJ", e as f64 / 1e3),
        e => format!("{e} J"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(id: &str, n: u32) -> Stack {
        Some((id.to_string(), n))
    }

    #[test]
    fn generator_burns_coal_into_energy() {
        let mut s = State::default();
        let fx = tick(Kind::CoalGenerator, &mut s, &[None, stack("minecraft:coal", 2), None], &[], |_| 64);
        assert_eq!(fx.consume, vec![FUEL]);
        assert!(fx.lit);
        assert_eq!(s.energy, GENERATOR_OUTPUT);
        assert_eq!(s.progress, 1599);
        // It doesn't take more fuel while burning.
        let fx = tick(Kind::CoalGenerator, &mut s, &[None, stack("minecraft:coal", 1), None], &[], |_| 64);
        assert!(fx.consume.is_empty());
    }

    #[test]
    fn generator_waits_when_full() {
        let mut s = State { energy: GENERATOR_BUFFER, ..Default::default() };
        let fx = tick(Kind::CoalGenerator, &mut s, &[None, stack("minecraft:coal", 1), None], &[], |_| 64);
        assert!(fx.consume.is_empty());
        assert!(!fx.lit);
    }

    #[test]
    fn crusher_needs_power_and_room() {
        let rs = vec![Recipe { input: "minecraft:iron_ore".into(), output: "voltz:iron_dust".into(), count: 2, secondary: None }];
        let slots = [stack("minecraft:iron_ore", 1), None, None];
        let mut s = State::default();
        let fx = tick(Kind::Crusher, &mut s, &slots, &rs, |_| 64);
        assert_eq!(s.progress, 0);
        assert!(!fx.lit);
        s.energy = CRUSHER_USE * CRUSHER_TICKS as i64;
        let mut done = None;
        for _ in 0..CRUSHER_TICKS {
            let fx = tick(Kind::Crusher, &mut s, &slots, &rs, |_| 64);
            if fx.produce.is_some() {
                done = Some(fx);
            }
        }
        let fx = done.expect("crushed");
        assert_eq!(fx.produce, Some(("voltz:iron_dust".into(), 2)));
        assert_eq!(fx.consume, vec![INPUT]);
        assert_eq!(s.energy, 0);
        // Full output: no progress.
        let full = [stack("minecraft:iron_ore", 1), None, stack("voltz:iron_dust", 63)];
        s.energy = 1000;
        tick(Kind::Crusher, &mut s, &full, &rs, |_| 64);
        assert_eq!(s.progress, 0);
    }

    #[test]
    fn geothermal_burns_lava_and_returns_the_bucket() {
        let mut s = State::default();
        let fx = tick(Kind::GeothermalGenerator, &mut s, &[None, stack("minecraft:lava_bucket", 1), None], &[], |_| 64);
        assert!(fx.bucket && fx.lit);
        assert_eq!(s.energy, GEOTHERMAL_OUTPUT);
        let mut s = State::default();
        let fx = tick(Kind::GeothermalGenerator, &mut s, &[None, stack("minecraft:coal", 1), None], &[], |_| 64);
        assert!(fx.consume.is_empty());
    }

    #[test]
    fn solar_needs_sun() {
        let mut s = State::default();
        tick_in(Kind::SolarPanel, &mut s, &[None, None, None], &[], |_| 64, Surroundings { sunlit: false, ..Default::default() });
        assert_eq!(s.energy, 0);
        tick_in(Kind::SolarPanel, &mut s, &[None, None, None], &[], |_| 64, Surroundings { sunlit: true, ..Default::default() });
        assert_eq!(s.energy, SOLAR_OUTPUT);
    }

    #[test]
    fn storage_tiers_rate_limit() {
        let s = State { energy: MFSU_CAPACITY, ..Default::default() };
        assert_eq!(Kind::Mfsu.offers(&s), MFSU_RATE);
        assert_eq!(Kind::Mfe.accepts(&State::default()), MFE_RATE);
        assert!(!Kind::Mfe.feeds(Kind::Mfsu));
        assert!(Kind::Mfsu.feeds(Kind::Compressor));
    }

    #[test]
    fn infuser_uses_up_both_inputs() {
        let rs = vec![recipes::from_fields2("iron_ingot", Some("redstone"), "voltz:enriched_alloy", None).unwrap()];
        let slots = [stack("minecraft:iron_ingot", 2), stack("minecraft:redstone", 5), None];
        let mut s = State { energy: 100 * 100, ..Default::default() };
        let mut last = Effects::default();
        for _ in 0..100 {
            last = tick(Kind::MetallurgicInfuser, &mut s, &slots, &rs, |_| 64);
        }
        assert_eq!(last.consume, vec![INPUT, FUEL]);
        assert_eq!(last.produce, Some(("voltz:enriched_alloy".into(), 1)));
        let none = [stack("minecraft:iron_ingot", 2), None, None];
        let mut s = State { energy: 10_000, ..Default::default() };
        tick(Kind::MetallurgicInfuser, &mut s, &none, &rs, |_| 64);
        assert_eq!(s.progress, 0);
    }

    #[test]
    fn wind_grows_with_height() {
        assert_eq!(wind_output(64, true), 0);
        assert_eq!(wind_output(200, true), WIND_MAX_OUTPUT);
        assert_eq!(wind_output(300, false), 0);
        assert!(wind_output(130, true) > 0);
    }

    #[test]
    fn cubes_are_storage() {
        let k = Kind::of(crate::ids::ULTIMATE_ENERGY_CUBE).unwrap();
        assert!(k.is_storage() && k.capacity() == 128_000_000 && k.rate() == 51_200);
    }

    #[test]
    fn table_machines_resolve() {
        assert_eq!(Kind::of("voltz:sag_mill"), Some(Kind::Proc(1)));
        assert_eq!(Kind::Proc(1).recipe_type(), Some("voltz:sag_milling"));
        let bank = Kind::of("voltz:vibrant_capacitor_bank").unwrap();
        assert!(bank.is_storage() && bank.capacity() == 100_000_000);
        let stirling = Kind::of("voltz:stirling_generator").unwrap();
        assert!(stirling.is_generator());
        let mut s = State::default();
        tick(stirling, &mut s, &[None, stack("minecraft:coal", 1), None], &[], |_| 64);
        assert_eq!(s.energy, 40);
        assert!(crate::ids::is_machine("voltz:alloy_smelter"));
    }

    #[test]
    fn reactor_melts_down_without_water() {
        let k = Kind::of("voltz:fission_reactor").unwrap();
        assert!(k.is_reactor() && k.is_generator());
        let slots = [None, stack(FISSILE_CELL, 1), None];
        let mut s = State::default();
        let cool = Surroundings { cooled: true, ..Default::default() };
        for _ in 0..2_000 {
            assert!(!tick_in(k, &mut s, &slots, &[], |_| 64, cool).meltdown);
        }
        assert_eq!(s.heat, 0);
        let mut boom = false;
        for _ in 0..(MELTDOWN_HEAT / HEAT_GAIN + 1) {
            s.energy = 0; // drained by the network, so it keeps burning
            boom |= tick_in(k, &mut s, &slots, &[], |_| 64, Surroundings::default()).meltdown;
        }
        assert!(boom);
    }

    #[test]
    fn batteries_do_not_feed_batteries() {
        assert!(!Kind::BatteryBox.feeds(Kind::BatteryBox));
        assert!(Kind::CoalGenerator.feeds(Kind::BatteryBox));
        assert!(Kind::BatteryBox.feeds(Kind::Crusher));
        assert!(!Kind::CoalGenerator.feeds(Kind::CoalGenerator));
    }

    #[test]
    fn state_round_trips() {
        let s = State { energy: 1234, progress: 5, progress_total: 100, heat: 7 };
        let f = s.fields();
        let back = State::load(|k| f.iter().find(|(n, _)| *n == k).map(|(_, v)| *v));
        assert_eq!(back, s);
    }

    #[test]
    fn joules_reads_well() {
        assert_eq!(joules(950), "950 J");
        assert_eq!(joules(12_500), "12.5 kJ");
        assert_eq!(joules(1_000_000), "1.00 MJ");
    }
}
