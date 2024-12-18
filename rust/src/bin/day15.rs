use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

#[derive(Debug)]
struct Input {
    map: Map,
    movements: Vec<Dir>,
}

#[derive(Debug, Clone)]
struct Map {
    robot: Pos,
    boxes: HashSet<Pos>,
    walls: HashSet<Pos>,
}

impl Map {
    fn move_robot(&mut self, dir: Dir) {
        let (dx, dy) = match dir {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        };

        let next_pos = self.robot.translate(dx, dy);

        if self.walls.contains(&next_pos) {
            return;
        }
        if !self.boxes.contains(&next_pos) {
            self.robot = next_pos;
            return;
        }

        // If here it means it's moving into a box

        let mut test_pos = next_pos;
        let empty_pos = loop {
            test_pos = test_pos.translate(dx, dy);
            if self.walls.contains(&test_pos) {
                break None;
            }
            if !self.boxes.contains(&test_pos) {
                break Some(test_pos);
            }
        };

        if let Some(empty_pos) = empty_pos {
            self.boxes.insert(empty_pos);
            assert!(self.boxes.remove(&next_pos));
            self.robot = next_pos;
        }
    }

    fn print(&self) {
        let max_x = self
            .walls
            .iter()
            .map(|Pos { x, .. }| *x)
            .max()
            .unwrap_or_default();
        let max_y = self
            .walls
            .iter()
            .map(|Pos { y, .. }| *y)
            .max()
            .unwrap_or_default();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let p = Pos::new(x, y);
                let c = if self.robot == p {
                    '@'
                } else if self.boxes.contains(&p) {
                    'O'
                } else if self.walls.contains(&p) {
                    '#'
                } else {
                    '.'
                };
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct WiderMap {
    robot: Pos,
    boxes: HashSet<Pos>,
    walls: HashSet<Pos>,
}

impl WiderMap {
    fn move_robot(&mut self, dir: Dir) {
        let (dx, dy) = match dir {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        };

        let next_pos = self.robot.translate(dx, dy);

        if self.walls.contains(&next_pos) {
            return;
        }

        println!("1) self.robot={:?}, next_pos={:?}", self.robot, next_pos);

        let maybe_box_pos = self
            .boxes
            .get(&next_pos)
            .or_else(|| self.boxes.get(&next_pos.translate(-1, 0)));

        let Some(box_pos) = maybe_box_pos.cloned() else {
            self.robot = next_pos;
            return;
        };

        // If here it means it's moving into a box

        let mut boxes_to_move = HashSet::new();
        boxes_to_move.insert(box_pos);

        let mut test_pos = box_pos;
        println!("1) test_pos={test_pos:?}");
        let maybe_boxes_to_move = if dir == Dir::Left || dir == Dir::Right {
            loop {
                test_pos = test_pos.translate(dx, dy);
                println!("test_pos={test_pos:?}");
                if self.walls.contains(&test_pos) {
                    break None;
                }

                let maybe_box_pos = self
                    .boxes
                    .get(&test_pos)
                    .or_else(|| self.boxes.get(&test_pos.translate(-1, 0)));

                println!("maybe_box_pos={maybe_box_pos:?}");

                let Some(box_pos) = maybe_box_pos.cloned() else {
                    // boxes_to_move.insert(box_pos);
                    break Some(boxes_to_move);
                };

                boxes_to_move.insert(box_pos);
            }
        } else {
            let mut test_positions = [test_pos].into_iter().collect::<HashSet<_>>();
            // let mut cnt = 0;

            'outer: loop {
                // cnt += 1;
                // if cnt > 5 {
                //     break 'outer None;
                // }
                let n_test_positions = test_positions.len();
                let mut n_to_move = 0;

                println!("== test_positions={test_positions:?}");
                for test_pos in std::mem::take(&mut test_positions) {
                    println!("--");
                    let mut maybe_box_pos2 = [None, None];
                    for i in 0..=1 {
                        let test_pos = test_pos.translate(dx + i as i64, dy);
                        println!("2) test_pos={test_pos:?}");
                        if self.walls.contains(&test_pos) {
                            break 'outer None;
                        }

                        let maybe_box_pos = self
                            .boxes
                            .get(&test_pos)
                            .or_else(|| self.boxes.get(&test_pos.translate(-1, 0)));

                        maybe_box_pos2[i] = maybe_box_pos;
                    }

                    // let maybe_box_pos = maybe_box_pos2[0].or(maybe_box_pos2[1]);

                    // println!("maybe_box_pos={maybe_box_pos:?} maybe_box_pos2={maybe_box_pos2:?}");
                    println!("maybe_box_pos2={maybe_box_pos2:?}");

                    // let Some(box_pos) = maybe_box_pos.cloned() else {
                    if maybe_box_pos2[0].is_none() && maybe_box_pos2[1].is_none() {
                        n_to_move += 1;
                        continue;
                    };

                    for i in 0..=1 {
                        if let Some(box_pos) = maybe_box_pos2[i].cloned() {
                            boxes_to_move.insert(box_pos);

                            if test_positions.insert(box_pos) {
                                // n_to_move += 1;
                            }
                        }
                    }
                }

                if n_test_positions == n_to_move {
                    break Some(boxes_to_move);
                }
            }
        };

        println!("maybe_boxes_to_move={maybe_boxes_to_move:?}");

        if let Some(boxes_to_move) = maybe_boxes_to_move {
            for b in &boxes_to_move {
                self.boxes.remove(b);
            }
            for b in boxes_to_move {
                self.boxes.insert(b.translate(dx, dy));
            }

            self.robot = next_pos;
        }
    }

    fn print(&self) {
        let max_x = self
            .walls
            .iter()
            .map(|Pos { x, .. }| *x)
            .max()
            .unwrap_or_default();
        let max_y = self
            .walls
            .iter()
            .map(|Pos { y, .. }| *y)
            .max()
            .unwrap_or_default();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let p = Pos::new(x, y);
                let c = if self.robot == p {
                    '@'
                } else if self.boxes.contains(&p) {
                    '['
                } else if self.walls.contains(&p) {
                    '#'
                } else {
                    if self.boxes.contains(&p.translate(-1, 0)) {
                        ']'
                    } else {
                        '.'
                    }
                };
                print!("{c}");
            }
            println!();
        }
    }
}

impl From<&Map> for WiderMap {
    fn from(map: &Map) -> Self {
        let mut robot = map.robot.clone();
        robot.x *= 2;
        let mut boxes = HashSet::new();
        let mut walls = HashSet::new();

        for b in &map.boxes {
            let mut p = b.clone();
            p.x *= 2;
            boxes.insert(p);
        }

        for b in &map.walls {
            let mut p = b.clone();
            p.x *= 2;
            walls.insert(p.clone());
            p.x += 1;
            walls.insert(p);
        }

        Self {
            robot,
            boxes,
            walls,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manh_dist(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
    }

    fn translate(&self, dx: i64, dy: i64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn all() -> impl Iterator<Item = Dir> + 'static {
        [Self::Right, Self::Down, Self::Left, Self::Up].into_iter()
    }
}

fn part1(input: &Input) -> i64 {
    // dbg!(input);
    input.map.print();
    println!("movements: {:?}", input.movements);

    let mut map = input.map.clone();

    for d in &input.movements {
        println!();
        println!("Move {:?}", d);

        map.move_robot(*d);
        map.print();
    }

    map.boxes.iter().map(|Pos { x, y }| 100 * y + x).sum()
}

fn part2(input: &Input) -> i64 {
    // dbg!(input);
    println!("Input map:");
    input.map.print();
    let mut map: WiderMap = (&input.map).into();
    println!("Wider map:");
    map.print();

    for (i, d) in input.movements.iter().enumerate()
    /* .take(195) */
    /* .take(12) */
    {
        println!();
        println!("({i}) Move {:?}", d);

        map.move_robot(*d);
        map.print();
    }

    map.boxes.iter().map(|Pos { x, y }| 100 * y + x).sum()
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines().map_while(Result::ok);

    let mut robot = None;
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();
    for (y, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new(x as i64, y as i64);
            if c == '@' {
                robot = Some(p);
            } else if c == 'O' {
                boxes.insert(p);
            } else if c == '#' {
                walls.insert(p);
            }
        }
    }

    let mut movements = vec![];
    for line in lines {
        for c in line.chars() {
            let dir = match c {
                '^' => Dir::Up,
                '>' => Dir::Right,
                'v' => Dir::Down,
                '<' => Dir::Left,
                _ => unreachable!(),
            };
            movements.push(dir);
        }
    }

    let map = Map {
        robot: robot.unwrap(),
        boxes,
        walls,
    };
    Ok(Input { map, movements })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<";

    const INPUT2: &str = "
        ##########
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

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                .skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT1)?), 2028);
    //     assert_eq!(part1(&as_input(INPUT2)?), 10092);
    //     Ok(())
    // }

    const INPUT3: &str = "
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^";

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(&as_input(INPUT3)?),
            (100 * 1 + 5) + (100 * 2 + 7) + (100 * 3 + 6)
        );
        assert_eq!(part2(&as_input(INPUT2)?), 9021);
        Ok(())
    }
}
