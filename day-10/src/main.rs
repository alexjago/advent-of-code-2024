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
    let grid: HashMap<Coord, isize> =
        <HashMap<Coord, isize> as Grid<isize>>::from_str_with(infile, |c| {
            c.to_digit(10).and_then(|x| Some(x as isize))
        });

    println!(
        "{}",
        <HashMap<Coord, isize> as Grid<isize>>::visualise(&grid)
    );

    let mut heads: HashMap<Coord, isize> = grid
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some((*k, 0)) } else { None })
        .collect();

    println!("{:?}", heads);

    for (head_coord, head_count) in heads.iter_mut() {
        let mut reachable: HashSet<Coord> = HashSet::new();
        let mut queue: Vec<Coord> = vec![head_coord.clone()];

        let dirs: [Coord; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        while let Some(this) = queue.pop() {
            let height = *grid.get(&this).unwrap();
            if height == 9 {
                reachable.insert(this.clone());
                continue;
            }

            for d in dirs {
                let next: Coord = [d[0] + this[0], d[1] + this[1]];

                if let Some(v) = grid.get(&next) {
                    if *v == height + 1 {
                        queue.push(next);
                    }
                }
            }
        }

        *head_count += reachable.len() as isize;
        println!("{:?}: {}", head_coord, head_count);
    }

    heads.values().sum::<isize>() as usize
}
fn part_2(infile: &str) -> usize {
    let grid: HashMap<Coord, isize> =
        <HashMap<Coord, isize> as Grid<isize>>::from_str_with(infile, |c| {
            c.to_digit(10).and_then(|x| Some(x as isize))
        });

    println!(
        "{}",
        <HashMap<Coord, isize> as Grid<isize>>::visualise(&grid)
    );

    let mut heads: HashMap<Coord, isize> = grid
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some((*k, 0)) } else { None })
        .collect();

    println!("{:?}", heads);

    for (head_coord, head_count) in heads.iter_mut() {
        // now we need to keep trail histories (plural!) for each location
        let mut reachable: HashSet<Coord> = HashSet::new();
        let mut queue: Vec<Coord> = vec![head_coord.clone()];

        let mut histories: HashMap<Coord, HashSet<Vec<Coord>>> = HashMap::new();

        histories.entry(*head_coord).or_default().insert(vec![]);

        let dirs: [Coord; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        while let Some(this) = queue.pop() {
            let height = *grid.get(&this).unwrap();
            if height == 9 {
                reachable.insert(this.clone());
                continue;
            }

            for d in dirs {
                let next: Coord = [d[0] + this[0], d[1] + this[1]];

                if let Some(v) = grid.get(&next) {
                    if *v == height + 1 {
                        queue.push(next);

                        let hhh = histories.entry(this).or_default().clone();

                        for hh in hhh.iter() {
                            let mut h = hh.clone();
                            h.push(this);
                            histories.entry(next.clone()).or_default().insert(h.clone());
                            // println!("start: {:?}\tfrom: {:?}\tto: {:?}", head_coord, this, next);
                        }
                    }
                }
            }
        }

        // println!("{head_coord:?}: {}", reachable.len());

        for r in reachable {
            if let Some(h) = histories.get(&r) {
                *head_count += h.len() as isize;
                // println!("{:?}", h);
            }
        }

        // println!("{:?}: {}", head_coord, head_count);
    }

    heads.values().sum::<isize>() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 36);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 81);
    }
}
