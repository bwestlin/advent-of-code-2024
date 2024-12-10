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

type Input = Map;

#[derive(Debug)]
struct Map {
    heights: HashMap<Pos, u8>,
}

impl Map {
    fn print(&self) {
        let max_x = self.heights.keys().map(|o| o.x).max().unwrap_or_default();
        let max_y = self.heights.keys().map(|o| o.y).max().unwrap_or_default();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let p = Pos::new(x, y);
                let h = self.heights.get(&p).unwrap();
                print!("{h}");
            }
            println!()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
}

fn part1(input: &Input) -> usize {
    dbg!(input);
    input.print();

    let trailheads = input
        .heights
        .iter()
        .filter(|(p, h)| h == &&0)
        .map(|(p, _)| p)
        .collect::<Vec<_>>();

    dbg!(&trailheads);
    let mut sum = 0;

    for th_p in trailheads {
        println!("----------------------------");
        println!("th_p={th_p:?}");
        let mut reachable = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back((*th_p, *input.heights.get(th_p).unwrap()));

        while let Some((p, h)) = queue.pop_front() {
            if h == 9 {
                println!("  reachable at {p:?}");
                reachable.insert(p);
            }

            for ap in p.adjacent() {
                if let Some(ah) = input.heights.get(&ap) {
                    if ah - h == 1 {
                        queue.push_back((ap, *ah));
                    }
                }
            }
        }

        println!("  num reachable={}", reachable.len());
        sum += reachable.len();
    }

    sum
}

fn part2(input: &Input) -> usize {
    dbg!(input);
    input.print();

    let trailheads = input
        .heights
        .iter()
        .filter(|(p, h)| h == &&0)
        .map(|(p, _)| p)
        .collect::<Vec<_>>();

    dbg!(&trailheads);
    let mut sum = 0;

    for th_p in trailheads {
        println!("----------------------------");
        println!("th_p={th_p:?}");
        let mut reachable = HashSet::new();
        let mut rating = 0;

        let mut queue = VecDeque::new();
        queue.push_back((*th_p, *input.heights.get(th_p).unwrap()));

        while let Some((p, h)) = queue.pop_front() {
            if h == 9 {
                println!("  reachable at {p:?}");
                reachable.insert(p);
                rating += 1;
            }

            for ap in p.adjacent() {
                if let Some(ah) = input.heights.get(&ap) {
                    if ah - h == 1 {
                        queue.push_back((ap, *ah));
                    }
                }
            }
        }

        println!("  rating={}", rating);
        sum += rating;
    }

    sum
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
    let mut heights = HashMap::new();

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new(x as i64, y as i64);
            heights.insert(p, c as u8 - b'0');
        }
    }

    Ok(Map { heights })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 36);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 81);
        Ok(())
    }
}
