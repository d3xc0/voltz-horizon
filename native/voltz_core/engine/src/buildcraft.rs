// BuildCraft-style item pipes and the quarry.
//
// Pipes: a wooden pipe pulls from the containers it touches every
// `PULL_TICKS` and sends the items to a container elsewhere on its pipe
// network (round robin). Transport pipes (cobblestone, stone, gold) only
// connect; a gold pipe anywhere in the network moves stacks of 4.
//
// Quarry: digs an 11x11 shaft under itself, layer by layer, one block per
// `QUARRY_TICKS` for `QUARRY_JOULES`, keeping what it digs in its own slots.

use std::collections::{HashSet, VecDeque};

use crate::energy::{Pos, FACES};

pub const WOODEN_PIPE: &str = "voltz:wooden_pipe";
pub const GOLD_PIPE: &str = "voltz:gold_pipe";
pub const PIPES: [&str; 4] = [WOODEN_PIPE, "voltz:cobblestone_pipe", "voltz:stone_pipe", GOLD_PIPE];
pub const QUARRY: &str = "voltz:quarry";
/// BuildCraft's mining well: a quarry one block wide.
pub const MINING_WELL: &str = "voltz:mining_well";

/// Mekanism's digital miner: a 31x31 quarry.
pub const DIGITAL_MINER: &str = "voltz:digital_miner";

pub fn is_quarry(block: &str) -> bool {
    matches!(block, QUARRY | MINING_WELL | DIGITAL_MINER)
}

/// How far a digger reaches to each side of its shaft.
pub fn quarry_radius(block: &str) -> i32 {
    match block {
        MINING_WELL => 0,
        DIGITAL_MINER => 15,
        _ => QUARRY_RADIUS,
    }
}

pub const PULL_TICKS: i32 = 10;
pub const MAX_PIPES: usize = 512;
pub const QUARRY_TICKS: i32 = 4;
pub const QUARRY_JOULES: i64 = 300;
pub const QUARRY_RADIUS: i32 = 5;
pub const QUARRY_SLOTS: usize = 27;

pub fn is_pipe(block: &str) -> bool {
    PIPES.contains(&block)
}

/// What a block is to a pipe walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Node {
    Pipe { gold: bool },
    Container,
    Other,
}

/// A container on the network and the pipe that touches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target {
    pub pos: Pos,
    pub pipe: Pos,
}

/// The containers a pipe network reaches from `start` (a pipe), the
/// container at `source` left out, and whether the network has a gold pipe.
pub fn targets(start: Pos, source: Pos, mut node_at: impl FnMut(Pos) -> Node) -> (Vec<Target>, bool) {
    let mut seen: HashSet<Pos> = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    let mut out = Vec::new();
    let mut gold = false;
    while let Some(at) = queue.pop_front() {
        for d in FACES {
            let next = [at[0] + d[0], at[1] + d[1], at[2] + d[2]];
            if next == source || !seen.insert(next) {
                continue;
            }
            match node_at(next) {
                Node::Pipe { gold: g } if seen.len() < MAX_PIPES => {
                    gold |= g;
                    queue.push_back(next);
                }
                Node::Container => out.push(Target { pos: next, pipe: at }),
                _ => {}
            }
        }
    }
    (out, gold)
}

/// The face index (down, up, north, south, west, east) of `to` as seen from
/// `from`, its neighbour.
pub fn face_toward(from: Pos, to: Pos) -> usize {
    let d = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    FACES.iter().position(|f| *f == d).unwrap_or(1)
}

/// The quarry's `cursor`-th block: rows of x within each layer, layers
/// downward from the one under the quarry. None once past the world's bottom.
pub fn quarry_target(quarry: Pos, cursor: i32, bottom: i32) -> Option<Pos> {
    quarry_target_r(quarry, cursor, bottom, QUARRY_RADIUS)
}

pub fn quarry_target_r(quarry: Pos, cursor: i32, bottom: i32, radius: i32) -> Option<Pos> {
    let side = radius * 2 + 1;
    let per_layer = side * side;
    let layer = cursor / per_layer;
    let y = quarry[1] - 1 - layer;
    if y < bottom {
        return None;
    }
    let i = cursor % per_layer;
    Some([quarry[0] - radius + i % side, y, quarry[2] - radius + i / side])
}

/// Blocks a quarry leaves where they are.
pub fn quarry_skips(block: &str) -> bool {
    matches!(
        block,
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air" | "minecraft:bedrock" | "minecraft:water"
            | "minecraft:lava" | "minecraft:barrier" | "minecraft:end_portal_frame" | "minecraft:reinforced_deepslate"
    ) || block.starts_with("voltz:")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn pipes_reach_other_containers() {
        // chest(0,0,0) - wooden(1,0,0) - gold(2,0,0) - furnace(3,0,0), and a chest on top of the gold pipe.
        let m: HashMap<Pos, Node> = [
            ([0, 0, 0], Node::Container),
            ([1, 0, 0], Node::Pipe { gold: false }),
            ([2, 0, 0], Node::Pipe { gold: true }),
            ([3, 0, 0], Node::Container),
            ([2, 1, 0], Node::Container),
        ]
        .into_iter()
        .collect();
        let (t, gold) = targets([1, 0, 0], [0, 0, 0], |p| *m.get(&p).unwrap_or(&Node::Other));
        assert!(gold);
        let mut got: Vec<Pos> = t.iter().map(|t| t.pos).collect();
        got.sort();
        assert_eq!(got, vec![[2, 1, 0], [3, 0, 0]]);
        assert_eq!(t.iter().find(|t| t.pos == [3, 0, 0]).unwrap().pipe, [2, 0, 0]);
        assert_eq!(face_toward([3, 0, 0], [2, 0, 0]), 4); // west
    }

    #[test]
    fn quarry_digs_down_in_rows() {
        let q = [100, 70, -20];
        assert_eq!(quarry_target(q, 0, -64), Some([95, 69, -25]));
        assert_eq!(quarry_target(q, 11, -64), Some([95, 69, -24]));
        assert_eq!(quarry_target(q, 121, -64), Some([95, 68, -25]));
        assert_eq!(quarry_target(q, 121 * 200, -64), None);
        assert!(quarry_skips("minecraft:bedrock") && !quarry_skips("minecraft:stone"));
        assert_eq!(quarry_target_r(q, 3, -64, 0), Some([100, 66, -20]));
    }
}
