use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp, default};

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Map;

#[derive(Debug, Clone)]
struct Map {
    obstacles: HashSet<Pos>,
    guard_pos: Pos,
    guard_dir: Dir,
    max_x: i64,
    max_y: i64,
}

impl Map {
    fn new(obstacles: HashSet<Pos>, guard_pos: Pos) -> Self {
        let max_x = obstacles.iter().map(|p| p.x).max().unwrap_or_default();
        let max_y = obstacles.iter().map(|p| p.y).max().unwrap_or_default();
        Self {
            obstacles,
            guard_pos,
            guard_dir: Default::default(),
            max_x,
            max_y,
        }
    }

    fn step(mut self) -> Option<Self> {
        let mut next_dir = self.guard_dir.clone();
        let mut next_pos = self.guard_pos.clone();

        for _ in 0..3 {
            let mut test_pos = next_pos.clone();
            match next_dir {
                Dir::Up => test_pos.y -= 1,
                Dir::Right => test_pos.x += 1,
                Dir::Down => test_pos.y += 1,
                Dir::Left => test_pos.x -= 1,
            };

            if test_pos.x < 0
                || test_pos.x > self.max_x
                || test_pos.y < 0
                || test_pos.y > self.max_y
            {
                return None;
            }

            if self.obstacles.contains(&test_pos) {
                next_dir = match next_dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };
                continue;
            }

            next_pos = test_pos;
            break;
        }

        assert_ne!(self.guard_pos, next_pos);

        self.guard_pos = next_pos;
        self.guard_dir = next_dir;

        Some(self)
    }

    fn test_loop(&self, visited: &HashSet<(Pos, Dir)>) -> Option<Pos> {
        // Place obstacle in front of guard
        let mut test_pos = self.guard_pos.clone();
        match self.guard_dir {
            Dir::Up => test_pos.y -= 1,
            Dir::Right => test_pos.x += 1,
            Dir::Down => test_pos.y += 1,
            Dir::Left => test_pos.x -= 1,
        };

        if self.obstacles.contains(&test_pos)
        /*|| visited.iter().any(|(p, _)| p == &test_pos)*/
        {
            return None;
        }

        let mut map = self.clone();
        let mut visited = visited.clone();
        map.obstacles.insert(test_pos);

        for s in 0.. {
            // println!("{s}:");
            // visited.insert((map.guard_pos, map.guard_dir));
            // print(&map, &visited);

            let last_pos = map.guard_pos;
            if let Some(next_map) = map.step() {
                map = next_map;
                let next_visited = (last_pos, map.guard_dir);

                if visited.contains(&next_visited) {
                    return Some(test_pos);
                }
                visited.insert(next_visited);
            } else {
                return None;
            }
        }

        None
    }

    fn print(&self) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let p = Pos::new(x, y);

                let c = if self.guard_pos == p {
                    match self.guard_dir {
                        Dir::Up => '^',
                        Dir::Right => '>',
                        Dir::Down => 'v',
                        Dir::Left => '<',
                    }
                } else if self.obstacles.contains(&p) {
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

fn part1(input: &Input) -> usize {
    let mut map = input.clone();
    let mut visited = HashSet::new();

    for s in 0.. {
        // println!("{s}:");
        // map.print();
        visited.insert(map.guard_pos);
        if let Some(next_map) = map.step() {
            map = next_map;
        } else {
            break;
        }
    }

    visited.len()
}

fn print(map: &Map, visited: &HashSet<(Pos, Dir)>) {
    for y in 0..=map.max_y {
        for x in 0..=map.max_x {
            let p = Pos::new(x, y);

            let c = if map.guard_pos == p {
                match map.guard_dir {
                    Dir::Up => '^',
                    Dir::Right => '>',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                }
            } else if map.obstacles.contains(&p) {
                '#'
            } else {
                visited
                    .iter()
                    .filter(|(x, _)| *x == p)
                    .map(|(_, d)| match d {
                        Dir::Up => '|',
                        Dir::Right => '-',
                        Dir::Down => '|',
                        Dir::Left => '-',
                    })
                    .next()
                    .unwrap_or('.')
            };

            print!("{c}");
        }
        println!();
    }
}

fn print_loop(map: &Map, visited: &HashSet<(Pos, Dir)>, loop_pos: Pos) {
    for y in 0..=map.max_y {
        for x in 0..=map.max_x {
            let p = Pos::new(x, y);

            let c = if loop_pos == p {
                'O'
            } else if map.guard_pos == p {
                match map.guard_dir {
                    Dir::Up => '^',
                    Dir::Right => '>',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                }
            } else if map.obstacles.contains(&p) {
                '#'
            } else {
                visited
                    .iter()
                    .filter(|(x, _)| *x == p)
                    .map(|(_, d)| match d {
                        Dir::Up => '|',
                        Dir::Right => '-',
                        Dir::Down => '|',
                        Dir::Left => '-',
                    })
                    .next()
                    .unwrap_or('.')
            };

            print!("{c}");
        }
        println!();
    }
}

fn print_loops(map: &Map, visited: &HashSet<(Pos, Dir)>, loops_pos: &HashSet<Pos>) {
    for y in 0..=map.max_y {
        for x in 0..=map.max_x {
            let p = Pos::new(x, y);

            let c = if loops_pos.contains(&p) {
                'O'
            } else if map.guard_pos == p {
                match map.guard_dir {
                    Dir::Up => '^',
                    Dir::Right => '>',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                }
            } else if map.obstacles.contains(&p) {
                '#'
            } else {
                visited
                    .iter()
                    .filter(|(x, _)| *x == p)
                    .map(|(_, d)| match d {
                        Dir::Up => '|',
                        Dir::Right => '-',
                        Dir::Down => '|',
                        Dir::Left => '-',
                    })
                    .next()
                    .unwrap_or('.')
            };

            print!("{c}");
        }
        println!();
    }
}

fn part2a(input: &Input) -> usize {
    let mut map = input.clone();
    let mut visited = HashSet::new();
    // visited.insert((map.guard_pos, map.guard_dir));
    let mut candidates = HashSet::new();

    for s in 0.. {
        // println!("{s}:");
        // visited.insert((map.guard_pos, map.guard_dir));
        // print(&map, &visited);

        if let Some(pos) = map.test_loop(&visited) {
            // println!("Loop at {pos:?}");
            // print_loop(&map, &visited, pos);
            candidates.insert(pos);
        }

        let last_pos = map.guard_pos;
        if let Some(next_map) = map.step() {
            map = next_map;
            visited.insert((last_pos, map.guard_dir));
            // if let Some((pos, dir)) = visited.iter().filter(|(p, _)| p == &map.guard_pos).next() {
            //     println!("Crossing at {pos:?} dir: {dir:?}");

            //     let test_dir = match map.guard_dir {
            //         Dir::Up => Dir::Right,
            //         Dir::Right => Dir::Down,
            //         Dir::Down => Dir::Left,
            //         Dir::Left => Dir::Up,
            //     };

            //     if dir == &test_dir {
            //         println!("Candidate!");
            //         candidates.insert(map.guard_pos);
            //     }
            // }
        } else {
            break;
        }
    }

    print_loops(&input, &visited, &candidates);

    candidates
        .into_iter()
        // .filter(|p| p != &input.guard_pos)
        .count()
    // 0
    // 2296 too high
    // 2295 too high
    // 2129 too low
}

fn part2(input: &Input) -> usize {
    let mut map = input.clone();
    let mut visited = HashSet::new();

    for s in 0.. {
        // println!("{s}:");
        // map.print();
        visited.insert(map.guard_pos);
        if let Some(next_map) = map.step() {
            map = next_map;
        } else {
            break;
        }
    }

    let mut candidates = HashSet::new();

    'next: for p in visited {
        let mut map = input.clone();
        map.obstacles.insert(p);

        let mut visited = HashSet::new();

        for s in 0.. {
            // println!("{s}:");
            // map.print();
            if !visited.insert((map.guard_pos, map.guard_dir)) {
                candidates.insert(p);
                continue 'next;
            }
            if let Some(next_map) = map.step() {
                map = next_map;
            } else {
                break;
            }
        }
    }

    candidates.len()
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
    let lines = reader.lines().map_while(Result::ok);

    let mut obstacles = HashSet::new();
    let mut guard_pos = None;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let p = Pos::new(x as i64, y as i64);
                obstacles.insert(p);
            }
            if c == '^' {
                let p = Pos::new(x as i64, y as i64);
                guard_pos = Some(p);
            }
        }
    }

    let guard_pos = guard_pos.context("No guard")?;

    Ok(Map::new(obstacles, guard_pos))
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 41);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 6);
        Ok(())
    }
}
