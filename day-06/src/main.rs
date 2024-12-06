use std::{collections::HashSet, fs::read_to_string};

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

fn part_1_internal(infile: &str) -> Vec<Vec<char>> {
    // we get a grid (. for empty space or # for obstacle)
    // with ^V<> representing a guard position and direction
    // guard moves forward until hitting an obstacle, then turns right
    // eventually guard will walk out of the grid
    // replace positions with Xs

    let mut grid: Vec<Vec<char>> = infile.lines().map(|x| x.chars().collect()).collect();

    let mut r = 0;
    let mut c = 0;

    'find_guard: for (tr, row) in grid.iter().enumerate() {
        for (tc, x) in row.iter().enumerate() {
            if *x == '^' || *x == 'v' || *x == '<' || *x == '>' {
                r = tr;
                c = tc;
                break 'find_guard;
            }
        }
    }

    // println!("Guard starts at ({r}, {c})");

    'fill: loop {
        let (dr, dc) = match grid[r][c] {
            '<' => Some((0, -1)),
            '>' => Some((0, 1)),
            '^' => Some((-1, 0)),
            'v' => Some((1, 0)),
            _ => None,
        }
        .unwrap();

        let fr = r as isize + dr;
        let fc = c as isize + dc;

        if fr < 0 || fc < 0 {
            grid[r][c] = 'X';
            break 'fill;
        }

        let fr = fr as usize;
        let fc = fc as usize;

        if fr >= grid.len() || fc >= grid[fr].len() {
            grid[r][c] = 'X';
            break 'fill;
        }

        // test faced direction
        let turn = match grid[fr][fc] {
            '#' => true,
            _ => false,
        };

        if turn {
            grid[r][c] = match grid[r][c] {
                '<' => '^',
                '>' => 'v',
                '^' => '>',
                'v' => '<',
                _ => unimplemented!(),
            }
        } else {
            grid[fr][fc] = grid[r][c];
            grid[r][c] = 'X';
            r = fr;
            c = fc;
        }
    }

    grid
}

fn part_1(infile: &str) -> usize {
    part_1_internal(infile)
        .iter()
        .flatten()
        .filter(|x| **x == 'X')
        .count()
}

fn part_2(infile: &str) -> usize {
    // hoo boy, now we need to place an obstacle so as to cause a loop
    // we have (checks notes) about five thousand options so this is technically brute-forceable (17k total map size)

    let part_1_grid = part_1_internal(infile);
    // println!(
    //     "{}\n",
    //     part_1_grid
    //         .iter()
    //         .map(|s| s.iter().collect::<String>())
    //         .join("\n")
    // );

    let grid_orig: Vec<Vec<char>> = infile.lines().map(|x| x.chars().collect()).collect();

    let mut guard_r = 0;
    let mut guard_c = 0;

    'find_guard: for (tr, row) in grid_orig.iter().enumerate() {
        for (tc, x) in row.iter().enumerate() {
            if *x == '^' || *x == 'v' || *x == '<' || *x == '>' {
                guard_r = tr;
                guard_c = tc;
                break 'find_guard;
            }
        }
    }

    // println!("Guard starts at ({guard_r}, {guard_c})");

    let mut placeables = vec![];
    let options: Vec<(usize, usize)> = part_1_grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, x)| {
                if *x == 'X' {
                    Some((r.clone(), c, *x))
                } else {
                    None
                }
            })
        })
        .map(|(r, c, _)| (r, c))
        .collect();

    // println!("{} options", options.len());

    'testing: for (obs_r, obs_c) in options {
        if grid_orig[obs_r][obs_c] != '.' {
            continue 'testing;
        }

        let mut r = guard_r;
        let mut c = guard_c;

        let mut grid = grid_orig.clone();

        grid[obs_r][obs_c] = '#';

        let mut turns: HashSet<(usize, usize, char)> = HashSet::new();

        'fill: loop {
            let (dr, dc) = match grid[r][c] {
                '<' => Some((0, -1)),
                '>' => Some((0, 1)),
                '^' => Some((-1, 0)),
                'v' => Some((1, 0)),
                _ => None,
            }
            .unwrap();

            let fr = r as isize + dr;
            let fc = c as isize + dc;

            if fr < 0 || fc < 0 {
                break 'fill;
            }

            let fr = fr as usize;
            let fc = fc as usize;

            if fr >= grid.len() || fc >= grid[fr].len() {
                break 'fill;
            }

            // test faced direction
            let turn = match grid[fr][fc] {
                '#' => true,
                _ => false,
            };

            if turn {
                if turns.contains(&(r, c, grid[r][c])) {
                    placeables.push((obs_r, obs_c));
                    break 'fill;
                } else {
                    turns.insert((r, c, grid[r][c]));
                }

                grid[r][c] = match grid[r][c] {
                    '<' => '^',
                    '>' => 'v',
                    '^' => '>',
                    'v' => '<',
                    _ => unimplemented!(),
                }
            } else {
                grid[fr][fc] = grid[r][c];
                grid[r][c] = 'X';
                r = fr;
                c = fc;
            }
            // continue 'fill
        }
    }

    // println!("{placeables:?}");

    placeables.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 41);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 6);
    }
}
