use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
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

    println!("Part 1:\n{}", part_1(&infile, 25));
    println!("Part 2:\n{}", part_2(&infile, 75));

    Ok(())
}

fn part_1(infile: &str, blink_total: usize) -> usize {
    let mut stones: Vec<usize> = infile
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    println!("{stones:?}");

    for blinks in 1..=blink_total {
        let mut i = 0;

        while let Some(n) = stones.get(i) {
            if *n == 0 {
                stones[i] = 1;
            } else if n.ilog(10) % 2 == 1 {
                let digits = (n.ilog(10) + 1) / 2;
                let left = *n / 10_usize.pow(digits);
                let right = *n % 10_usize.pow(digits);
                stones[i] = left;
                stones.insert(i + 1, right);
                i += 1; // advance i one extra
            } else {
                stones[i] *= 2024;
            }
            i += 1;
        }
        if blinks <= 6 {
            println!("{blinks}:\t{:?}", stones);
        } else if blinks >= 30 && blinks % 5 == 0 {
            println!("{blinks}")
        }
    }

    stones.len()
}

fn part_2(infile: &str, blink_total: usize) -> usize {
    // {position: value}
    // I need an encoding for position that sorts lexicographically: 10 > 2
    // it has been pointed out to me that in the worst case I could have 2^75 splits
    // ... and I do not have 2^75 anything of RAM

    // OK so I need to map (stone value, number of blinks left) : total number of stones
    // maybe

    // whats the 1 -> 2024 cycle do?

    // 1, 2024, (20, 24), (2, 0, 2, 4), (4048, 1, 4048, 8096), (40, 48, 2024, 80, 96), (4, 0, 4, 8, 20, 24, 8, 0, 9, 6)
    // ... (8096, 1, 8096, 16192, 2, 0, 2, 4, 16192, 1, 18216, 12144)
    // .. oh geez

    // rolling our own memoization here!
    let mut lookup: HashMap<(usize, usize), usize> = HashMap::new();

    infile
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .map(|x| p2_helper(x, blink_total, &mut lookup))
        .sum()
}

fn p2_helper(value: usize, remaining: usize, lookup: &mut HashMap<(usize, usize), usize>) -> usize {
    //! Maps a value and a number of blinks remaining to the number of stones in the result
    //! Base case: no blinks remaining ==> one stone
    //! Memoized by `lookup`

    if remaining == 0 {
        return 1;
    }

    if let Some(rez) = lookup.get(&(value, remaining)) {
        return *rez;
    } else {
        let rez = if value == 0 {
            p2_helper(1, remaining - 1, lookup)
        } else if value.ilog(10) % 2 == 1 {
            let digits = value.ilog(10) + 1;
            let left = value / 10_usize.pow(digits / 2);
            let right = value % 10_usize.pow(digits / 2);
            p2_helper(left, remaining - 1, lookup) + p2_helper(right, remaining - 1, lookup)
        } else {
            p2_helper(value * 2024, remaining - 1, lookup)
        };

        lookup.insert((value, remaining), rez);
        return rez;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"125 17";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1, 25), 55312);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1, 25), 55312);
    }

    #[test]
    fn sort_test() {
        assert!(String::from("11") < String::from("2"))
    }
}
