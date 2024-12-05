use std::{
    collections::{hash_set::Iter, HashSet},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use nom;
use regex;
use strum;

use std::cmp::Ordering::*;

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
    // input's in two sections
    // first section A|B :: A toposorts? before B (or at least A < B)
    // second section A, B, C...: page numbers of each update
    // need to identify which updates (second section) are in the right order

    // strats: we can build a LUT of first section.
    // If we have (B, A) but the first section contains (A, B) then we can reject
    // (no reject if pair not in LUT?)

    // flavour text suggests we exhaustively try each pair in second section

    // actual problem: sum of middle-page numbers of correct updates

    let mut correct_updates = vec![];

    let mut lookups: HashSet<(usize, usize)> = HashSet::new();

    for l in infile.lines() {
        if let Some((a, b)) = l
            .split('|')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect_tuple()
        {
            lookups.insert((a, b));
        }
    }

    let mut maxlen = 0;

    'line: for l in infile.lines() {
        let v: Vec<usize> = l
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        if v.len() == 0 {
            continue 'line;
        }

        if v.len() > maxlen {
            maxlen = v.len();
        }

        for i in 0..v.len() {
            let a = v[i];
            for j in i..v.len() {
                let b = v[j];

                if lookups.contains(&(b, a)) {
                    continue 'line;
                }
            }
        }
        correct_updates.push(v);
    }

    println!("max update length: {maxlen}");

    let mut out = 0;

    for k in correct_updates {
        out += k[k.len() / 2]
    }
    out
}
fn part_2(infile: &str) -> usize {
    // Now we must re-order the incorrectly ordered ones, and add up *those* middle numbers

    // this really looks like a toposort over the first section, and then select where matching in the second section

    // the maximum update length is 23 which is annoyingly many to brute force re-order

    // ughhhh I really don't want to write a toposort

    let mut lookups: HashSet<(usize, usize)> = HashSet::new();

    let mut updates = vec![];

    let mut out = 0;

    for l in infile.lines() {
        if let Some((a, b)) = l
            .split('|')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect_tuple()
        {
            lookups.insert((a, b));
        }

        let v: Vec<usize> = l
            .split(',')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect();

        if v.len() > 0 {
            updates.push(v);
        }
    }

    // I suppose we don't really need to write a toposort after all

    for u in updates {
        let mut uu = u.clone();

        uu.sort_by(|a, b| -> std::cmp::Ordering {
            if lookups.contains(&(*a, *b)) {
                Less
            } else if lookups.contains(&(*b, *a)) {
                Greater
            } else {
                Equal
            }
        });

        if u != uu {
            out += uu[uu.len() / 2];
            // println!("{u:?}\n{uu:?}\n");
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 143);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 123);
    }
}
