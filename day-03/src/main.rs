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
    // looking for mul(X,Y) where X, Y are 3-digit numbers

    // get matches for `mul\((\d\d?\d?),(\d\d?\d?)\)` regex

    let re = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();

    let mut tot = 0;

    for (_, [x, y]) in re.captures_iter(infile).map(|c| c.extract()) {
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();

        tot += x * y
    }
    tot
}
fn part_2(infile: &str) -> usize {
    // looking for mul(X,Y) where X, Y are 3-digit numbers
    // except we can also be enabled by `do()` or disabled by `don't()`

    let re = regex::Regex::new(r"(mul\((\d\d?\d?),(\d\d?\d?)\))|(do\(\))|(don't\(\))").unwrap();

    let mut on = true;
    let mut tot = 0;

    for k in re.captures_iter(infile) {
        // println!("{k:?}");

        let kk = k.get(0).unwrap().as_str();

        if kk.starts_with("mul(") {
            let x: usize = k.get(2).unwrap().as_str().parse().unwrap();
            let y: usize = k.get(3).unwrap().as_str().parse().unwrap();
            if on {
                tot += x * y;
            }
        } else if kk == "don't()" {
            on = false;
        } else if kk == "do()" {
            on = true;
        }
    }
    tot
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 161);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_2), 48);
    }
}
