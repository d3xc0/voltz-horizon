// IndustrialCraft 2's nuclear reactor: a 9x6 grid of components simulated
// once a second while a redstone signal reaches it, by IC2's rules:
//
// - A fuel rod of n cells (1, 2, 4) gives each cell 1 + n/2 pulses, plus
//   one for every neighbouring rod or neutron reflector. Each cell makes
//   5 EU/t per pulse and 2·p·(p+1) heat a second. Dual and quad rods are
//   IC2's 20 EU/t / 24 heat and 60 EU/t / 96 heat on their own.
// - A rod's heat goes in equal parts to its neighbours that hold heat
//   (vents, exchangers, coolant cells, condensators), or to the hull.
// - Vents shed heat; reactor vents pull it from the hull first; component
//   vents cool their neighbours; exchangers even out heat with their
//   neighbours and the hull. A component past its limit is destroyed.
// - Plating raises the hull's limit. A hull at its limit explodes.
// - Rods last 20 000 seconds, then turn into depleted rods.

pub const W: usize = 9;
pub const H: usize = 6;
pub const SLOTS: usize = W * H;
pub const HULL_MAX: i32 = 10_000;
pub const ROD_LIFE: i32 = 20_000;
/// Game ticks per reactor second.
pub const TICKS: i32 = 20;
/// Joules per EU (the coal generator's 60 J/t is IC2's 10 EU/t).
pub const JOULES_PER_EU: i64 = 6;
pub const REACTOR: &str = "voltz:nuclear_reactor";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// cells, mox
    Rod(i32, bool),
    /// durability (pulses it can reflect), 0 = unbreakable (iridium)
    Reflector(i32),
    /// max heat, self vent, hull pull
    Vent(i32, i32, i32),
    /// cools each neighbour by this much
    ComponentVent(i32),
    /// max heat, neighbour switch, hull switch
    Exchanger(i32, i32, i32),
    /// max heat (coolant cells, condensators)
    Store(i32),
    /// hull limit added
    Plating(i32),
}

pub fn part(id: &str) -> Option<Part> {
    let id = id.strip_prefix("voltz:")?;
    // The public build's own parts carry generic names; the private build's
    // ported ones keep their `ic2_` ids.
    let id = match id {
        "uranium_fuel_rod" => "fuelrod_uranium",
        "dual_uranium_fuel_rod" => "fuelrod_dualuranium",
        "quad_uranium_fuel_rod" => "fuelrod_quaduranium",
        "neutron_reflector" => "reactor_neutronreflector",
        "thick_neutron_reflector" => "reactor_thickneutronreflector",
        "reinforced_neutron_reflector" => "reactor_iridiumreflector",
        "heat_vent" => "reactor_heatvent",
        "advanced_heat_vent" => "reactor_advancedheatvent",
        "casing_heat_vent" => "reactor_reactorheatvent",
        "high_flow_heat_vent" => "reactor_overclockedheatvent",
        "spreading_heat_vent" => "reactor_componentheatvent",
        "heat_exchanger" => "reactor_heatexchanger",
        "advanced_heat_exchanger" => "reactor_advancedheatexchanger",
        "casing_heat_exchanger" => "reactor_reactorheatexchanger",
        "spreading_heat_exchanger" => "reactor_componentheatexchanger",
        "coolant_cell_10k" => "reactor_heatstorage",
        "coolant_cell_30k" => "reactor_triheatstorage",
        "coolant_cell_60k" => "reactor_hexheatstorage",
        "redstone_condenser" => "reactor_rshcondensator",
        "lapis_condenser" => "reactor_lzhcondensator",
        "reactor_plating" => "reactor_plating",
        "heat_plating" => "reactor_heatplating",
        "containment_plating" => "reactor_containmentplating",
        other => other.strip_prefix("ic2_")?,
    };
    Some(match id {
        "fuelrod_uranium" => Part::Rod(1, false),
        "fuelrod_dualuranium" => Part::Rod(2, false),
        "fuelrod_quaduranium" => Part::Rod(4, false),
        "fuelrod_mox" => Part::Rod(1, true),
        "fuelrod_dualmox" => Part::Rod(2, true),
        "fuelrod_quadmox" => Part::Rod(4, true),
        "reactor_neutronreflector" => Part::Reflector(30_000),
        "reactor_thickneutronreflector" => Part::Reflector(120_000),
        "reactor_iridiumreflector" => Part::Reflector(0),
        "reactor_heatvent" => Part::Vent(1_000, 6, 0),
        "reactor_advancedheatvent" => Part::Vent(1_000, 12, 0),
        "reactor_reactorheatvent" | "reactor_heatventreactor" => Part::Vent(1_000, 5, 5),
        "reactor_overclockedheatvent" => Part::Vent(1_000, 20, 36),
        "reactor_componentheatvent" => Part::ComponentVent(4),
        "reactor_heatexchanger" => Part::Exchanger(2_500, 12, 4),
        "reactor_advancedheatexchanger" => Part::Exchanger(10_000, 24, 8),
        "reactor_reactorheatexchanger" => Part::Exchanger(5_000, 0, 72),
        "reactor_componentheatexchanger" => Part::Exchanger(5_000, 36, 0),
        "reactor_heatstorage" => Part::Store(10_000),
        "reactor_triheatstorage" => Part::Store(30_000),
        "reactor_hexheatstorage" => Part::Store(60_000),
        "reactor_rshcondensator" => Part::Store(20_000),
        "reactor_lzhcondensator" => Part::Store(100_000),
        "reactor_plating" => Part::Plating(1_000),
        "reactor_heatplating" => Part::Plating(1_700),
        "reactor_containmentplating" => Part::Plating(500),
        _ => return None,
    })
}

/// What a spent rod becomes.
pub fn depleted(id: &str) -> Option<&'static str> {
    Some(match id {
        "voltz:ic2_fuelrod_uranium" => "voltz:ic2_nuclear_depleteduranium",
        "voltz:ic2_fuelrod_dualuranium" => "voltz:ic2_nuclear_depleteddualuranium",
        "voltz:ic2_fuelrod_quaduranium" => "voltz:ic2_nuclear_depletedquaduranium",
        "voltz:ic2_fuelrod_mox" => "voltz:ic2_nuclear_depletedmox",
        "voltz:ic2_fuelrod_dualmox" => "voltz:ic2_nuclear_depleteddualmox",
        "voltz:ic2_fuelrod_quadmox" => "voltz:ic2_nuclear_depletedquadmox",
        "voltz:uranium_fuel_rod" => "voltz:depleted_uranium_fuel_rod",
        "voltz:dual_uranium_fuel_rod" => "voltz:depleted_dual_uranium_fuel_rod",
        "voltz:quad_uranium_fuel_rod" => "voltz:depleted_quad_uranium_fuel_rod",
        _ => return None,
    })
}

/// One grid slot: its component, the heat it holds and the life it has used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub part: Part,
    pub heat: i32,
    pub used: i32,
}

impl Cell {
    fn max_heat(&self) -> i32 {
        match self.part {
            Part::Vent(m, ..) | Part::Exchanger(m, ..) | Part::Store(m) => m,
            _ => 0,
        }
    }
    fn holds_heat(&self) -> bool {
        self.max_heat() > 0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
    /// EU per game tick for the next reactor second.
    pub eu_per_tick: i64,
    /// Slots whose component burned up (emptied).
    pub destroyed: Vec<usize>,
    /// Slots whose rod is spent (replaced by its depleted rod).
    pub spent: Vec<usize>,
    pub meltdown: bool,
}

fn neighbours(i: usize) -> impl Iterator<Item = usize> {
    let (x, y) = ((i % W) as i32, (i / W) as i32);
    [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().filter_map(move |(dx, dy)| {
        let (nx, ny) = (x + dx, y + dy);
        (nx >= 0 && ny >= 0 && nx < W as i32 && ny < H as i32).then_some((ny * W as i32 + nx) as usize)
    })
}

pub fn hull_max(grid: &[Option<Cell>]) -> i32 {
    HULL_MAX
        + grid.iter().flatten().map(|c| if let Part::Plating(p) = c.part { p } else { 0 }).sum::<i32>()
}

/// One reactor second. `grid` (54 slots) and `hull` are changed in place.
pub fn step(grid: &mut [Option<Cell>], hull: &mut i32) -> Outcome {
    let mut out = Outcome::default();
    let mut eu = 0i64;
    // Fuel: pulses, energy, heat.
    for i in 0..grid.len().min(SLOTS) {
        let Some(Cell { part: Part::Rod(cells, mox), .. }) = grid[i] else { continue };
        let mut near = 0;
        for n in neighbours(i) {
            match grid[n].map(|c| c.part) {
                Some(Part::Rod(..)) => near += 1,
                Some(Part::Reflector(_)) => {
                    near += 1;
                    if let Some(c) = grid[n].as_mut() {
                        c.used += cells;
                    }
                }
                _ => {}
            }
        }
        let pulses = 1 + cells / 2 + near;
        let mut cell_eu = 5 * pulses as i64 * cells as i64;
        if mox {
            // MOX gives more the hotter the hull: up to 5x at the limit.
            cell_eu = cell_eu * (100 + 400 * (*hull).max(0) as i64 / HULL_MAX as i64) / 100;
        }
        eu += cell_eu;
        let heat = cells * 2 * pulses * (pulses + 1);
        let sinks: Vec<usize> = neighbours(i).filter(|n| grid[*n].is_some_and(|c| c.holds_heat())).collect();
        if sinks.is_empty() {
            *hull += heat;
        } else {
            let each = heat / sinks.len() as i32;
            let rest = heat - each * sinks.len() as i32;
            for (k, n) in sinks.iter().enumerate() {
                if let Some(c) = grid[*n].as_mut() {
                    c.heat += each + if k == 0 { rest } else { 0 };
                }
            }
        }
        if let Some(c) = grid[i].as_mut() {
            c.used += 1;
        }
    }
    // Exchangers even out heat with neighbours and the hull.
    for i in 0..grid.len().min(SLOTS) {
        let Some(Cell { part: Part::Exchanger(max, side, hull_rate), .. }) = grid[i] else { continue };
        let me = |g: &[Option<Cell>]| g[i].map_or(0, |c| c.heat);
        for n in neighbours(i) {
            let Some(other) = grid[n].filter(|c| c.holds_heat()) else { continue };
            // Move heat toward equal fill, at most `side`.
            let mine = me(grid) * 100 / max.max(1);
            let theirs = other.heat * 100 / other.max_heat().max(1);
            let flow = ((mine - theirs) * max / 200).clamp(-side, side);
            if let Some(c) = grid[i].as_mut() {
                c.heat -= flow;
            }
            if let Some(c) = grid[n].as_mut() {
                c.heat += flow;
            }
        }
        if hull_rate > 0 {
            let mine = me(grid) * 100 / max.max(1);
            let theirs = (*hull).max(0) * 100 / HULL_MAX;
            let flow = ((mine - theirs) * max / 200).clamp(-hull_rate, hull_rate);
            if let Some(c) = grid[i].as_mut() {
                c.heat -= flow;
            }
            *hull += flow;
        }
    }
    // Vents.
    for i in 0..grid.len().min(SLOTS) {
        match grid[i].map(|c| c.part) {
            Some(Part::Vent(_, own, pull)) => {
                let took = pull.min((*hull).max(0));
                *hull -= took;
                if let Some(c) = grid[i].as_mut() {
                    c.heat = (c.heat + took - own).max(0);
                }
            }
            Some(Part::ComponentVent(cool)) => {
                for n in neighbours(i) {
                    if let Some(c) = grid[n].as_mut().filter(|c| c.holds_heat()) {
                        c.heat = (c.heat - cool).max(0);
                    }
                }
            }
            _ => {}
        }
    }
    // Burned-up components, spent rods and reflectors, the hull.
    for i in 0..grid.len().min(SLOTS) {
        let Some(c) = grid[i] else { continue };
        match c.part {
            Part::Rod(..) if c.used >= ROD_LIFE => out.spent.push(i),
            Part::Reflector(life) if life > 0 && c.used >= life => out.destroyed.push(i),
            _ if c.holds_heat() && c.heat >= c.max_heat() => out.destroyed.push(i),
            _ => {}
        }
    }
    for i in &out.destroyed {
        grid[*i] = None;
    }
    *hull = (*hull).max(0);
    out.meltdown = *hull >= hull_max(grid);
    out.eu_per_tick = eu;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> Vec<Option<Cell>> {
        vec![None; SLOTS]
    }
    fn put(g: &mut [Option<Cell>], i: usize, id: &str) {
        g[i] = Some(Cell { part: part(id).unwrap(), heat: 0, used: 0 });
    }

    #[test]
    fn rods_match_ic2_on_their_own() {
        for (id, eu, heat) in [
            ("voltz:ic2_fuelrod_uranium", 5, 4),
            ("voltz:ic2_fuelrod_dualuranium", 20, 24),
            ("voltz:ic2_fuelrod_quaduranium", 60, 96),
        ] {
            let mut g = grid();
            put(&mut g, 20, id);
            let mut hull = 0;
            let o = step(&mut g, &mut hull);
            assert_eq!((o.eu_per_tick, hull), (eu, heat), "{id}");
        }
    }

    #[test]
    fn neighbouring_rods_pulse_each_other() {
        let mut g = grid();
        put(&mut g, 20, "voltz:ic2_fuelrod_uranium");
        put(&mut g, 21, "voltz:ic2_fuelrod_uranium");
        let mut hull = 0;
        let o = step(&mut g, &mut hull);
        // Each: 2 pulses -> 10 EU/t, 12 heat.
        assert_eq!((o.eu_per_tick, hull), (20, 24));
    }

    #[test]
    fn vents_carry_heat_away() {
        let mut g = grid();
        put(&mut g, 20, "voltz:ic2_fuelrod_uranium");
        put(&mut g, 21, "voltz:ic2_reactor_heatvent");
        let mut hull = 0;
        for _ in 0..100 {
            step(&mut g, &mut hull);
        }
        assert_eq!(hull, 0);
        assert_eq!(g[21].unwrap().heat, 0); // 4 in, 6 out
    }

    #[test]
    fn an_uncooled_quad_melts_down() {
        let mut g = grid();
        put(&mut g, 20, "voltz:ic2_fuelrod_quaduranium");
        let mut hull = 0;
        let mut seconds = 0;
        while !step(&mut g, &mut hull).meltdown {
            seconds += 1;
            assert!(seconds < 1_000);
        }
        assert_eq!(seconds, HULL_MAX / 96);
    }

    #[test]
    fn plating_raises_the_limit_and_rods_deplete() {
        let mut g = grid();
        put(&mut g, 0, "voltz:ic2_reactor_plating");
        assert_eq!(hull_max(&g), HULL_MAX + 1_000);
        put(&mut g, 20, "voltz:ic2_fuelrod_uranium");
        g[20].as_mut().unwrap().used = ROD_LIFE - 1;
        let mut hull = 0;
        assert_eq!(step(&mut g, &mut hull).spent, vec![20]);
        assert_eq!(depleted("voltz:ic2_fuelrod_uranium"), Some("voltz:ic2_nuclear_depleteduranium"));
    }

    #[test]
    fn public_names_are_the_same_parts() {
        assert_eq!(part("voltz:quad_uranium_fuel_rod"), part("voltz:ic2_fuelrod_quaduranium"));
        assert_eq!(part("voltz:casing_heat_vent"), Some(Part::Vent(1_000, 5, 5)));
        assert_eq!(part("voltz:spreading_heat_exchanger"), Some(Part::Exchanger(5_000, 36, 0)));
        assert_eq!(part("voltz:iron_plate"), None);
        assert_eq!(depleted("voltz:uranium_fuel_rod"), Some("voltz:depleted_uranium_fuel_rod"));
    }

    #[test]
    fn overheated_vents_burn_up() {
        let mut g = grid();
        put(&mut g, 20, "voltz:ic2_fuelrod_quaduranium");
        put(&mut g, 21, "voltz:ic2_reactor_heatvent");
        let mut hull = 0;
        let mut gone = false;
        for _ in 0..30 {
            gone |= step(&mut g, &mut hull).destroyed.contains(&21);
        }
        assert!(gone && g[21].is_none());
    }
}
