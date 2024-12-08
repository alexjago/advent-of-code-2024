use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeBounds,
};

use anyhow::Result;
use clap::Parser;
use itertools::{Itertools, Unique};
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
    // Nodes marked by [0-9][A-Z][a-z]
    // If two nodes of the same type are at
    // (X, Y) and (X + i, Y + j)
    // Then antinodes occur at
    //   (X + 2i, Y + 2j)
    //   (X - i, Y - j)
    // within the bounds of the map of course

    let grid = HashMap::<Coord, char>::from_str_with(infile, |x| {
        if x.is_digit(10) || x.is_ascii_alphabetic() {
            Some(x)
        } else {
            None
        }
    });

    // println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&grid));

    // need a reverse LUT

    let xmax = infile.lines().map(|x| x.len()).max().unwrap_or(0) as isize;
    let ymax = infile.lines().count() as isize;

    let mut lut: HashMap<char, Vec<Coord>> = HashMap::new();

    for (coords, cha) in grid {
        lut.entry(cha).or_insert(vec![]).push(coords);
    }

    let mut outpos: HashMap<Coord, char> = HashMap::new();

    for (cha, coords) in lut {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }

                let [ix, iy] = coords[i];
                let [jx, jy] = coords[j];

                let dx = ix - jx;
                let dy = iy - jy;

                let nx = ix + dx;
                let ny = iy + dy;

                // println!("{cha}: ({ix}, {iy}) & ({jx}, {jy}) => ({nx}, {ny})");

                if nx >= 0 && nx < xmax && ny >= 0 && ny < ymax {
                    outpos.insert([nx, ny], cha);
                }
            }
        }
    }

    // println!(
    //     "{}",
    //     <HashMap<Coord, char> as Grid<char>>::visualise(&outpos)
    // );

    // 290 was too high, probably because not unique
    // actually, because I had infile.len() rather than infile.lines().count()
    outpos.iter().map(|(k, _)| k).unique().count()
}

fn part_2(infile: &str) -> usize {
    // Nodes marked by [0-9][A-Z][a-z]
    // If two nodes of the same type are at
    // (X, Y) and (X + i, Y + j)
    // Then antinodes occur at
    //   (X + 2i, Y + 2j)
    //   (X - i, Y - j)
    // within the bounds of the map of course
    // now for part two they occur generally at (X + ki, Y + kj) for integer k

    let grid = HashMap::<Coord, char>::from_str_with(infile, |x| {
        if x.is_digit(10) || x.is_ascii_alphabetic() {
            Some(x)
        } else {
            None
        }
    });

    // println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&grid));

    // need a reverse LUT

    let xmax = infile.lines().map(|x| x.len()).max().unwrap_or(0) as isize;
    let ymax = infile.lines().count() as isize;

    let mut lut: HashMap<char, Vec<Coord>> = HashMap::new();

    for (coords, cha) in grid {
        lut.entry(cha).or_insert(vec![]).push(coords);
    }

    let mut outpos: HashMap<Coord, char> = HashMap::new();

    for (cha, coords) in lut {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }

                let [ix, iy] = coords[i];
                let [jx, jy] = coords[j];

                let dx = ix - jx;
                let dy = iy - jy;

                for k in 0..(xmax.max(ymax)) {
                    let nx = ix + dx * k;
                    let ny = iy + dy * k;

                    // println!("{cha}: ({ix}, {iy}) & ({jx}, {jy}) => ({nx}, {ny})");

                    if nx >= 0 && nx < xmax && ny >= 0 && ny < ymax {
                        outpos.insert([nx, ny], cha);
                    }
                }
            }
        }
    }

    // println!(
    //     "{}",
    //     <HashMap<Coord, char> as Grid<char>>::visualise(&outpos)
    // );

    // worked first try!
    outpos.iter().map(|(k, _)| k).unique().count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 14);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 34);
    }
}
