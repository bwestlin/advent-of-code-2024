use std::cmp::{self, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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

type Input = Map;

#[derive(Debug, Clone)]
struct Map {
    start: Pos,
    end: Pos,
    walls: HashSet<Pos>,
}

impl Map {
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
                let c = if self.start == p {
                    'S'
                } else if self.end == p {
                    'E'
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

fn part1(map: &Input) -> usize {
    // dbg!(input);
    map.print();

    let width = map
        .walls
        .iter()
        .map(|Pos { x, .. }| *x)
        .max()
        .unwrap_or_default()
        + 1;

    let height = map
        .walls
        .iter()
        .map(|Pos { y, .. }| *y)
        .max()
        .unwrap_or_default()
        + 1;

    let mut visited = HashMap::<Pos, usize>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize, map.start.clone())));

    let mut min_steps = 0;

    while let Some(Reverse((steps, pos))) = queue.pop() {
        if let Some(v_steps) = visited.get(&pos) {
            if *v_steps <= steps {
                continue;
            }
        }

        visited.insert(pos, steps);

        if pos == map.end {
            println!("steps={steps}");
            min_steps = steps;
            break;
        }

        for adj in pos.adjacent() {
            if adj.x < 0
                || adj.y < 0
                || adj.x >= width
                || adj.y >= height
                || map.walls.contains(&adj)
            {
                continue;
            }

            if let Some(v_steps) = visited.get(&adj) {
                if *v_steps <= (steps + 1) {
                    continue;
                }
            }

            queue.push(Reverse((steps + 1, adj)));
        }
    }

    dbg!(&min_steps);
    let mut steps_saved = BTreeMap::<usize, usize>::new();

    for (pos, steps) in &visited {
        for a1 in pos.adjacent() {
            if !map.walls.contains(&a1) {
                continue;
            }

            for a2 in a1.adjacent() {
                if a2 == *pos || map.walls.contains(&a2) {
                    continue;
                }

                if let Some(steps2) = visited.get(&a2) {
                    let this_steps = steps + 2 + (min_steps - steps2);
                    if this_steps < min_steps {
                        let saved = min_steps - this_steps;
                        // if pos.x == 7 && pos.y == 1 {
                        // println!("saved={saved}");
                        // }
                        *steps_saved.entry(saved).or_default() += 1;
                    }
                }
            }
        }
    }

    dbg!(&steps_saved);

    steps_saved
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, cnt)| cnt)
        .sum()
}

fn part2(map: &Input) -> usize {
    // dbg!(input);
    map.print();

    let width = map
        .walls
        .iter()
        .map(|Pos { x, .. }| *x)
        .max()
        .unwrap_or_default()
        + 1;

    let height = map
        .walls
        .iter()
        .map(|Pos { y, .. }| *y)
        .max()
        .unwrap_or_default()
        + 1;

    let mut visited = HashMap::<Pos, usize>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize, map.start.clone())));

    let mut min_steps = 0;

    while let Some(Reverse((steps, pos))) = queue.pop() {
        if let Some(v_steps) = visited.get(&pos) {
            if *v_steps <= steps {
                continue;
            }
        }

        visited.insert(pos, steps);

        if pos == map.end {
            println!("steps={steps}");
            min_steps = steps;
            break;
        }

        for adj in pos.adjacent() {
            if adj.x < 0
                || adj.y < 0
                || adj.x >= width
                || adj.y >= height
                || map.walls.contains(&adj)
            {
                continue;
            }

            if let Some(v_steps) = visited.get(&adj) {
                if *v_steps <= (steps + 1) {
                    continue;
                }
            }

            queue.push(Reverse((steps + 1, adj)));
        }
    }

    dbg!(&min_steps);
    let mut steps_saved = BTreeMap::<usize, usize>::new();

    // Calculate the delta position for 20 steps of cheat
    let mut cheat_deltas = BTreeSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((Pos::new(0, 0), 0));

    while let Some((pos, steps)) = queue.pop_front() {
        if steps > 20 {
            continue;
        }

        if cheat_deltas.contains(&pos) {
            continue;
        }

        for adj in pos.adjacent() {
            if cheat_deltas.contains(&adj) {
                continue;
            }

            queue.push_back((adj, steps + 1));
        }

        cheat_deltas.insert(pos);
    }

    // dbg!(&cheat_deltas);

    // The go through all the visited
    for (pos, steps) in &visited {
        for dp in &cheat_deltas {
            let pos2 = Pos::new(pos.x + dp.x, pos.y + dp.y);
            if pos2.x < 0
                || pos2.y < 0
                || pos2.x >= width
                || pos2.y >= height
                || map.walls.contains(&pos2)
            {
                continue;
            }

            if let Some(steps2) = visited.get(&pos2) {
                let cheat_steps = ((pos.x - pos2.x).abs() + (pos.y - pos2.y).abs()) as usize;

                let this_steps = steps + cheat_steps + (min_steps - steps2);
                if this_steps < min_steps {
                    let saved = min_steps - this_steps;
                    *steps_saved.entry(saved).or_default() += 1;
                }
            }
        }
    }

    dbg!(&steps_saved);

    steps_saved
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, cnt)| cnt)
        .sum()
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

    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new(x as i64, y as i64);
            if c == 'S' {
                start = Some(p);
            } else if c == 'E' {
                end = Some(p);
            } else if c == '#' {
                walls.insert(p);
            }
        }
    }

    let map = Map {
        start: start.unwrap(),
        end: end.unwrap(),
        walls,
    };
    Ok(map)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 1337);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 1337);
        Ok(())
    }
}
