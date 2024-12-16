use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
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

/// reimplementing complex multiplication:
/// A * B = r(A)*r(B) @ t(A) + t(B)
/// this is only gonna be good enough for
/// * forward: r = 1, t = 0 ===> 1 + 0i ===> [1,  0]
/// * left: r = 1, t = pi/2 ===> 0 + 1i ===> [0,  1]
/// * right: r = 1, t = -pi/2 => 0 - 1i ===> [0, -1]
fn turn(facing: Coord, by: Coord) -> Coord {
    // first, -last (i**2), inners, outers
    [
        facing[0] * by[0] - facing[1] * by[1],
        facing[0] * by[1] + facing[1] * by[0],
    ]
}

/// Return the position you'd be in
/// if you were at `pos`, facing `facing`, then turned `dir`
/// and then moved one step further
fn advance(pos: Coord, facing: Coord, dir: Coord) -> Coord {
    add(pos, turn(facing, dir))
}

/// +Y = south means reversal
const LEFT: Coord = [0, -1];
/// +Y = south means reversal
const RIGHT: Coord = [0, 1];
const FWD: Coord = [1, 0];
const EAST: Coord = [1, 0];
const WEST: Coord = [-1, 0];
/// +Y = south means reversal
const NORTH: Coord = [0, -1];
/// +Y = south means reversal
const SOUTH: Coord = [0, 1];

fn part_1(infile: &str) -> usize {
    let grid = <HashMap<Coord, char> as Grid<char>>::from_str_with(infile, |c| Some(c));

    println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&grid));

    let turns = [(LEFT, 1001), (RIGHT, 1001), (FWD, 1)];

    println!(
        "east (+x) is {:?}, turning left facing north (+y?) {:?}",
        [1, 0],
        turn([1, 0], [0, 1])
    );

    // there is the smallll problem that in MapGrid convention, down (South) is +Y
    // but for part 1 turning left or right are worth the same

    let start_pos = grid
        .iter()
        .find_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .unwrap();
    let end_pos = grid
        .iter()
        .find_map(|(k, v)| if *v == 'E' { Some(*k) } else { None })
        .unwrap();

    println!("start: {start_pos:?}\tend: {end_pos:?}");

    // OK so we can basically just do a BFS thing here right?

    let mut vis = grid.clone();

    let mut scores: HashMap<Coord, usize> = HashMap::new();
    scores.insert(start_pos.clone(), 0);

    let mut queue = vec![];

    queue.push((start_pos, EAST));

    while let Some((pos, facing)) = queue.pop() {
        let sco = *scores.get(&pos).unwrap_or(&usize::MAX);

        for (dir, pts) in turns {
            let newpos = advance(pos, facing, dir);

            let newsco = sco + pts;

            if (newsco < *scores.get(&newpos).unwrap_or(&usize::MAX))
                && !(grid.get(&newpos).unwrap_or(&'#') == &'#')
            {
                scores.insert(newpos, newsco);
                queue.push((newpos, turn(facing, dir)));

                vis.insert(
                    pos,
                    match turn(facing, dir) {
                        NORTH => '^',
                        SOUTH => 'v',
                        EAST => '>',
                        WEST => '<',
                        _ => unimplemented!(),
                    },
                );
            }
        }
    }

    println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&vis));

    *scores.get(&end_pos).unwrap_or(&usize::MAX)
}
// now we have to keep track of all of the best paths through the maze

// do we just have an auxiliary histories map (like scores)?

// turns out we need to be a bit more careful about storing where we came from

fn part_2(infile: &str) -> usize {
    let expected_score = part_1(infile);

    let grid = <HashMap<Coord, char> as Grid<char>>::from_str_with(infile, |c| Some(c));

    // println!("{}", <HashMap<Coord, char> as Grid<char>>::visualise(&grid));

    let turns = [(LEFT, 1000), (RIGHT, 1000), (FWD, 1)];

    let start_pos = grid
        .iter()
        .find_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .unwrap();
    let end_pos = grid
        .iter()
        .find_map(|(k, v)| if *v == 'E' { Some(*k) } else { None })
        .unwrap();

    println!("start: {start_pos:?}\tend: {end_pos:?}");

    // OK so we can basically just do a BFS thing here right?

    // let mut vis = grid.clone();

    // alongside score, keep track of all visited coords
    // (position, facing)
    let mut histories: HashMap<(Coord, Coord), HashSet<(Coord, Coord)>> = HashMap::new();
    histories.insert((start_pos.clone(), EAST), HashSet::new());

    let mut scores: HashMap<(Coord, Coord), usize> = HashMap::new();
    scores.insert((start_pos.clone(), EAST), 0);

    let mut queue: BTreeSet<(Coord, Coord)> = BTreeSet::new();
    queue.insert((start_pos, EAST));

    while let Some((pos, facing)) = queue.pop_first() {
        let sco = *scores.get(&(pos, facing)).unwrap_or(&usize::MAX);

        if sco > expected_score {
            // no good will come of this
            continue;
        }

        for (dir, pts) in turns {
            let newpos = if dir == FWD {
                advance(pos, facing, dir)
            } else {
                pos
            };

            let newsco = sco + pts;

            let newdir = turn(facing, dir);

            let oldsco = *scores.get(&(newpos, newdir)).unwrap_or(&usize::MAX);

            if (newsco <= oldsco)
                && (grid.contains_key(&newpos))
                && (grid.get(&newpos).unwrap_or(&'#') != &'#')
            {
                scores.insert((newpos, newdir), newsco);

                // PERF: the queue was previously a Vec
                // EXPERIMENT: trying a BTreeSet instead of a Vec
                // RESULT: way, way, way faster
                if !queue.contains(&(newpos, newdir)) {
                    queue.insert((newpos, newdir));
                }

                let mut hist = histories
                    .get(&(newpos, newdir))
                    .unwrap_or(&HashSet::<(Coord, Coord)>::new())
                    .clone();

                if newsco == oldsco {
                    // both equally valid histories
                    hist.extend(histories.get(&(pos, facing)).unwrap().iter());
                } else {
                    // replacement
                    hist = histories.get(&(pos, facing)).unwrap().clone();
                }
                // add self to new node's history
                hist.insert((pos, facing));

                // replace new node's history
                histories.insert((newpos, newdir), hist);
            }
        }

        if scores.len() % 1000 == 0 {
            println!(
                "progress: {} (out of max {}) with queue depth {}",
                scores.len(),
                grid.values().filter(|c| **c != '#').count() * 4,
                queue.len()
            )
        }
    }

    println!(
        "progress: {} (out of max {}) with queue depth {}",
        scores.len(),
        grid.values().filter(|c| **c != '#').count() * 4,
        queue.len()
    );

    /*
    println!(
        "{}",
        <HashSet<Coord> as Grid<char>>::visualise(histories.get(&(end_pos, end_dir)).unwrap())
    );
    */

    let mut end_dir = [0, 0];
    let mut lowscore = usize::MAX;
    for ((p, d), h) in histories.iter().filter(|((p, _), _)| *p == end_pos) {
        let sco = scores.get(&(*p, *d)).unwrap();
        if *sco < lowscore {
            end_dir = *d;
            lowscore = *sco;
        }
        println!("\n{p:?} {d:?} got {sco}\n{h:?}");
    }

    println!("Winner: {end_dir:?}");

    let hist2 = histories
        .get(&(end_pos, end_dir))
        .unwrap()
        .iter()
        .map(|(p, _)| p.clone())
        .counts();

    println!(
        "{}",
        <HashMap<Coord, usize> as Grid<usize>>::visualise(&hist2)
    );

    // need the +1 because we don't store ourselves in our history
    hist2.len() + 1
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_2: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn directionality() {
        assert_eq!(turn(NORTH, LEFT), WEST);
        assert_eq!(turn(NORTH, RIGHT), EAST);
        assert_eq!(turn(NORTH, FWD), NORTH);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 7036);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(EXAMPLE_1), 45);
    }
    #[test]
    fn part_2_example_2() {
        assert_eq!(part_2(EXAMPLE_2), 64);
    }
}
