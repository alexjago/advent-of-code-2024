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
    let mut count = 0;

    for l in infile.lines() {
        let deltas: Vec<i32> = l
            .split_whitespace()
            .filter_map(|k| k.parse::<i32>().ok())
            .tuple_windows()
            .map(|(i, j)| j - i)
            .collect();

        // println!("{l}\n{deltas:?}");
        if deltas.iter().map(|x| x.abs()).all(|x| (x > 0) && (x < 4))
            && deltas.iter().map(|i| i.signum()).all_equal()
        {
            // println!("safe");
            count += 1;
        } else {
            // println!("unsafe");
        }
    }

    count
}
fn part_2(infile: &str) -> usize {
    let mut count = 0;
    for l in infile.lines() {
        let deltas: Vec<i32> = l
            .split_whitespace()
            .filter_map(|k| k.parse::<i32>().ok())
            .tuple_windows()
            .map(|(i, j)| j - i)
            .collect();

        if deltas.iter().map(|x| x.abs()).all(|x| (x > 0) && (x < 4))
            && deltas.iter().map(|i| i.signum()).all_equal()
        {
            count += 1;
        } else {
            // experimentally remove a level
            for k in 0..=deltas.len() {
                let deltwo: Vec<i32> = l
                    .split_whitespace()
                    .filter_map(|k| k.parse::<i32>().ok())
                    .enumerate()
                    .filter_map(|(i, x)| if i != k { Some(x) } else { None })
                    .tuple_windows()
                    .map(|(i, j)| j - i)
                    .collect();
                if deltwo.iter().map(|x| x.abs()).all(|x| (x > 0) && (x < 4))
                    && deltwo.iter().map(|i| i.signum()).all_equal()
                {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 2);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 4);
    }
}
