use std::{char, fs::read_to_string};

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
    // searching for the string XMAS in a 2D grid
    // can appear in any 8-orientation, backwards, overlapping

    // doesn't appear to wrap around

    // OK so overlapping makes it easier
    // and backwards isn't a problem, we can match SAMX just as easily

    // treat infile as a 2d grid of chars

    let grid: Vec<Vec<char>> = infile.lines().map(|x| x.chars().collect()).collect();

    let mut count = 0;

    let cmax = grid.iter().map(|x| x.len()).max().unwrap_or_default();
    let rmax = grid.len();

    let fwd = ['X', 'M', 'A', 'S'];
    let bak = ['S', 'A', 'M', 'X'];

    for r in 0..rmax {
        for c in 0..cmax {
            if c < cmax - 3 {
                let horz: [char; 4] = [grid[r][c], grid[r][c + 1], grid[r][c + 2], grid[r][c + 3]];

                if horz == fwd || horz == bak {
                    count += 1;
                    // println!("({r}, {c}), {}, H", horz == fwd);
                }
            }

            if r < rmax - 3 {
                let vert = [grid[r][c], grid[r + 1][c], grid[r + 2][c], grid[r + 3][c]];

                if vert == fwd || vert == bak {
                    count += 1;
                    // println!("({r}, {c}), {}, V", vert == fwd);
                }
            }

            // diagonal down-right
            if (c < cmax - 3) && (r < rmax - 3) {
                let diag = [
                    grid[r][c],
                    grid[r + 1][c + 1],
                    grid[r + 2][c + 2],
                    grid[r + 3][c + 3],
                ];

                if diag == fwd || diag == bak {
                    count += 1;
                    // println!("({r}, {c}), {}, R", diag == fwd);
                }
            }

            // diagonal down-left
            if (c > 2) && (r < rmax - 3) {
                let diag = [
                    grid[r][c],
                    grid[r + 1][c - 1],
                    grid[r + 2][c - 2],
                    grid[r + 3][c - 3],
                ];

                if diag == fwd || diag == bak {
                    count += 1;
                    // println!("({r}, {c}), {}, L", diag == fwd);
                }
            }
        }
    }

    count
}
fn part_2(infile: &str) -> usize {
    // ok now we're looking for MAS in the shape of an X
    // M.S
    // .A.
    // M.S

    let grid: Vec<Vec<char>> = infile.lines().map(|x| x.chars().collect()).collect();

    let mut count = 0;

    let cmax = grid.iter().map(|x| x.len()).max().unwrap_or_default();
    let rmax = grid.len();

    let west = ['M', 'S', 'A', 'M', 'S'];
    let east = ['S', 'M', 'A', 'S', 'M'];
    let north = ['M', 'M', 'A', 'S', 'S'];
    let south = ['S', 'S', 'A', 'M', 'M'];

    for r in 0..(rmax - 2) {
        for c in 0..(cmax - 2) {
            let needle = [
                grid[r][c],
                grid[r][c + 2],
                grid[r + 1][c + 1],
                grid[r + 2][c],
                grid[r + 2][c + 2],
            ];

            if needle == north || needle == south || needle == east || needle == west {
                count += 1;
                // println!("({r}, {c})");
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 18);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 9);
    }
}
