use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
use num::{integer::ExtendedGcd, Integer};
use regex::Regex;

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

/// (+A, +B, Prize)
fn parse_input(infile: &str) -> Vec<(Coord, Coord, Coord)> {
    let re = Regex::new(r#".*?(\d+).*?(\d+)"#).unwrap();
    infile
        .lines()
        .filter(|x| x.len() > 0)
        .filter_map(|x| re.captures(x))
        .map(|c| {
            [
                c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            ]
        })
        .tuples()
        .collect()
}

fn part_1(infile: &str) -> usize {
    // 3 tokens for button A, 1 token for button B
    // want to spend: smallest number of tokens to get to goal
    // limit of 100 button presses per machine

    let machines = parse_input(infile);

    // we need to find some h i j k such that
    // h * X_a + i * X_b == X_p
    // j * Y_a + k * Y_b == Y_p

    // (or show impossible)

    // is this some sort of reverse Bezout?
    // in Bezout the result is the GCD
    // >> Moreover, the integers of the form az + bt are exactly the multiples of d

    // or is this the obligatory CRT question?
    // inputs are not always pairwise coprime but if we divide the result through by their GCD

    // example: X_a = 94, X_b = 22, result X = 8400
    // divide through by 2 (gcd)
    // 47, 11, 4200

    // c'mon we can just bruteforce part 1?
    // luckily on my input the first result found (I guess because of structuring to minimise press_A) is correct
    let mut tokens = 0;
    'm: for (a, b, p) in machines {
        for press_a in 0..100 {
            for press_b in 0..100 {
                if press_a * a[0] + press_b * b[0] == p[0]
                    && press_a * a[1] + press_b * b[1] == p[1]
                {
                    tokens += (3 * press_a + press_b) as usize;
                    println!("{press_a} * {a:?} + {press_b} * {b:?} => {p:?}");
                    continue 'm;
                }
            }
        }
    }

    tokens
}
fn part_2(infile: &str) -> isize {
    // +10000000000000 to the X and Y coordinates of each prize
    // remove button-press limit

    let machines = parse_input(infile);

    // we can no longer bruteforce part 2 :(
    // let's figure out the moduli in each
    // oh yeah it is Bezout

    // >> ax + by = c
    // >> This Diophantine equation has a solution (where x and y are integers) if and only if c is a multiple of the greatest common divisor of a and b.

    // having gotten x, y as a Bezout pair for each axis (x and y aren't axes here)
    // we can generate (x', y') as follows
    // x' = x - k * b / d
    // y' = y + k * a / d
    // for some arbitrary integer k? and where d is the gcd of the A, B values for that axis

    // then x' * X_a + y' * X_b = X_p

    // or.....................

    // we could do linear algebra
    // h/t to everyone who suggested Cramers rule (especially villuna)

    // ax + by == p    (where a, b stand in for the X components of button A, B and p stands in for the X target)
    // cx + dy == q    (where c, d " ...  q Y)
    // solve for x, y (where x is the number of button presses for A, y the number for B)

    /*

    [a b] [x] = [p]
    [c d] [y]   [q]

    then

    x = (pd - bq) / (ad - bc)
    y = (aq - pc) / (ad - bc)
    */

    let mut tokens = 0;
    for ([a, c], [b, d], [p, q]) in machines {
        let [p, q] = add([p, q], [10000000000000, 10000000000000]);
        let x = (p * d - b * q) / (a * d - b * c);
        let y = (a * q - p * c) / (a * d - b * c);

        if (a * x + b * y == p) && (c * x + d * y == q) {
            tokens += 3 * x + y;
            println!("{a}*{x} + {b}*{y} == {p}");
            println!("{c}*{x} + {d}*{y} == {q}\n");
        }
    }

    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 480);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 875318608908);
    }
}
