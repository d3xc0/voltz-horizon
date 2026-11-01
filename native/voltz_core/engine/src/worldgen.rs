// Ore veins for a freshly generated chunk. Horizon's content mods can't add
// features to vanilla's biomes, so the glue places these itself when a new
// overworld chunk loads: deterministic from the world seed and the chunk, so
// the same seed gives the same ores.

/// Veins per chunk, blocks per vein, and the height range (as Mekanism's
/// osmium: common, mid-depth).
pub const VEINS: u32 = 8;
pub const VEIN_SIZE: u32 = 7;
pub const MIN_Y: i32 = -48;
pub const MAX_Y: i32 = 64;

/// SplitMix64: small, fast, and good enough for ore placement.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// An ore the glue places: its stone and deepslate blocks, veins per chunk,
/// blocks per vein, height range and a salt that keeps its veins apart.
pub struct Ore {
    /// The level it generates in.
    pub dimension: &'static str,
    /// The blocks it replaces: the first becomes `stone`, the second `deepslate`.
    pub hosts: [&'static str; 2],
    pub stone: &'static str,
    pub deepslate: &'static str,
    pub veins: u32,
    pub size: u32,
    pub min_y: i32,
    pub max_y: i32,
    pub salt: u64,
}

const OVERWORLD: &str = "minecraft:overworld";
const STONES: [&str; 2] = ["minecraft:stone", "minecraft:deepslate"];

pub static ORES: [Ore; 10] = [
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: crate::ids::OSMIUM_ORE,
        deepslate: crate::ids::DEEPSLATE_OSMIUM_ORE,
        veins: VEINS,
        size: VEIN_SIZE,
        min_y: MIN_Y,
        max_y: MAX_Y,
        salt: 0x05A1_0000,
    },
    // Atomic Science's uranium: rare and deep.
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:uranium_ore",
        deepslate: "voltz:deepslate_uranium_ore",
        veins: 3,
        size: 4,
        min_y: -60,
        max_y: 16,
        salt: 0x0235_0238,
    },
    // IndustrialCraft 2's tin and lead, Applied Energistics' certus quartz.
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:tin_ore",
        deepslate: "voltz:deepslate_tin_ore",
        veins: 7,
        size: 6,
        min_y: -16,
        max_y: 80,
        salt: 0x0071_0000,
    },
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:lead_ore",
        deepslate: "voltz:deepslate_lead_ore",
        veins: 4,
        size: 5,
        min_y: -48,
        max_y: 40,
        salt: 0x00B2_0000,
    },
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:ae2_quartzore",
        deepslate: "voltz:ae2_quartzore",
        veins: 3,
        size: 5,
        min_y: -40,
        max_y: 60,
        salt: 0x0CE2_7005,
    },
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:silver_ore",
        deepslate: "voltz:deepslate_silver_ore",
        veins: 3,
        size: 5,
        min_y: -48,
        max_y: 32,
        salt: 0x5117_0000,
    },
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:nickel_ore",
        deepslate: "voltz:deepslate_nickel_ore",
        veins: 3,
        size: 5,
        min_y: -32,
        max_y: 48,
        salt: 0x0071_C4E1,
    },
    Ore {
        dimension: OVERWORLD,
        hosts: STONES,
        stone: "voltz:zinc_ore",
        deepslate: "voltz:deepslate_zinc_ore",
        veins: 5,
        size: 5,
        min_y: -16,
        max_y: 64,
        salt: 0x0002_1AC0,
    },
    // Tinkers' Construct: cobalt and ardite in the nether's netherrack.
    Ore {
        dimension: "minecraft:the_nether",
        hosts: ["minecraft:netherrack", "minecraft:netherrack"],
        stone: "voltz:cobalt_ore",
        deepslate: "voltz:cobalt_ore",
        veins: 4,
        size: 5,
        min_y: 10,
        max_y: 118,
        salt: 0xC0BA_1700,
    },
    Ore {
        dimension: "minecraft:the_nether",
        hosts: ["minecraft:netherrack", "minecraft:netherrack"],
        stone: "voltz:ardite_ore",
        deepslate: "voltz:ardite_ore",
        veins: 4,
        size: 5,
        min_y: 10,
        max_y: 118,
        salt: 0xA4D1_7E00,
    },
];

/// Candidate positions for osmium in chunk `[cx, cz]`.
pub fn osmium_positions(seed: i64, chunk: [i32; 2]) -> Vec<[i32; 3]> {
    positions(&ORES[0], seed, chunk)
}

/// Candidate positions for `ore` in chunk `[cx, cz]`; the glue replaces those
/// that are stone (or deepslate, with the deepslate ore).
pub fn positions(ore: &Ore, seed: i64, chunk: [i32; 2]) -> Vec<[i32; 3]> {
    let (veins, size, min_y, max_y) = (ore.veins, ore.size, ore.min_y, ore.max_y);
    let mut state = mix(seed as u64 ^ ((chunk[0] as u64) << 32) ^ (chunk[1] as u32 as u64) ^ ore.salt);
    let mut next = |n: u32| -> i32 {
        state = mix(state);
        (state % n as u64) as i32
    };
    let mut out = Vec::new();
    for _ in 0..veins {
        let mut p = [chunk[0] * 16 + next(16), min_y + next((max_y - min_y) as u32), chunk[1] * 16 + next(16)];
        for _ in 0..size {
            out.push(p);
            match next(6) {
                0 => p[0] += 1,
                1 => p[0] -= 1,
                2 => p[1] += 1,
                3 => p[1] -= 1,
                4 => p[2] += 1,
                _ => p[2] -= 1,
            }
            // Stay inside the chunk so a vein never touches an ungenerated one.
            p[0] = p[0].clamp(chunk[0] * 16, chunk[0] * 16 + 15);
            p[2] = p[2].clamp(chunk[1] * 16, chunk[1] * 16 + 15);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn veins_are_deterministic_and_inside_the_chunk() {
        let a = osmium_positions(42, [3, -2]);
        assert_eq!(a, osmium_positions(42, [3, -2]));
        assert_ne!(a, osmium_positions(43, [3, -2]));
        assert_eq!(a.len(), (VEINS * VEIN_SIZE) as usize);
        for p in &a {
            assert!((48..64).contains(&p[0]) && (-32..-16).contains(&p[2]), "{p:?}");
            assert!(p[1] >= MIN_Y - VEIN_SIZE as i32 && p[1] <= MAX_Y + VEIN_SIZE as i32);
        }
    }
}
