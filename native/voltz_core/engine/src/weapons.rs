// ICBM missiles and launchers, Open Modular Turrets, Techguns-style guns.
// Horizon has no mod entities, so these are made of what vanilla has:
// a missile is an item display flown along its arc by teleports each tick
// and set off at its target with the game's explosion; turrets hurt the
// nearest hostile mob with /damage; guns fire vanilla arrows and fireballs
// along the shooter's look. This module is the maths and the tables.

use crate::ids::*;

pub const LAUNCHER: &str = "voltz:missile_launcher";
pub const DESIGNATOR: &str = "voltz:target_designator";

/// Missile item -> the explosive it carries.
pub const MISSILES: [(&str, &str); 12] = [
    ("voltz:condensed_missile", CONDENSED_EXPLOSIVE),
    ("voltz:incendiary_missile", INCENDIARY_EXPLOSIVE),
    ("voltz:shrapnel_missile", SHRAPNEL_EXPLOSIVE),
    ("voltz:fragmentation_missile", FRAGMENTATION_EXPLOSIVE),
    ("voltz:thermobaric_missile", THERMOBARIC_EXPLOSIVE),
    ("voltz:exothermic_missile", EXOTHERMIC_EXPLOSIVE),
    ("voltz:endothermic_missile", ENDOTHERMIC_EXPLOSIVE),
    ("voltz:chemical_missile", CHEMICAL_EXPLOSIVE),
    ("voltz:debilitation_missile", DEBILITATION_EXPLOSIVE),
    ("voltz:emp_missile", EMP_EXPLOSIVE),
    ("voltz:anvil_missile", ANVIL_EXPLOSIVE),
    ("voltz:nuclear_missile", NUCLEAR_EXPLOSIVE),
];

pub fn warhead(missile: &str) -> Option<&'static str> {
    MISSILES.iter().find(|m| m.0 == missile).map(|m| m.1)
}

/// Flight time in ticks for a missile going `dist` blocks: about 3 blocks a
/// tick, at least 2 s.
pub fn flight_ticks(dist: f64) -> i32 {
    ((dist / 3.0) as i32).clamp(40, 600)
}

/// Where a missile is `t` ticks into a `total`-tick flight from `from` to
/// `to`: straight across, up and over an arc a third of the distance high.
pub fn missile_pos(from: [f64; 3], to: [f64; 3], t: i32, total: i32) -> [f64; 3] {
    let s = (t as f64 / total.max(1) as f64).clamp(0.0, 1.0);
    let dist = ((to[0] - from[0]).powi(2) + (to[2] - from[2]).powi(2)).sqrt();
    let arc = (dist / 3.0).max(20.0);
    [
        from[0] + (to[0] - from[0]) * s,
        from[1] + (to[1] - from[1]) * s + arc * 4.0 * s * (1.0 - s),
        from[2] + (to[2] - from[2]) * s,
    ]
}

/// The unit vector a player looks along (Minecraft's yaw and pitch, degrees).
pub fn look(yaw: f32, pitch: f32) -> [f64; 3] {
    let (y, p) = ((yaw as f64).to_radians(), (pitch as f64).to_radians());
    [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()]
}

/// A gun: id, projectile, speed (blocks/tick), damage, ammo item, ticks between shots.
pub struct Gun {
    pub id: &'static str,
    pub projectile: Projectile,
    pub speed: f64,
    pub damage: f64,
    pub ammo: &'static str,
    pub cooldown: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projectile {
    Arrow,
    /// An exploding fireball (power).
    Rocket(i32),
}

pub const GUNS: [Gun; 4] = [
    Gun { id: "voltz:pistol", projectile: Projectile::Arrow, speed: 3.5, damage: 6.0, ammo: "voltz:pistol_rounds", cooldown: 8 },
    Gun { id: "voltz:assault_rifle", projectile: Projectile::Arrow, speed: 4.5, damage: 8.0, ammo: "voltz:rifle_rounds", cooldown: 3 },
    Gun { id: "voltz:minigun", projectile: Projectile::Arrow, speed: 4.0, damage: 4.0, ammo: "voltz:minigun_drum", cooldown: 1 },
    Gun { id: "voltz:rocket_launcher", projectile: Projectile::Rocket(2), speed: 1.5, damage: 0.0, ammo: "voltz:rocket", cooldown: 30 },
];

pub fn gun(id: &str) -> Option<&'static Gun> {
    GUNS.iter().find(|g| g.id == id)
}

/// A turret: id, range, damage per shot, joules per shot, ticks between shots, damage type.
pub const TURRETS: [(&str, i32, f64, i64, i32, &str); 4] = [
    ("voltz:machine_gun_turret", 16, 4.0, 200, 10, "minecraft:arrow"),
    ("voltz:incendiary_turret", 12, 3.0, 400, 20, "minecraft:in_fire"),
    ("voltz:laser_turret", 24, 8.0, 1_000, 20, "minecraft:magic"),
    ("voltz:rocket_turret", 32, 14.0, 3_000, 60, "minecraft:explosion"),
];

pub fn turret(id: &str) -> Option<(i32, f64, i64, i32, &'static str)> {
    TURRETS.iter().find(|t| t.0 == id).map(|t| (t.1, t.2, t.3, t.4, t.5))
}

/// Everything a turret or a bullet leaves alone: players, things that aren't
/// mobs, and the peaceful mobs (Horizon's selectors don't take entity tags,
/// so this is a list).
pub const NOT_TARGETS: [&str; 62] = [
    "player", "item", "experience_orb", "arrow", "spectral_arrow", "trident", "item_display", "block_display",
    "text_display", "interaction", "armor_stand", "item_frame", "glow_item_frame", "painting", "minecart", "tnt",
    "falling_block", "fireball", "small_fireball", "snowball", "egg", "ender_pearl", "potion", "firework_rocket",
    "lightning_bolt", "marker", "leash_knot", "area_effect_cloud", "oak_boat", "spruce_boat", "birch_boat",
    "cow", "pig", "sheep", "chicken", "horse", "donkey", "mule", "llama", "trader_llama", "villager",
    "wandering_trader", "iron_golem", "snow_golem", "wolf", "cat", "ocelot", "parrot", "rabbit", "fox", "bee",
    "turtle", "axolotl", "frog", "goat", "camel", "sniffer", "armadillo", "allay", "mooshroom", "panda", "bat",
];

/// `@e[...]` for the nearest target within `range` (no leading `@e[`).
pub fn target_selector(range: f64) -> String {
    let mut s = String::from("@e[");
    for t in NOT_TARGETS {
        s.push_str("type=!minecraft:");
        s.push_str(t);
        s.push(',');
    }
    s.push_str(&format!("distance=..{range},sort=nearest,limit=1]"));
    s
}

/// Distances along a bullet's path where the glue looks for something to hit.
pub const BULLET_RANGE: i32 = 48;
pub const BULLET_STEP: i32 = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missiles_fly_an_arc_and_land() {
        let (a, b) = ([0.0, 64.0, 0.0], [300.0, 70.0, 0.0]);
        let total = flight_ticks(300.0);
        assert_eq!(total, 100);
        assert_eq!(missile_pos(a, b, 0, total), a);
        assert_eq!(missile_pos(a, b, total, total), b);
        let mid = missile_pos(a, b, total / 2, total);
        assert!(mid[1] > 160.0 && (mid[0] - 150.0).abs() < 1e-9);
        assert_eq!(warhead("voltz:nuclear_missile"), Some(NUCLEAR_EXPLOSIVE));
    }

    #[test]
    fn look_matches_minecraft() {
        let south = look(0.0, 0.0);
        assert!((south[2] - 1.0).abs() < 1e-9);
        let west = look(90.0, 0.0);
        assert!((west[0] + 1.0).abs() < 1e-9);
        let up = look(0.0, -90.0);
        assert!((up[1] - 1.0).abs() < 1e-9);
        assert!(gun("voltz:minigun").is_some() && turret("voltz:laser_turret").is_some());
        let sel = target_selector(16.0);
        assert!(sel.starts_with("@e[type=!minecraft:player,") && sel.ends_with("distance=..16,sort=nearest,limit=1]"));
    }
}
