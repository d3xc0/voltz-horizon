// The energy network: machines joined by copper wire, or touching each other.
// There is no saved network: each tick a source walks the wire from its own
// position (bounded by `MAX_WIRES`) and shares what it offers among what it
// finds, so breaking a wire takes effect at once.

use std::collections::{HashSet, VecDeque};

pub type Pos = [i32; 3];

/// Wire blocks one walk may cross; a bigger network is cut off past it.
pub const MAX_WIRES: usize = 1024;

pub const FACES: [Pos; 6] = [[0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1], [-1, 0, 0], [1, 0, 0]];

fn add(p: Pos, d: Pos) -> Pos {
    [p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

/// What a block is, as far as the network cares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Node {
    Wire,
    Machine,
    Other,
}

/// The machines reachable from `start` (itself left out): its neighbours, and
/// the neighbours of every wire joined to it, in walk order.
pub fn reachable(start: Pos, mut node_at: impl FnMut(Pos) -> Node) -> Vec<Pos> {
    let mut seen: HashSet<Pos> = HashSet::from([start]);
    let mut queue: VecDeque<Pos> = VecDeque::from([start]);
    let mut machines = Vec::new();
    let mut wires = 0;
    while let Some(at) = queue.pop_front() {
        for d in FACES {
            let next = add(at, d);
            if !seen.insert(next) {
                continue;
            }
            match node_at(next) {
                Node::Machine => machines.push(next),
                Node::Wire if wires < MAX_WIRES => {
                    wires += 1;
                    queue.push_back(next);
                }
                _ => {}
            }
        }
    }
    machines
}

/// Shares `offer` joules among sinks that each take at most their `accepts`,
/// as evenly as they allow (a sink that wants less leaves its share to the
/// others). Returns what each sink took, in order; their sum is what left.
pub fn share(offer: i64, accepts: &[i64]) -> Vec<i64> {
    let mut taken = vec![0i64; accepts.len()];
    let mut left = offer.max(0);
    let mut open: Vec<usize> = (0..accepts.len()).filter(|&i| accepts[i] > 0).collect();
    while left > 0 && !open.is_empty() {
        let each = (left / open.len() as i64).max(1);
        let mut still = Vec::with_capacity(open.len());
        for &i in &open {
            if left == 0 {
                break;
            }
            let give = each.min(accepts[i] - taken[i]).min(left);
            taken[i] += give;
            left -= give;
            if taken[i] < accepts[i] {
                still.push(i);
            }
        }
        open = still;
    }
    taken
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn world(blocks: &[(Pos, Node)]) -> impl FnMut(Pos) -> Node {
        let m: HashMap<Pos, Node> = blocks.iter().copied().collect();
        move |p| *m.get(&p).unwrap_or(&Node::Other)
    }

    #[test]
    fn walks_wire_to_machines() {
        let w = world(&[
            ([1, 0, 0], Node::Wire),
            ([2, 0, 0], Node::Wire),
            ([3, 0, 0], Node::Machine),
            ([0, 1, 0], Node::Machine),
            ([2, 1, 0], Node::Machine),
        ]);
        let mut got = reachable([0, 0, 0], w);
        got.sort();
        assert_eq!(got, vec![[0, 1, 0], [2, 1, 0], [3, 0, 0]]);
    }

    #[test]
    fn machines_do_not_relay() {
        // A machine between the source and another machine does not carry power.
        let w = world(&[([1, 0, 0], Node::Machine), ([2, 0, 0], Node::Machine)]);
        assert_eq!(reachable([0, 0, 0], w), vec![[1, 0, 0]]);
    }

    #[test]
    fn wire_walk_is_bounded() {
        let mut calls = 0;
        let got = reachable([0, 0, 0], |p| {
            calls += 1;
            if p[1] == 0 && p[2] == 0 && p[0] > 0 { Node::Wire } else { Node::Other }
        });
        assert!(got.is_empty());
        assert!(calls < (MAX_WIRES + 2) * 6);
    }

    #[test]
    fn share_is_fair_and_capped() {
        assert_eq!(share(90, &[10, 100, 100]), vec![10, 40, 40]);
        assert_eq!(share(5, &[0, 0]), vec![0, 0]);
        assert_eq!(share(1000, &[10, 20]), vec![10, 20]);
        let t = share(7, &[5, 5, 5]);
        assert_eq!(t.iter().sum::<i64>(), 7);
        assert_eq!(share(-3, &[5]), vec![0]);
    }
}
