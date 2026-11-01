// An Applied Energistics-style storage network. ME cable joins an ME
// controller (which needs energy), ME drives (whose storage cells set how
// many items the network holds), ME terminals (a chest menu onto everything
// stored) and ME import buses (which pull from the container they face).
// The items themselves live in the drives' saved compounds, by item id.

use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::energy::{Pos, FACES};

pub const CABLE: &str = "voltz:me_cable";
pub const CONTROLLER: &str = "voltz:me_controller";
pub const DRIVE: &str = "voltz:me_drive";
pub const TERMINAL: &str = "voltz:me_terminal";
pub const IMPORT_BUS: &str = "voltz:me_import_bus";

/// Joules per tick a controller needs to keep its network online.
pub const CONTROLLER_UPKEEP: i64 = 20;
pub const DRIVE_SLOTS: usize = 9;
pub const MAX_CABLES: usize = 1024;
/// A terminal page: 45 items, then a row of buttons.
pub const PAGE: usize = 45;
pub const PREV_SLOT: i32 = 45;
pub const INFO_SLOT: i32 = 49;
pub const NEXT_SLOT: i32 = 53;
pub const IMPORT_TICKS: i32 = 10;

/// Items a storage cell holds.
pub fn cell_capacity(item: &str) -> u64 {
    match item {
        "voltz:storage_cell_1k" => 1_024,
        "voltz:storage_cell_4k" => 4_096,
        "voltz:storage_cell_16k" => 16_384,
        "voltz:storage_cell_64k" => 65_536,
        _ => 0,
    }
}

pub fn is_device(block: &str) -> bool {
    matches!(block, CONTROLLER | DRIVE | TERMINAL | IMPORT_BUS)
}

/// The devices on the network `start` belongs to, by kind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Network {
    pub controllers: Vec<Pos>,
    pub drives: Vec<Pos>,
}

pub fn walk(start: Pos, mut block_at: impl FnMut(Pos) -> String) -> Network {
    let mut seen: HashSet<Pos> = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    let mut net = Network::default();
    let mut cables = 0;
    while let Some(at) = queue.pop_front() {
        for d in FACES {
            let next = [at[0] + d[0], at[1] + d[1], at[2] + d[2]];
            if !seen.insert(next) {
                continue;
            }
            let b = block_at(next);
            match b.as_str() {
                CABLE if cables < MAX_CABLES => {
                    cables += 1;
                    queue.push_back(next);
                }
                CONTROLLER => {
                    net.controllers.push(next);
                    queue.push_back(next);
                }
                DRIVE => {
                    net.drives.push(next);
                    queue.push_back(next);
                }
                TERMINAL | IMPORT_BUS => queue.push_back(next),
                _ => {}
            }
        }
    }
    net
}

/// One drive's items and capacity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Store {
    pub items: BTreeMap<String, u64>,
    pub capacity: u64,
}

impl Store {
    pub fn used(&self) -> u64 {
        self.items.values().sum()
    }
    pub fn room(&self) -> u64 {
        self.capacity.saturating_sub(self.used())
    }
}

/// Puts up to `n` of `item` into the drives with room, first first; returns
/// how many went in.
pub fn insert(stores: &mut [Store], item: &str, n: u64) -> u64 {
    let mut left = n;
    for s in stores.iter_mut() {
        let put = left.min(s.room());
        if put > 0 {
            *s.items.entry(item.to_string()).or_default() += put;
            left -= put;
        }
        if left == 0 {
            break;
        }
    }
    n - left
}

/// Takes up to `n` of `item` from the drives; returns how many came out.
pub fn take(stores: &mut [Store], item: &str, n: u64) -> u64 {
    let mut left = n;
    for s in stores.iter_mut() {
        if let Some(have) = s.items.get_mut(item) {
            let got = left.min(*have);
            *have -= got;
            left -= got;
            if *have == 0 {
                s.items.remove(item);
            }
        }
        if left == 0 {
            break;
        }
    }
    n - left
}

/// Everything stored, most first (then by id), merged over the drives.
pub fn listing(stores: &[Store]) -> Vec<(String, u64)> {
    let mut all: BTreeMap<&str, u64> = BTreeMap::new();
    for s in stores {
        for (k, v) in &s.items {
            *all.entry(k).or_default() += v;
        }
    }
    let mut out: Vec<(String, u64)> = all.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

pub fn pages(entries: usize) -> usize {
    entries.div_ceil(PAGE).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn network_walks_cable_and_devices() {
        let m: HashMap<Pos, &str> = [
            ([1, 0, 0], CABLE),
            ([2, 0, 0], DRIVE),
            ([3, 0, 0], CONTROLLER),
            ([1, 1, 0], CABLE),
            ([1, 2, 0], DRIVE),
            ([5, 0, 0], DRIVE), // not connected
        ]
        .into_iter()
        .collect();
        let net = walk([0, 0, 0], |p| m.get(&p).copied().unwrap_or("minecraft:air").to_string());
        assert_eq!(net.controllers, vec![[3, 0, 0]]);
        let mut d = net.drives.clone();
        d.sort();
        assert_eq!(d, vec![[1, 2, 0], [2, 0, 0]]);
    }

    #[test]
    fn stores_fill_in_order_and_list_merged() {
        let mut s = vec![Store { capacity: 10, ..Default::default() }, Store { capacity: 100, ..Default::default() }];
        assert_eq!(insert(&mut s, "minecraft:stone", 15), 15);
        assert_eq!(s[0].used(), 10);
        assert_eq!(insert(&mut s, "minecraft:dirt", 200), 95);
        assert_eq!(take(&mut s, "minecraft:stone", 12), 12);
        assert_eq!(take(&mut s, "minecraft:stone", 12), 3);
        assert_eq!(listing(&s), vec![("minecraft:dirt".to_string(), 95)]);
        assert_eq!(pages(0), 1);
        assert_eq!(pages(46), 2);
        assert_eq!(cell_capacity("voltz:storage_cell_4k"), 4096);
    }
}
