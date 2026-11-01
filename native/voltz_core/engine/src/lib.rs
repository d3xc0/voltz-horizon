// Voltz Core's game-independent logic: the energy network and the machines'
// state machines. The game glue (helpers compiled into horizon-client) reads a
// block entity's saved compound into these types, ticks them, and writes them
// back. Energy is in joules (J); a tick is 1/20 s, so 100 J/t is 2 kW.

pub mod ae2;
pub mod buildcraft;
pub mod energy;
pub mod explosives;
pub mod forcefield;
pub mod ic2reactor;
pub mod machines;
pub mod recipes;
pub mod weapons;
pub mod worldgen;

/// Block ids of the mod, so the glue and the tests agree on them.
pub mod ids {
    pub const NS: &str = "voltz";
    pub const WIRE: &str = "voltz:copper_wire";
    pub const COAL_GENERATOR: &str = "voltz:coal_generator";
    pub const BATTERY_BOX: &str = "voltz:battery_box";
    pub const ELECTRIC_FURNACE: &str = "voltz:electric_furnace";
    pub const CRUSHER: &str = "voltz:crusher";
    pub const GEOTHERMAL_GENERATOR: &str = "voltz:geothermal_generator";
    pub const SOLAR_PANEL: &str = "voltz:solar_panel";
    pub const MFE: &str = "voltz:mfe";
    pub const MFSU: &str = "voltz:mfsu";
    pub const COMPRESSOR: &str = "voltz:compressor";
    pub const EXTRACTOR: &str = "voltz:extractor";
    // Mekanism
    pub const HEAT_GENERATOR: &str = "voltz:heat_generator";
    pub const WIND_GENERATOR: &str = "voltz:wind_generator";
    pub const BASIC_ENERGY_CUBE: &str = "voltz:basic_energy_cube";
    pub const ADVANCED_ENERGY_CUBE: &str = "voltz:advanced_energy_cube";
    pub const ELITE_ENERGY_CUBE: &str = "voltz:elite_energy_cube";
    pub const ULTIMATE_ENERGY_CUBE: &str = "voltz:ultimate_energy_cube";
    pub const ENRICHMENT_CHAMBER: &str = "voltz:enrichment_chamber";
    pub const MEKANISM_CRUSHER: &str = "voltz:mekanism_crusher";
    pub const ENERGIZED_SMELTER: &str = "voltz:energized_smelter";
    pub const OSMIUM_COMPRESSOR: &str = "voltz:osmium_compressor";
    pub const METALLURGIC_INFUSER: &str = "voltz:metallurgic_infuser";

    pub const OSMIUM_ORE: &str = "voltz:osmium_ore";
    pub const DEEPSLATE_OSMIUM_ORE: &str = "voltz:deepslate_osmium_ore";

    /// Every block that holds a Voltz block entity and joins the network.
    pub const MACHINES: [&str; 21] = [
        COAL_GENERATOR, GEOTHERMAL_GENERATOR, SOLAR_PANEL, BATTERY_BOX, MFE, MFSU, ELECTRIC_FURNACE, CRUSHER,
        COMPRESSOR, EXTRACTOR, HEAT_GENERATOR, WIND_GENERATOR, BASIC_ENERGY_CUBE, ADVANCED_ENERGY_CUBE,
        ELITE_ENERGY_CUBE, ULTIMATE_ENERGY_CUBE, ENRICHMENT_CHAMBER, MEKANISM_CRUSHER, ENERGIZED_SMELTER,
        OSMIUM_COMPRESSOR, METALLURGIC_INFUSER,
    ];

    pub fn is_machine(block: &str) -> bool {
        MACHINES.contains(&block) || super::machines::Kind::of(block).is_some()
    }

    /// Blocks that carry power: IC2's copper cable and Ender IO's conduits.
    pub const WIRES: [&str; 4] = [WIRE, "voltz:energy_conduit", "voltz:enhanced_energy_conduit", "voltz:ender_energy_conduit"];

    pub fn is_wire(block: &str) -> bool {
        WIRES.contains(&block)
    }

    pub const CONDENSED_EXPLOSIVE: &str = "voltz:condensed_explosive";
    pub const INCENDIARY_EXPLOSIVE: &str = "voltz:incendiary_explosive";
    pub const SHRAPNEL_EXPLOSIVE: &str = "voltz:shrapnel_explosive";
    pub const BREACHING_EXPLOSIVE: &str = "voltz:breaching_explosive";

    pub const FRAGMENTATION_EXPLOSIVE: &str = "voltz:fragmentation_explosive";
    pub const THERMOBARIC_EXPLOSIVE: &str = "voltz:thermobaric_explosive";
    pub const EXOTHERMIC_EXPLOSIVE: &str = "voltz:exothermic_explosive";
    pub const ENDOTHERMIC_EXPLOSIVE: &str = "voltz:endothermic_explosive";
    pub const SONIC_EXPLOSIVE: &str = "voltz:sonic_explosive";
    pub const NUCLEAR_EXPLOSIVE: &str = "voltz:nuclear_explosive";
    pub const SMOKE_EXPLOSIVE: &str = "voltz:smoke_explosive";
    pub const CHEMICAL_EXPLOSIVE: &str = "voltz:chemical_explosive";
    pub const DEBILITATION_EXPLOSIVE: &str = "voltz:debilitation_explosive";
    pub const CONTAGIOUS_EXPLOSIVE: &str = "voltz:contagious_explosive";
    pub const EMP_EXPLOSIVE: &str = "voltz:emp_explosive";
    pub const ANVIL_EXPLOSIVE: &str = "voltz:anvil_explosive";

    pub const EXPLOSIVES: [&str; 16] = [
        CONDENSED_EXPLOSIVE, INCENDIARY_EXPLOSIVE, SHRAPNEL_EXPLOSIVE, BREACHING_EXPLOSIVE, FRAGMENTATION_EXPLOSIVE,
        THERMOBARIC_EXPLOSIVE, EXOTHERMIC_EXPLOSIVE, ENDOTHERMIC_EXPLOSIVE, SONIC_EXPLOSIVE, NUCLEAR_EXPLOSIVE,
        SMOKE_EXPLOSIVE, CHEMICAL_EXPLOSIVE, DEBILITATION_EXPLOSIVE, CONTAGIOUS_EXPLOSIVE, EMP_EXPLOSIVE, ANVIL_EXPLOSIVE,
    ];

    pub fn is_explosive(block: &str) -> bool {
        EXPLOSIVES.contains(&block)
    }

    /// Iron Chests: id and slot count (vanilla's menus top out at 54).
    pub const CHESTS: [(&str, usize); 6] = [
        ("voltz:copper_chest", 45),
        ("voltz:iron_chest", 54),
        ("voltz:gold_chest", 54),
        ("voltz:diamond_chest", 54),
        ("voltz:crystal_chest", 54),
        ("voltz:obsidian_chest", 54),
    ];

    pub fn chest_slots(block: &str) -> Option<usize> {
        CHESTS.iter().find(|c| c.0 == block).map(|c| c.1)
    }

    /// Blocks that get a Voltz block entity: machines, explosives, chests,
    /// force field projectors.
    pub fn has_block_entity(block: &str) -> bool {
        is_machine(block)
            || is_explosive(block)
            || chest_slots(block).is_some()
            || block == super::forcefield::PROJECTOR
            || block == super::buildcraft::WOODEN_PIPE
            || super::buildcraft::is_quarry(block)
            || block == super::ae2::DRIVE
            || block == super::ae2::IMPORT_BUS
            || block == super::weapons::LAUNCHER
    }

    /// Blocks whose block entity ticks (chests just hold items).
    pub fn ticks(block: &str) -> bool {
        is_machine(block)
            || is_explosive(block)
            || block == super::forcefield::PROJECTOR
            || block == super::buildcraft::WOODEN_PIPE
            || super::buildcraft::is_quarry(block)
            || block == super::ae2::IMPORT_BUS
            || block == super::weapons::LAUNCHER
    }
}
