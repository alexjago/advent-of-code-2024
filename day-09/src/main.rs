use std::{fs::read_to_string};

use anyhow::Result;
use clap::Parser;

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
    // input of ((\d)(\d))*(\d?)
    // pairs of {file length, empty space length}

    // we need to compact files moving blocks one at a time from the end to the first available free

    // files have an index based on the initial representation (starting at zero)

    // let re: Regex = Regex::new(r"((\d)(\d))*(\d?)").unwrap();

    let mut indexes: Vec<Option<usize>> = vec![];

    for (i, c) in infile.chars().filter(|x| x.is_digit(10)).enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // file number is i/2
            for _ in 0..num {
                indexes.push(Some(i / 2));
            }
        } else {
            for _ in 0..num {
                indexes.push(None);
            }
        }
    }

    // println!(
    //     "{}",
    //     indexes
    //         .iter()
    //         .map(|x| match x {
    //             Some(y) => y.to_string(),
    //             None => String::from("."),
    //         })
    //         .collect::<String>()
    // );

    while let Some(k) = indexes.iter().position(|x| x.is_none()) {
        indexes.swap_remove(k);
    }

    // println!(
    //     "{}",
    //     indexes
    //         .iter()
    //         .map(|x| match x {
    //             Some(y) => y.to_string(),
    //             None => String::from("."),
    //         })
    //         .collect::<String>()
    // );

    indexes
        .iter()
        .enumerate()
        .map(|(i, x)| if let Some(xx) = *x { i * xx } else { 0 })
        .sum()
}
fn part_2(infile: &str) -> usize {
    // ah yes, now it's all or nothing

    // file id (None for empty), length
    let mut data: Vec<(Option<usize>, usize)> = vec![];

    for (i, c) in infile.chars().filter(|x| x.is_digit(10)).enumerate() {
        let num = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // file number is i/2
            data.push((Some(i / 2), num));
        } else {
            data.push((None, num));
        }
    }

    // println!("{data:?}");

    let highest_id = (data.len() - 1) / 2;

    for i in (0..highest_id).map(|x| highest_id - x) {
        // println!("Attempting to move index {i}...");

        let me = data
            .iter()
            .position(|(x, _)| x.is_some_and(|y| y == i))
            .unwrap();

        // println!("\tfrom position {me}...");

        for k in 0..me {
            let slot = data[k];
            if slot.0.is_none() {
                // there's empty space we could move to

                let move_me = data[me];
                data[me].0 = None;

                if slot.1 == move_me.1 {
                    data[k] = move_me;
                    // println!("\t to position {k} (exactly)");
                    break;
                } else if slot.1 > move_me.1 {
                    data.insert(k, move_me);
                    // update length, slot var invalidated?
                    data[k + 1].1 -= data[k].1;
                    // println!("\t to position {k} (with {} leftover)", data[k + 1].1);
                    break;
                } else {
                    // put it back
                    data[me] = move_me;
                }
            }
        }
    }

    // println!("{data:?}");

    let mut out = 0;
    let mut pos = 0;

    for (i, n) in data {
        for p in pos..(pos + n) {
            out += p * match i {
                Some(y) => y,
                None => 0,
            }
        }
        pos += n;
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"2333133121414131402";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 1928);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 2858);
    }
}
