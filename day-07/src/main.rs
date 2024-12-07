use std::{collections::VecDeque, fs::read_to_string, ops::AddAssign};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use memoize::memoize;
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Ops {
    Add,
    Mul,
    Concat,
}

#[memoize(Capacity:8192)]
fn eval_ops(mut ops: VecDeque<Ops>, mut vals: VecDeque<u128>) -> u128 {
    // println!("\t\t{ops:?}\n\t\t{vals:?}");

    let right = vals.pop_back().unwrap();

    if let Some(op) = ops.pop_back() {
        match op {
            Ops::Add => return eval_ops(ops, vals) + right,
            Ops::Mul => return eval_ops(ops, vals) * right,
            Ops::Concat => {
                let left = eval_ops(ops, vals);
                return left * 10_u128.pow(right.checked_ilog10().unwrap_or(0) + 1) + right;
            }
        }
    } else {
        return right;
    }
}

fn part_1(infile: &str) -> u128 {
    // before the colon: result
    // left to right, no precedence
    // no reordering

    infile
        .lines()
        .filter_map(|line| {
            let rez = line
                .split(":")
                .nth(0)
                .and_then(|x| x.parse::<u128>().ok())
                .unwrap();
            let vals: VecDeque<u128> = line
                .split(":")
                .nth(1)
                .and_then(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u128>().ok())
                        .collect()
                })
                .unwrap_or_else(VecDeque::new);

            // I think we can bruteforce part 1?
            // looks like there's 8 values per line and about 850 lines

            // println!("{line}\n{rez}\t{vals:?}");

            let mut out = None;

            for k in 0..(1 << (vals.len() - 1)) {
                let mut ops = VecDeque::new();
                let mut kk = k;
                for _ in 1..vals.len() {
                    match kk % 2 {
                        0 => {
                            ops.push_back(Ops::Add);
                        }
                        1 => {
                            ops.push_back(Ops::Mul);
                        }
                        _ => unimplemented!(),
                    };
                    kk = kk >> 1;
                }

                // println!("{rez}?\n\t{ops:?}\n\t{vals:?}");

                if rez == eval_ops(ops.clone(), vals.clone()) {
                    // println!("{rez} = \n{ops:?}\n{vals:?}");
                    out = Some(rez);
                    break;
                }
            }
            out
        })
        .sum()
}
fn part_2(infile: &str) -> u128 {
    infile
        .lines()
        .filter_map(|line| {
            let rez = line
                .split(":")
                .nth(0)
                .and_then(|x| x.parse::<u128>().ok())
                .unwrap();
            let vals: VecDeque<u128> = line
                .split(":")
                .nth(1)
                .and_then(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u128>().ok())
                        .collect()
                })
                .unwrap_or_else(VecDeque::new);

            let mut out = None;

            for k in 0..(3_u32.pow(vals.len() as u32 - 1)) {
                let mut ops = VecDeque::new();
                let mut kk = k;
                for _ in 1..vals.len() {
                    match kk % 3 {
                        0 => {
                            ops.push_back(Ops::Add);
                        }
                        1 => {
                            ops.push_back(Ops::Mul);
                        }
                        2 => {
                            ops.push_back(Ops::Concat);
                        }
                        _ => unimplemented!(),
                    };
                    kk = kk / 3;
                }

                if rez == eval_ops(ops.clone(), vals.clone()) {
                    // println!("{rez} = \n{ops:?}\n{vals:?}");
                    out = Some(rez);
                    break;
                }
            }
            out
        })
        .sum::<u128>()
    // + part_1(infile)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 3749);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 11387);
    }

    #[test]
    fn eval_ops_example_1() {
        assert_eq!(
            eval_ops(
                VecDeque::from(vec![Ops::Concat]),
                VecDeque::from(vec![1, 2])
            ),
            12
        );
    }

    #[test]
    fn eval_ops_example_2() {
        assert_eq!(
            eval_ops(
                VecDeque::from(vec![Ops::Add, Ops::Mul, Ops::Add]),
                VecDeque::from(vec![11, 6, 16, 20])
            ),
            292
        );
    }
}
