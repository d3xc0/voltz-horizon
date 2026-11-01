// MFFS-style force field projectors. A redstone-powered projector with
// energy raises a shell of force field blocks around itself (cube, sphere,
// wall or tube, from the module in its input slot); without power or
// energy the field drops. The shell is recomputed each time, so nothing has
// to be saved about which blocks are field.

pub const PROJECTOR: &str = "voltz:force_field_projector";
pub const FIELD: &str = "voltz:force_field";
pub const SCALE_MODULE: &str = "voltz:scale_module";

/// Joules per tick per 8 field blocks held up, and the most blocks one
/// projector places or removes in a tick (big fields grow over a few ticks).
pub const BLOCKS_PER_JOULE: i64 = 8;
pub const BLOCKS_PER_TICK: usize = 512;
pub const BASE_RADIUS: i32 = 4;
pub const MAX_RADIUS: i32 = 20;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Cube,
    Sphere,
    Wall,
    Tube,
}

impl Shape {
    pub fn of_module(item: &str) -> Option<Shape> {
        Some(match item {
            "voltz:cube_module" => Shape::Cube,
            "voltz:sphere_module" => Shape::Sphere,
            "voltz:wall_module" => Shape::Wall,
            "voltz:tube_module" => Shape::Tube,
            _ => return None,
        })
    }
}

/// The radius from the scale modules in the fuel slot.
pub fn radius(scale_modules: u32) -> i32 {
    (BASE_RADIUS + scale_modules as i32).min(MAX_RADIUS)
}

/// The field's blocks around `center` (the projector itself left out).
pub fn shell(shape: Shape, center: [i32; 3], r: i32) -> Vec<[i32; 3]> {
    let mut out = Vec::new();
    for dx in -r..=r {
        for dy in -r..=r {
            for dz in -r..=r {
                let on = match shape {
                    Shape::Cube => dx.abs() == r || dy.abs() == r || dz.abs() == r,
                    Shape::Sphere => {
                        let d2 = dx * dx + dy * dy + dz * dz;
                        d2 <= r * r && d2 > (r - 1) * (r - 1)
                    }
                    // A wall across x, one block in front (+z) of the projector.
                    Shape::Wall => dz == 1 && dy >= 0,
                    // A square tube along z: the cube's sides, open at both ends.
                    Shape::Tube => (dx.abs() == r || dy.abs() == r) && dz.abs() < r,
                };
                if on && (dx, dy, dz) != (0, 0, 0) {
                    out.push([center[0] + dx, center[1] + dy, center[2] + dz]);
                }
            }
        }
    }
    out
}

/// What holding `blocks` field blocks up costs per tick.
pub fn upkeep(blocks: usize) -> i64 {
    (blocks as i64 + BLOCKS_PER_JOULE - 1) / BLOCKS_PER_JOULE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shells_have_the_right_size() {
        assert_eq!(shell(Shape::Cube, [0, 0, 0], 1).len(), 26);
        let s = shell(Shape::Sphere, [0, 0, 0], 5);
        assert!(s.iter().all(|p| {
            let d2 = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
            d2 <= 25 && d2 > 16
        }));
        assert_eq!(shell(Shape::Wall, [0, 0, 0], 2).len(), 5 * 3);
        assert!(!shell(Shape::Tube, [0, 0, 0], 3).iter().any(|p| p[2].abs() == 3));
        assert_eq!(radius(100), MAX_RADIUS);
        assert_eq!(upkeep(9), 2);
        assert_eq!(Shape::of_module("voltz:sphere_module"), Some(Shape::Sphere));
    }
}
