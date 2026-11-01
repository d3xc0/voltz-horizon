// ICBM-style explosive blocks. A redstone signal arms one; it hisses for its
// fuse, then goes off. The glue removes the block, calls the game's
// explosion with `blast`, then carries out its `after` effect: commands run
// as the server console, or an EMP that drains the machines around it.

use crate::ids::*;

/// Ticks from the signal to the blast (vanilla TNT's 80).
pub const FUSE: i32 = 80;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blast {
    /// Explosion power (vanilla TNT is 4); 0 for no explosion at all.
    pub radius: f32,
    /// Sets fires as it goes off.
    pub fire: bool,
    /// Breaks blocks; a shrapnel charge only hurts what is near.
    pub breaks_blocks: bool,
    /// How many blasts, stacked downward one block apart (a breaching charge
    /// bores a shaft instead of a crater).
    pub depth: i32,
    pub after: After,
}

/// What a charge does besides its blast.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum After {
    None,
    /// Water to ice, lava to obsidian and frost on whatever is near.
    Freeze,
    /// Smoke and blindness.
    Smoke,
    /// Mob effects on everything in range: (effect, seconds, amplifier).
    Effects(&'static [(&'static str, i32, i32)]),
    /// Drains every machine within the radius.
    Emp,
    /// Anvils rain down around it.
    Anvils,
}

const fn b(radius: f32, fire: bool, breaks_blocks: bool, depth: i32, after: After) -> Blast {
    Blast { radius, fire, breaks_blocks, depth, after }
}

/// How far an effect reaches (Effects, Smoke, Freeze, Emp, Anvils).
pub const EFFECT_RANGE: i32 = 12;

pub fn blast(block: &str) -> Option<Blast> {
    Some(match block {
        CONDENSED_EXPLOSIVE => b(7.0, false, true, 1, After::None),
        INCENDIARY_EXPLOSIVE => b(5.0, true, true, 1, After::None),
        SHRAPNEL_EXPLOSIVE => b(8.0, false, false, 1, After::None),
        BREACHING_EXPLOSIVE => b(2.5, false, true, 6, After::None),
        FRAGMENTATION_EXPLOSIVE => b(10.0, false, false, 1, After::None),
        THERMOBARIC_EXPLOSIVE => b(14.0, false, true, 1, After::None),
        EXOTHERMIC_EXPLOSIVE => b(10.0, true, true, 1, After::None),
        ENDOTHERMIC_EXPLOSIVE => b(4.0, false, false, 1, After::Freeze),
        SONIC_EXPLOSIVE => b(9.0, false, true, 3, After::None),
        NUCLEAR_EXPLOSIVE => b(30.0, true, true, 1, After::Effects(&[("minecraft:wither", 30, 1), ("minecraft:nausea", 20, 0)])),
        SMOKE_EXPLOSIVE => b(0.0, false, false, 1, After::Smoke),
        CHEMICAL_EXPLOSIVE => b(2.0, false, false, 1, After::Effects(&[("minecraft:poison", 30, 2)])),
        DEBILITATION_EXPLOSIVE => b(2.0, false, false, 1, After::Effects(&[
            ("minecraft:slowness", 30, 2), ("minecraft:weakness", 30, 1), ("minecraft:nausea", 15, 0),
        ])),
        CONTAGIOUS_EXPLOSIVE => b(2.0, false, false, 1, After::Effects(&[("minecraft:wither", 20, 1), ("minecraft:hunger", 30, 2)])),
        EMP_EXPLOSIVE => b(0.0, false, false, 1, After::Emp),
        ANVIL_EXPLOSIVE => b(2.0, false, false, 1, After::Anvils),
        _ => return None,
    })
}

/// One tick of an explosive: `fuse` is its saved fuse (0 = not armed).
/// Returns the new fuse and whether it goes off now.
pub fn tick(fuse: i32, powered: bool) -> (i32, bool) {
    match fuse {
        0 if powered => (FUSE, false),
        0 => (0, false),
        1 => (0, true),
        f => (f - 1, false),
    }
}

/// Console commands that carry out `after` for a charge at `pos`.
pub fn commands(after: After, pos: [i32; 3]) -> Vec<String> {
    let [x, y, z] = pos;
    let r = EFFECT_RANGE;
    let at = format!("execute positioned {x} {y} {z} run");
    match after {
        After::None | After::Emp => Vec::new(),
        After::Effects(list) => list
            .iter()
            .map(|(e, secs, amp)| format!("{at} effect give @e[distance=..{r}] {e} {secs} {amp}"))
            .collect(),
        After::Smoke => vec![
            format!("particle minecraft:campfire_signal_smoke {x} {y} {z} 4 3 4 0.02 600 force"),
            format!("{at} effect give @e[distance=..{r}] minecraft:blindness 15 0"),
        ],
        After::Freeze => {
            let fill = |from: &str, to: &str| {
                format!("{at} fill ~-{r} ~-{r} ~-{r} ~{r} ~{r} ~{r} {to} replace {from}")
            };
            vec![
                fill("minecraft:water", "minecraft:ice"),
                fill("minecraft:lava", "minecraft:obsidian"),
                fill("minecraft:short_grass", "minecraft:snow"),
                format!("{at} effect give @e[distance=..{r}] minecraft:slowness 20 3"),
                format!("particle minecraft:snowflake {x} {y} {z} 6 3 6 0.05 800 force"),
            ]
        }
        After::Anvils => (-2..=2)
            .flat_map(|dx| (-2..=2).map(move |dz| (dx * 3, dz * 3)))
            .map(|(dx, dz)| {
                format!(
                    "summon minecraft:falling_block {} {} {} {{BlockState:{{Name:\"minecraft:anvil\"}},Time:1,HurtEntities:1b,FallHurtAmount:4f,FallHurtMax:40}}",
                    x + dx,
                    y + 25,
                    z + dz
                )
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arms_then_counts_down() {
        let (mut f, mut boom) = tick(0, false);
        assert_eq!((f, boom), (0, false));
        (f, boom) = tick(0, true);
        assert_eq!((f, boom), (FUSE, false));
        let mut ticks = 0;
        while !boom {
            (f, boom) = tick(f, false); // the signal may go away: still goes off
            ticks += 1;
        }
        assert_eq!(ticks, FUSE);
        assert_eq!(f, 0);
    }

    #[test]
    fn every_explosive_has_a_blast() {
        for e in EXPLOSIVES {
            assert!(blast(e).is_some(), "{e}");
        }
        assert!(blast(COAL_GENERATOR).is_none());
        assert!(!blast(SHRAPNEL_EXPLOSIVE).unwrap().breaks_blocks);
    }

    #[test]
    fn effect_commands_name_the_spot() {
        let c = commands(blast(CHEMICAL_EXPLOSIVE).unwrap().after, [1, 64, -3]);
        assert_eq!(c, vec!["execute positioned 1 64 -3 run effect give @e[distance=..12] minecraft:poison 30 2"]);
        assert_eq!(commands(After::Anvils, [0, 0, 0]).len(), 25);
        assert!(commands(After::Freeze, [0, 0, 0])[0].contains("replace minecraft:water"));
        assert!(commands(After::Emp, [0, 0, 0]).is_empty());
    }
}
