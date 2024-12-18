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

type Input = Vec<Pos>;

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

    fn translate(&self, dx: i64, dy: i64) -> Pos {
        Self::new(self.x + dx, self.y + dy)
    }
}

fn print(corrupted: &HashSet<Pos>, visited: &HashMap<Pos, usize>, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            let p = Pos::new(x, y);
            let c = if visited.contains_key(&p) {
                'O'
            } else if corrupted.contains(&p) {
                '#'
            } else {
                '.'
            };
            print!("{c}");
        }
        println!();
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);

    #[cfg(test)]
    let (width, height, n_bytes) = (7, 7, 12);
    #[cfg(not(test))]
    let (width, height, n_bytes) = (71, 71, 1024);

    let target = Pos::new(width - 1, height - 1);

    let mut bytes = input.clone();
    let last_bytes = bytes.split_off(n_bytes);
    let mut corrupted = HashSet::new();
    dbg!(bytes.len());

    for p in bytes {
        corrupted.insert(p);
    }

    let mut visited = HashMap::<Pos, usize>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize, Pos::new(0, 0))));

    let mut res = 0;

    // println!("Initial:");
    // print(&corrupted, &visited, width, height);

    while let Some(Reverse((steps, pos))) = queue.pop() {
        // println!("pos={pos:?}, steps={steps}");

        // print(&corrupted, &visited, width, height);

        if pos == target {
            res = steps;
            break;
        }

        if let Some(v_steps) = visited.get(&pos) {
            if *v_steps <= steps {
                continue;
            }
        }

        visited.insert(pos, steps);

        for adj in pos.adjacent() {
            if adj.x < 0
                || adj.y < 0
                || adj.x >= width
                || adj.y >= height
                || corrupted.contains(&adj)
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

    res
}

fn part2(input: &Input) -> i32 {
    // dbg!(input);

    #[cfg(test)]
    let (width, height, n_bytes) = (7, 7, 12);
    #[cfg(not(test))]
    let (width, height, n_bytes) = (71, 71, 1024);

    let target = Pos::new(width - 1, height - 1);

    let mut bytes = input.clone();
    let last_bytes = bytes.split_off(n_bytes);
    let mut corrupted = HashSet::new();
    dbg!(bytes.len());

    for p in bytes {
        corrupted.insert(p);
    }

    for to_add in last_bytes {
        corrupted.insert(to_add);

        let mut visited = HashMap::<Pos, usize>::new();
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0usize, Pos::new(0, 0))));

        let mut res = 0;

        // println!("Initial:");
        // print(&corrupted, &visited, width, height);

        while let Some(Reverse((steps, pos))) = queue.pop() {
            // println!("pos={pos:?}, steps={steps}");

            // print(&corrupted, &visited, width, height);

            if pos == target {
                res = steps;
                break;
            }

            if let Some(v_steps) = visited.get(&pos) {
                if *v_steps <= steps {
                    continue;
                }
            }

            visited.insert(pos, steps);

            for adj in pos.adjacent() {
                if adj.x < 0
                    || adj.y < 0
                    || adj.x >= width
                    || adj.y >= height
                    || corrupted.contains(&adj)
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

        if res == 0 {
            println!("Found {to_add:?}");
            break;
        }
    }

    0
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

impl FromStr for Pos {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context("No , ?")?;
        Ok(Pos {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Pos>().context("Unable to parse input line"))
        .collect()
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 22);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 1337);
        Ok(())
    }
}
