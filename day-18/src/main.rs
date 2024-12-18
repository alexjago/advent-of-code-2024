use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
use nom;
use regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use strum;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile, 1024, 70));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str, falls: usize, max: isize) -> usize {
    let walls: HashSet<Coord> = infile
        .lines()
        .take(falls)
        .flat_map(|s| s.split(","))
        .map(|s| s.parse().unwrap())
        .tuples()
        .map(|(x, y)| [x, y])
        .collect();

    println!("{}", <HashSet<Coord> as Grid>::visualise(&walls));

    let mut combine = HashMap::new();

    let path = simple_maze(&walls, [0, 0], [max, max], 0, max, 0, max);

    for k in walls {
        combine.insert(k, '#');
    }

    for k in &path {
        combine.insert(k.clone(), 'O');
    }
    println!("{}", <HashMap<Coord, char> as Grid>::visualise(&combine));

    path.len()
}
fn part_2(infile: &str) -> usize {
    todo!()
}

const DIRS: [Coord; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

fn simple_maze(
    walls: &HashSet<Coord>,
    start: Coord,
    end: Coord,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> Vec<Coord> {
    let mut out = vec![];

    let mut visited = HashMap::new();

    let mut queue = BTreeSet::new();

    visited.insert(start.clone(), 0_usize);
    queue.insert(start.clone());

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

    let mut here = end;

    loop {
        let here_dist = visited.get(&here).unwrap();

        for d in DIRS {
            let next = add(here, d);
            let next_dist = visited.get(&next).unwrap_or(&usize::MAX);

            if *next_dist == here_dist - 1 {
                here = next;
                out.push(next);
                break;
            }
        }
        if *here_dist == 1 {
            break;
        }
    }

    out.reverse();
    out
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1, 12, 6), 22);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
