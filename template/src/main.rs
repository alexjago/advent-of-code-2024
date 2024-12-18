use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use env_logger;
use itertools::Itertools;
use log::{debug, info, warn};
use mapgrid::*;
use nom;
use regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::read_to_string;
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
        .init();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile));
    println!("Part 2:\n{}", part_2(&infile));

    Ok(())
}

fn part_1(infile: &str) -> usize {
    todo!()
}
fn part_2(infile: &str) -> usize {
    todo!()
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
