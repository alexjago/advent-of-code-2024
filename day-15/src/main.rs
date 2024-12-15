use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use mapgrid::*;
use strum::{self, Display, EnumString};

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

#[derive(Debug, Display, EnumString, PartialEq, Eq, Hash, Clone, Copy)]
enum Entity {
    #[strum(serialize = "#")]
    Wall,
    #[strum(serialize = "@")]
    Robot,
    #[strum(serialize = "O")]
    Box,
    /// conventionally, positioned by its left edge
    /// but taking up TWO positions on x axis
    #[strum(serialize = "[]")]
    WideBox,
}

fn move_to_dir(m: char) -> Option<Coord> {
    match m {
        '^' => Some([0, -1]),
        'v' => Some([0, 1]),
        '<' => Some([-1, 0]),
        '>' => Some([1, 0]),
        _ => None,
    }
}

fn part_1(infile: &str) -> isize {
    let (grid_raw, moves) = infile.split("\n\n").collect_tuple().unwrap();

    let grid = <HashMap<Coord, Entity> as Grid<Entity>>::from_str_with(grid_raw, |c| {
        Entity::try_from(c.to_string().as_str()).ok()
    });

    /*
    println!(
        "{}",
        <HashMap<Coord, Entity> as Grid<Entity>>::visualise(&grid)
    );
    */

    let walls: HashSet<Coord> = grid
        .iter()
        .filter(|(_, v)| **v == Entity::Wall)
        .map(|(k, _)| k)
        .cloned()
        .collect();
    let mut boxes: HashSet<Coord> = grid
        .iter()
        .filter(|(_, v)| **v == Entity::Box)
        .map(|(k, _)| k)
        .cloned()
        .collect();
    let mut robot: Coord = *grid
        .iter()
        .filter(|(_, v)| **v == Entity::Robot)
        .map(|(k, _)| k)
        .next()
        .unwrap();

    drop(grid);

    for dir in moves.chars().filter_map(move_to_dir) {
        let mut test = robot;
        loop {
            test = add(test, dir);

            if walls.contains(&test) {
                // no movement in this direction
                break;
            } else if boxes.contains(&test) {
                continue;
            } else {
                // empty space found
                let newrobot = add(robot, dir);
                boxes.insert(test); // we "pushed" a line of boxes
                boxes.remove(&newrobot);
                robot = newrobot;
                break;
            }
        }
        /*
        // diagnostic printing
        let mut grid = HashMap::new();
        for k in &walls {
            grid.insert(k.clone(), Entity::Wall);
        }
        for k in &boxes {
            grid.insert(k.clone(), Entity::Box);
        }
        grid.insert(robot.clone(), Entity::Robot);
        println!(
            "{}",
            <HashMap<Coord, Entity> as Grid<Entity>>::visualise(&grid)
        );
        */
    }

    boxes.iter().map(|[x, y]| x + y * 100).sum::<isize>()
}
fn part_2(infile: &str) -> isize {
    let (grid_raw, moves) = infile.split("\n\n").collect_tuple().unwrap();

    let grid_narrow = <HashMap<Coord, Entity> as Grid<Entity>>::from_str_with(grid_raw, |c| {
        Entity::try_from(c.to_string().as_str()).ok()
    });

    let mut grid = HashMap::new();

    for (k, v) in grid_narrow.into_iter() {
        if v == Entity::Robot {
            grid.insert([k[0] * 2, k[1]], v);
        } else if v == Entity::Box {
            grid.insert([k[0] * 2, k[1]], Entity::WideBox);
        } else if v == Entity::Wall {
            grid.insert([k[0] * 2, k[1]], v);
            grid.insert([k[0] * 2 + 1, k[1]], v);
        }
    }

    /*
        println!(
            "{}",
            <HashMap<Coord, Entity> as Grid<Entity>>::visualise(&grid)
        );
        println!("{:?}", grid.values().counts());

    */
    let walls: HashSet<Coord> = grid
        .iter()
        .filter(|(_, v)| **v == Entity::Wall)
        .map(|(k, _)| k)
        .cloned()
        .collect();
    let mut wide_boxes: HashSet<Coord> = grid
        .iter()
        .filter(|(_, v)| **v == Entity::WideBox)
        .map(|(k, _)| k)
        .cloned()
        .collect();
    let mut robot: Coord = *grid
        .iter()
        .filter(|(_, v)| **v == Entity::Robot)
        .map(|(k, _)| k)
        .next()
        .unwrap();

    drop(grid);

    // previously each box could only push one other box
    // but now we can push two
    // this means the following is possible:

    /*
        [][][][][]
         [][][][]
          [][][]
           [][]
            []
            @
    */

    for (i, dir) in moves.chars().filter_map(move_to_dir).enumerate() {
        let mut moving = vec![];
        let mut queue = vec![robot];
        let mut wall_found = false;
        while let Some(next) = queue.pop() {
            // if we're the robot, we only need to test whether a wall is directly blocking us
            // or if we're pushing on LHS or RHS of a widebox

            // if we're a widebox (not the robot)
            // we need to test whether a wall is blocking our LHS or RHS
            // and our LHS could be pushing on an RHS or an LHS, and our RHS could be pushing on an LHS tooi

            let is_robot = next == robot;

            let test = add(next, dir);

            if walls.contains(&test) || (!is_robot && walls.contains(&add(test, [1, 0]))) {
                // no movement in this direction
                moving.clear();
                queue.clear();
                wall_found = true;
                // println!("{i}: {next:?} hit wall");
                break;
            }

            for d in [[-1, 0], [0, 0], [1, 0]] {
                let side = add(test, d);
                if wide_boxes.contains(&side) && !moving.contains(&side) && !queue.contains(&side) {
                    if d == [1, 0] && is_robot {
                        continue;
                    }
                    queue.push(side);
                    moving.push(side);
                    // else: empty space, continue
                    /*
                    println!(
                        "{i}: {next:?} found widebox at {side:?}, queue now {} deep",
                        queue.len()
                    );
                    */
                }
            }
        }
        if !wall_found {
            // once we have dealt with everything

            for m in &moving {
                wide_boxes.remove(m);
            }
            for m in moving {
                wide_boxes.insert(add(m, dir));
            }
            robot = add(robot, dir);
        }
        /*
        // diagnostic printing
        let mut grid = HashMap::new();
        for k in &walls {
            grid.insert(*k, Entity::Wall);
        }
        for k in &wide_boxes {
            grid.insert(*k, Entity::WideBox);
        }
        grid.insert(robot, Entity::Robot);
        println!(
            "{i} {dir:?}:\n{}",
            <HashMap<Coord, Entity> as Grid<Entity>>::visualise(&grid)
        );
        */
    }

    wide_boxes.iter().map(|[x, y]| x + y * 100).sum::<isize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const LARGER_EG: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALLER_2: &str = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(EXAMPLE_1), 2028);
    }
    #[test]
    fn part_1_large() {
        assert_eq!(part_1(LARGER_EG), 10092);
    }

    #[test]
    fn part_2_small() {
        assert_eq!(part_2(SMALLER_2), 618);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(LARGER_EG), 9021);
    }
}
