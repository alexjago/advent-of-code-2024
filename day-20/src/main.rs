use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use itertools::Itertools;
use log::{debug, trace};
use mapgrid::*;
use nom;
use regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::RangeBounds;
use strum;

#[derive(Parser)]
pub struct Opts {
    /// Tell me more (or less)
    #[clap(flatten)]
    verbose: Verbosity<clap_verbosity_flag::InfoLevel>,
    /// Input file
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();
    env_logger::Builder::new()
        .filter_level(opts.verbose.log_level_filter())
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        .init();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

const DIRS: [Coord; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

/// distance to the goal
fn flood_fill(
    walls: &HashSet<Coord>,
    goal: Coord,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> HashMap<Coord, usize> {
    let mut visited = HashMap::new();

    let mut queue = BTreeSet::new();

    visited.insert(goal.clone(), 0_usize);
    queue.insert(goal.clone());

    while let Some(spot) = queue.pop_last() {
        let distance = *visited.get(&spot).unwrap();
        for d in DIRS {
            let next = add(d, spot);
            if next[0] >= min_x
                && next[0] <= max_x
                && next[1] >= min_y
                && next[1] <= max_y
                && (!walls.contains(&next))
                && (distance + 1 < *visited.get(&next).unwrap_or(&usize::MAX))
            {
                visited.insert(next, distance + 1);
                queue.insert(next);
            }
        }
    }
    visited
}

fn cheat_2_moves() -> HashSet<Coord> {
    let mut out = HashSet::new();
    for i in DIRS {
        for j in DIRS {
            out.insert(add(i, j));
        }
    }
    out
}

fn part_1(infile: &str) -> usize {
    // we're running a maze ([S]tart, [E]nd, `.` path, `#` wall)
    // 4-neighbours
    // exactly once in the run, we may glitch through walls for two moves
    // we want to know how many

    let grid: HashMap<Coord, char> = Grid::from_str_with(infile, |c| Some(c));

    let start_pos = grid
        .iter()
        .filter_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .next()
        .unwrap();
    let end_pos = grid
        .iter()
        .filter_map(|(k, v)| if *v == 'E' { Some(*k) } else { None })
        .next()
        .unwrap();

    let walls: HashSet<Coord> = grid
        .iter()
        .filter_map(|(k, c)| if *c == '#' { Some(*k) } else { None })
        .collect();

    let [xb, yb] = <HashMap<Coord, char> as Grid>::bounds(&grid);

    let [xmin, xmax] = [*xb.start(), *xb.end()];
    let [ymin, ymax] = [*yb.start(), *yb.end()];

    // first get the normal time

    let dists = flood_fill(&walls, end_pos, xmin, xmax, ymin, ymax);

    let no_cheats_time = dists.get(&start_pos).unwrap();

    trace!("No cheating time: {no_cheats_time}");
    trace!("{dists:?}");

    let moves = cheat_2_moves();

    trace!("2-move cheats: {moves:?}");

    let mut cheats: BTreeMap<(Coord, Coord), usize> = BTreeMap::new();

    for (pos, dist) in &dists {
        for m in &moves {
            let cheat = add(*pos, *m);

            if let Some(cheat_dist) = dists.get(&cheat) {
                if cheat_dist + 2 < *dist {
                    let saving = (dist - cheat_dist) - 2;
                    trace!("Cheat: start @{pos:?} ${dist} finish @{cheat:?} ${cheat_dist} saving ${saving}");
                    cheats.insert((*pos, cheat), saving);
                }
            }
        }
    }

    trace!("All Cheats:\n{cheats:?}");
    trace!("Grouped by savings:\n{:?}", cheats.values().counts());

    cheats.iter().filter(|(_, v)| **v >= 100).count()
}

/// Cheat by up to 20 moves (but at least two)
fn cheat_20_moves() -> HashSet<Coord> {
    let mut out = HashSet::new();

    for x in -20..=20_isize {
        for y in -20..=20_isize {
            let d: isize = x.abs() + y.abs();

            if d >= 2 && d <= 20 {
                out.insert([x, y]);
            }
        }
    }
    out
}

fn part_2(infile: &str) -> usize {
    // we're running a maze ([S]tart, [E]nd, `.` path, `#` wall)
    // 4-neighbours
    // exactly once in the run, we may glitch through walls for two moves
    // we want to know how many

    let grid: HashMap<Coord, char> = Grid::from_str_with(infile, |c| Some(c));

    let start_pos = grid
        .iter()
        .filter_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .next()
        .unwrap();
    let end_pos = grid
        .iter()
        .filter_map(|(k, v)| if *v == 'E' { Some(*k) } else { None })
        .next()
        .unwrap();

    let walls: HashSet<Coord> = grid
        .iter()
        .filter_map(|(k, c)| if *c == '#' { Some(*k) } else { None })
        .collect();

    let [xb, yb] = <HashMap<Coord, char> as Grid>::bounds(&grid);

    let [xmin, xmax] = [*xb.start(), *xb.end()];
    let [ymin, ymax] = [*yb.start(), *yb.end()];

    // first get the normal time

    let dists = flood_fill(&walls, end_pos, xmin, xmax, ymin, ymax);

    let no_cheats_time = dists.get(&start_pos).unwrap();

    trace!("No cheating time: {no_cheats_time}");
    trace!("{dists:?}");

    let moves = cheat_20_moves();

    trace!("20-move cheats: {moves:?}");

    let mut cheats: BTreeMap<(Coord, Coord), usize> = BTreeMap::new();

    for (pos, dist) in &dists {
        for m in &moves {
            let cheat = add(*pos, *m);

            if let Some(cheat_dist) = dists.get(&cheat) {
                let cost = (m[0].abs() + m[1].abs()) as usize;
                if cheat_dist + cost < *dist {
                    let saving = (dist - cheat_dist) - cost;
                    trace!("Cheat: start @{pos:?} ${dist} finish @{cheat:?} ${cheat_dist} saving ${saving}");
                    cheats.insert((*pos, cheat), saving);
                }
            }
        }
    }

    trace!("All Cheats:\n{cheats:?}");
    debug!(
        "Cheats, grouped by savings:\n{:?}",
        cheats.values().filter(|v| **v >= 50).counts()
    );

    cheats.iter().filter(|(_, v)| **v >= 100).count()
}

#[cfg(test)]
mod test {
    use super::*;

    fn init(level: log::LevelFilter) {
        env_logger::Builder::new()
            .filter_level(level)
            .format_timestamp(None)
            .format_module_path(false)
            .format_target(false)
            .format_level(false)
            .is_test(true)
            .init();
    }

    const EXAMPLE_1: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part_1_example() {
        init(log::LevelFilter::Trace);
        assert_eq!(part_1(EXAMPLE_1), 0);
    }

    #[test]
    fn part_2_example() {
        init(log::LevelFilter::Debug);
        assert_eq!(part_2(EXAMPLE_1), 285);
    }
}
