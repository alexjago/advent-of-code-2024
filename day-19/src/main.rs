use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use itertools::Itertools;
use log::{debug, info, trace, warn};
use mapgrid::*;
use nom;
use regex;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::AddAssign;
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

fn part_1(infile: &str) -> usize {
    /*!

        input:

        1. sequence of sequences of atoms (towels with stripes)
        2. sequence of sequences of atoms (orders of stripes)

    No limit on how many towels you can use


    This feels DFS-y? iterate through each choice, if you can match a prefix then recurse

    */

    let mut lines = infile.lines().filter(|s| s.len() > 0);

    let towels: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_owned)
        .collect();

    debug!("{towels:?}");

    let mut memo = HashMap::new();

    let mut total = 0;
    for (i, d) in lines.enumerate() {
        if part_1_helper(&towels, d, &mut memo) {
            total += 1;
        }
    }
    total
}

/// For each towel, check if
fn part_1_helper<'a>(
    towels: &[String],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    trace!("{design}");
    if memo.contains_key(design) {
        return *memo.get(design).unwrap();
    }
    for t in towels {
        if design == t {
            memo.insert(design, true);
            return true;
        } else if design.starts_with(t) {
            if part_1_helper(towels, &design[t.len()..], memo) {
                memo.insert(design, true);
                return true;
            }
        }
    }
    memo.insert(design, false);
    return false;
}

fn part_2(infile: &str) -> usize {
    /*!

        input:

        1. sequence of sequences of atoms (towels with stripes)
        2. sequence of sequences of atoms (orders of stripes)

    No limit on how many towels you can use


    This feels DFS-y? iterate through each choice, if you can match a prefix then recurse

    */

    let mut lines = infile.lines().filter(|s| s.len() > 0);

    let towels: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::to_owned)
        .collect();

    debug!("{towels:?}");

    let mut memo = HashMap::new();

    let mut total = 0;
    for (i, d) in lines.enumerate() {
        let rez = part_2_helper(&towels, d, &mut memo);

        debug!("{i}: +{rez}");

        total += rez;
    }
    debug!("{memo:?}");

    total
}

/// For each towel, check if
fn part_2_helper<'a>(
    towels: &[String],
    design: &'a str,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if memo.contains_key(design) {
        return *memo.get(design).unwrap();
    }
    let mut arrangements = 0;
    for t in towels {
        if design == t {
            arrangements += 1;
        } else if design.starts_with(t) {
            arrangements += part_2_helper(towels, &design[t.len()..], memo);
        }
    }
    memo.insert(design, arrangements);
    return arrangements;
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Trace)
            .format_timestamp(None)
            .format_module_path(false)
            .format_target(false)
            .format_level(false)
            .is_test(true)
            .try_init();
    }

    const EXAMPLE_1: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part_1_example() {
        init();
        assert_eq!(part_1(EXAMPLE_1), 6);
    }

    #[test]
    fn part_2_example() {
        init();
        assert_eq!(part_2(EXAMPLE_1), 16);
    }
}
