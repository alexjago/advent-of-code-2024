use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::{Coord, Grid};
use nom;
use regex;
use strum;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    let grid: HashMap<Coord, char> =
        <HashMap<Coord, char> as Grid<char>>::from_str_with(infile, |x| Some(x));

    // println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&grid));

    // Target: first found
    let mut regions_rev: HashMap<Coord, Coord> = HashMap::new();

    // not sure if we can have diagonal neighbours, omit for now
    let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    // flood fill over each starting position?
    for (coord, cha) in &grid {
        if regions_rev.contains_key(coord) {
            continue;
        } else {
            regions_rev.insert(coord.clone(), coord.clone());
        }

        let mut queue = vec![coord.clone()];

        while let Some(this) = queue.pop() {
            for d in dirs {
                let next = [this[0] + d[0], this[1] + d[1]];

                if let Some(nc) = grid.get(&next) {
                    if regions_rev.contains_key(&next) {
                        continue;
                    } else if cha == nc {
                        regions_rev.insert(next, coord.clone());
                        queue.push(next)
                    }
                }
            }
        }
    }

    let mut regions: HashMap<Coord, HashSet<Coord>> = HashMap::new();

    for (h, t) in regions_rev {
        regions.entry(t).or_default().insert(h);
    }

    // ok now that we have partitioned the space...

    let mut total = 0;
    for (root, plots) in regions {
        let mut sides = 0;
        for p in &plots {
            sides += 4 - dirs
                .iter()
                .map(|d| [p[0] + d[0], p[1] + d[1]])
                .filter(|n| plots.contains(n))
                .count();
        }
        // println!("{root:?}: {plots:?} has {sides} sides");
        total += plots.len() * sides;
    }
    total
}
fn part_2(infile: &str) -> usize {
    // now we want the number of sides of regions however long they may be!
    let grid: HashMap<Coord, char> =
        <HashMap<Coord, char> as Grid<char>>::from_str_with(infile, |x| Some(x));

    // Target: first found
    let mut regions_rev: HashMap<Coord, Coord> = HashMap::new();

    let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    for (coord, cha) in &grid {
        if regions_rev.contains_key(coord) {
            continue;
        } else {
            regions_rev.insert(coord.clone(), coord.clone());
        }

        let mut queue = vec![coord.clone()];

        while let Some(this) = queue.pop() {
            for d in dirs {
                let next = [this[0] + d[0], this[1] + d[1]];

                if let Some(nc) = grid.get(&next) {
                    if regions_rev.contains_key(&next) {
                        continue;
                    } else if cha == nc {
                        regions_rev.insert(next, coord.clone());
                        queue.push(next)
                    }
                }
            }
        }
    }

    let mut regions: HashMap<Coord, HashSet<Coord>> = HashMap::new();

    for (h, t) in regions_rev {
        regions.entry(t).or_default().insert(h);
    }
    // we can *merge edges* iff they neighbour (have an end point in common) and have the same orientation
    // we can imagine a (position, orientation) grid
    // where our plot centre (x, y) is 2* the coords of our plot

    let mut total = 0;
    for (root, plots) in regions {
        // yes, we can have duplicates here
        let mut edges: HashSet<(Coord, Coord)> = HashSet::new();
        for [x, y] in &plots {
            for [dx, dy] in &dirs {
                if plots.contains(&[x + dx, y + dy]) {
                    continue;
                } else {
                    // suppose I have a gap above me (-Y) = [0, -1]
                    // then I need an edge from my top left corner [-1, -1] to my top right corner [+1, -1]
                    // gap below me (+Y)
                    // need edge bottom right [+1, +1] to bottom left [-1, +1]
                    // suppose I have a gap to my right (+X) = [1, 0]
                    // then I need an edge from my top right corner [+1, -1] to my bottom right corner [+1, +1]
                    // gap to left (-X)
                    // need edge bottom left [-1, +1] to top left [-1, -1]

                    let [dfx, dfy, dtx, dty] = match [dx, dy] {
                        [0, -1] => [-1, -1, 1, -1], // -Y
                        [0, 1] => [1, 1, -1, 1],    // +Y
                        [1, 0] => [1, -1, 1, 1],    // +X
                        [-1, 0] => [-1, 1, -1, -1], // -X
                        _ => unimplemented!(),
                    };

                    let fx = 2 * x + dfx;
                    let tx = 2 * x + dtx;
                    let fy = 2 * y + dfy;
                    let ty = 2 * y + dty;

                    edges.insert(([fx, fy], [tx, ty]));
                }
            }
        }

        println!("{root:?} starts with {} edges", edges.len());

        println!("{}", <HashSet<Coord> as Grid<char>>::visualise(&plots));

        let mut corners: HashMap<Coord, char> = HashMap::new();
        for (k, v) in &edges {
            corners.insert(
                k.clone(),
                match [(v[0] - k[0]).signum(), (v[1] - k[1]).signum()] {
                    [0, 1] => 'v',  // down
                    [0, -1] => '^', // up,
                    [1, 0] => '>',  // right
                    [-1, 0] => '<', // left
                    _ => unimplemented!(),
                },
            );

            if !corners.contains_key(v) {
                corners.insert(v.clone(), '@');
            }
        }

        println!(
            "{}",
            <HashMap<Coord, char> as Grid<char>>::visualise(&corners)
        );

        let mut queue: Vec<(Coord, Coord)> =
            edges.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        while let Some((f, t)) = queue.pop() {
            let d = [(t[0] - f[0]).signum(), (t[1] - f[1]).signum()];
            let candidates: Vec<(Coord, Coord)> = edges
                .iter()
                .filter(|(nf, nt)| {
                    *nf == t && [(nt[0] - nf[0]).signum(), (nt[1] - nf[1]).signum()] == d
                })
                .cloned()
                .collect();

            for (nf, nt) in candidates {
                edges.insert((f, nt));
                queue.push((f, nt));
                edges.remove(&(f, t));
                edges.remove(&(nf, nt));
            }
        }

        println!(
            "region for {root:?} ({}) has {} edges",
            grid.get(&root).unwrap(),
            edges.len()
        );

        println!("{:?}", edges);

        println!("{}", <HashSet<Coord> as Grid<char>>::visualise(&plots));

        let mut corners: HashMap<Coord, char> = HashMap::new();
        for (k, v) in &edges {
            corners.insert(
                k.clone(),
                match [(v[0] - k[0]).signum(), (v[1] - k[1]).signum()] {
                    [0, 1] => 'v',  // down
                    [0, -1] => '^', // up,
                    [1, 0] => '>',  // right
                    [-1, 0] => '<', // left
                    _ => unimplemented!(),
                },
            );
            if !corners.contains_key(v) {
                corners.insert(v.clone(), '@');
            }
        }

        println!(
            "{}",
            <HashMap<Coord, char> as Grid<char>>::visualise(&corners)
        );

        total += plots.len() * edges.len();
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 1930);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 1206);
    }
}
