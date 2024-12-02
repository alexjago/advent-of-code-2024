use std::fs::read_to_string;

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

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    // read the lists

    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for (l, r) in infile.split_whitespace().tuples() {
        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    }

    // sort the lists

    left.sort();
    right.sort();

    // sum of pairwise absolute differences
    let mut tot = 0;
    for k in 0..left.len() {
        tot += left[k].abs_diff(right[k])
    }

    tot
}
fn part_2(infile: &str) -> usize {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for (l, r) in infile.split_whitespace().tuples() {
        left.push(l.parse::<usize>().unwrap());
        right.push(r.parse::<usize>().unwrap());
    }

    let r_ctr = right.iter().counts();

    let mut tot = 0;

    for k in left {
        tot += k * r_ctr.get(&k).unwrap_or(&0);
    }

    tot
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"
";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), todo!());
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), todo!());
    }
}
