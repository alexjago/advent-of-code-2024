use std::{collections::HashSet, fs::read_to_string};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
use rayon::{
    self,
    iter::{IntoParallelIterator, ParallelIterator},
};
use regex::Regex;

#[derive(Parser)]
pub struct Opts {
    infile: std::path::PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = clap::Parser::parse();

    let infile = read_to_string(opts.infile)?;

    println!("Part 1:\n{}", part_1(&infile, 101, 103));
    println!("Part 2:\n{}", part_2(&infile, 101, 103));

    Ok(())
}

fn part_1(infile: &str, width: isize, height: isize) -> usize {
    // world wrap

    let re = Regex::new(r#"-?\d+"#).unwrap();

    let robots: Vec<(Coord, Coord)> = re
        .find_iter(infile)
        .filter_map(|x| x.as_str().parse::<isize>().ok())
        .tuples()
        .map(|(px, py, vx, vy)| ([px, py], [vx, vy]))
        .collect();

    // for r in &robots {
    //     println!("{r:?}");
    // }

    let after_100s: Vec<(Coord, Coord)> = robots
        .into_iter()
        .map(|([px, py], [vx, vy])| {
            (
                [
                    (px + 100 * vx + (100 * width)) % width,
                    (py + 100 * vy + (100 * height)) % height,
                ],
                [vx, vy],
            )
        })
        .collect();

    let positions_count = after_100s.iter().map(|(coord, _)| coord).counts();

    let mut quads = [0, 0, 0, 0];

    for ([px, py], count) in positions_count {
        let w = width / 2;
        let h = height / 2;
        let qx: usize = if *px > w {
            1
        } else if *px < w {
            0
        } else {
            continue;
        };
        let qy: usize = if *py > h {
            2
        } else if *py < h {
            0
        } else {
            continue;
        };

        // println!("{} robot at [{}, {}] in quad {}", count, px, py, qx + qy);

        quads[qx + qy] += count;
    }
    quads.iter().fold(1, |acc, x| acc * x)
}

fn part_2(infile: &str, width: isize, height: isize) -> isize {
    // world wrap

    let re = Regex::new(r#"-?\d+"#).unwrap();

    let robots_orig: Vec<(Coord, Coord)> = re
        .find_iter(infile)
        .filter_map(|x| x.as_str().parse::<isize>().ok())
        .tuples()
        .map(|(px, py, vx, vy)| ([px, py], [vx, vy]))
        .collect();

    // for r in &robots {
    //     println!("{r:?}");
    // }

    let seconds_tot = width * height;

    if let Some(seconds) = (0..seconds_tot)
        .into_par_iter()
        .filter_map(|s| p2_helper(&robots_orig, s, width, height))
        .min()
    {
        return seconds;
    } else {
        println!(
        "Could not find tree after {seconds_tot} seconds; here's the grid on the last iteration"
    );

        let grid: HashSet<Coord> = robots_orig
            .iter()
            .map(|([px, py], [vx, vy])| {
                [
                    (px + seconds_tot * vx + (seconds_tot * width)) % width,
                    (py + seconds_tot * vy + (seconds_tot * height)) % height,
                ]
            })
            .collect();

        println!("{}", <HashSet<Coord> as Grid<char>>::visualise(&grid));
        return -1;
    }
}

fn p2_helper(
    robots: &[(Coord, Coord)],
    seconds: isize,
    width: isize,
    height: isize,
) -> Option<isize> {
    let grid: HashSet<Coord> = robots
        .iter()
        .map(|([px, py], [vx, vy])| {
            [
                (px + seconds * vx + (seconds * width)) % width,
                (py + seconds * vy + (seconds * height)) % height,
            ]
        })
        .collect();

    // we're looking for a picture of a christmas tree, which if rumour is to be believed means a block similar to the below
    /*
    ....#....
    ...###...
    ..#####..
    .#######.
    */

    // we have 500 robots and a 101x103 grid
    // the triangle numbers approaching 500 are:
    // 253 (22nd), 276 (23rd), 300 (24th), 325 (25th), 351 (26th), 378 (27th),
    // 406 (28th), 435 (29th), 465 (30th) and 496 (31st)

    // so we should search for a row containing at least 23 contiguous occupied spaces ("most of the robots" and odd)
    // if we find this then we check to see if the row above is of pattern .#####################. (21 contig with spaces at edge)
    // if it is we've probably found it

    // Looking at some spoilers it's not quite that easy
    // but the approach of "find a run within a row, then check if there's a run in the row above" seems OK

    for y in 0..height {
        let mut run = 0_isize;
        for x in 0..width {
            if grid.contains(&[x, y]) {
                run += 1;
                if run >= 7_isize {
                    // check for run in row above
                    let yy = y - 1;
                    let mut above = 0;
                    for xx in (x - run)..=x {
                        if grid.contains(&[xx, yy]) {
                            above += 1;
                        }
                    }
                    if above == run - 2 {
                        println!("Possible Christmas Tree after {seconds} seconds");
                        println!("{}", <HashSet<Coord> as Grid<char>>::visualise(&grid));
                        return Some(seconds);
                    }
                }
            } else {
                run = 0;
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    const PART_2_DEBUG: &str = r"p=3,0 v=0,0
p=2,1 v=0,0
p=3,1 v=0,0
p=4,1 v=0,0
p=1,2 v=0,0
p=2,2 v=0,0
p=3,2 v=0,0
p=4,2 v=0,0
p=5,2 v=0,0
p=0,3 v=0,0
p=1,3 v=0,0
p=2,3 v=0,0
p=3,3 v=0,0
p=4,3 v=0,0
p=5,3 v=0,0
p=6,3 v=0,0";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1, 11, 7), 12);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(PART_2_DEBUG, 7, 4), 0);
    }
}
