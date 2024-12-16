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

type Input = Vec<Robot>;

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn steps(&self, n: usize, w: i64, h: i64) -> Robot {
        let mut next = self.clone();

        next.pos.x += self.vel.x * n as i64;
        if next.pos.x < 0 {
            next.pos.x += ((next.pos.x.abs() / w) + 1) * w;
            next.pos.x = next.pos.x % w;
        } else if next.pos.x >= w {
            next.pos.x = next.pos.x % w;
        }

        next.pos.y += self.vel.y * n as i64;
        if next.pos.y < 0 {
            next.pos.y += ((next.pos.y.abs() / h) + 1) * h;
            next.pos.y = next.pos.y % h;
        } else if next.pos.y >= h {
            next.pos.y = next.pos.y % h;
        }

        next
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn print(robots: &Vec<Robot>, w: i64, h: i64) {
    for y in 0..=h {
        for x in 0..=w {
            let p = Vec2::new(x, y);
            let n = robots.iter().filter(|Robot { pos, .. }| pos == &p).count();
            if n > 0 {
                print!("{n}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);

    #[cfg(test)]
    let (w, h) = (11, 7);
    #[cfg(not(test))]
    let (w, h) = (101, 103);

    println!("Initial");
    print(input, w, h);

    let mut robots = vec![];

    for r in input {
        robots.push(r.steps(100, w, h));
    }

    println!("After 100");
    print(&robots, w, h);

    let hx = w / 2;
    let hy = h / 2;
    let quadrants = [
        (0..hx, 0..hy),
        ((hx + 1)..w, 0..hy),
        (0..hx, (hy + 1)..h),
        ((hx + 1)..w, (hy + 1)..h),
    ];

    dbg!(&quadrants);

    let mut q_counts = vec![];

    for (rx, ry) in quadrants {
        let n_robots = robots
            .iter()
            .filter(|Robot { pos, .. }| rx.contains(&pos.x) && ry.contains(&pos.y))
            .count();

        dbg!(n_robots);
        q_counts.push(n_robots);
    }

    q_counts.into_iter().product::<usize>()
}

fn part2(input: &Input) -> usize {
    #[cfg(test)]
    let (w, h) = (11, 7);
    #[cfg(not(test))]
    let (w, h) = (101, 103);

    // println!("Initial");
    // print(input, w, h);

    for n in 0.. {
        let mut robots = vec![];

        for r in input {
            robots.push(r.steps(n, w, h));
        }

        // let hx = w / 2;
        // if (0..h).all(|y| {
        //     robots
        //         .iter()
        //         .any(|Robot { pos, .. }| pos.y == y && pos.x == hx)
        // }) {
        //     println!("After {n}");
        //     print(&robots, w, h);
        // }

        // println!("After 100");
        // print(&robots, w, h);

        let hx = w / 2;
        let hy = h / 2;
        let quadrants = [
            (0..hx, 0..hy),
            ((hx + 1)..w, 0..hy),
            (0..hx, (hy + 1)..h),
            ((hx + 1)..w, (hy + 1)..h),
        ];

        // dbg!(&quadrants);

        // let mut q_counts = vec![];

        // for (rx, ry) in quadrants {
        //     let n_robots = robots
        //         .iter()
        //         .filter(|Robot { pos, .. }| rx.contains(&pos.x) && ry.contains(&pos.y))
        //         .count();

        //     // dbg!(n_robots);
        //     q_counts.push(n_robots);
        // }

        // // q_counts.into_iter().product::<usize>()
        // if q_counts[0] + q_counts[2] == q_counts[1] + q_counts[3] {
        //     println!("After {n}");
        //     print(&robots, w, h);
        // }

        if (n - 22) % 101 == 0 {
            println!("After {n}");
            print(&robots, w, h);
        }

        if n > 308274 {
            break;
        }
    }

    // 22
    // 123 D 101
    // 224 D 101
    // 325
    // 426
    // 527
    // 830
    //

    // 308274 too high
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

impl FromStr for Robot {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').context("")?;
        let (_, p) = p.split_once('=').context("")?;
        let (_, v) = v.split_once('=').context("")?;

        Ok(Robot {
            pos: p.parse()?,
            vel: v.parse()?,
        })
    }
}

impl FromStr for Vec2 {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').context("")?;

        Ok(Vec2 {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Robot>().context("Unable to parse input line"))
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
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3";

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
    // fn test_robot_steps() -> Result<()> {
    //     let robot = "p=2,4 v=2,-3".parse::<Robot>()?;
    //     dbg!(&robot);

    //     for s in 1..=5 {
    //         let r = robot.steps(s, 11, 7);
    //         println!("s={s}, r={r:?}");
    //     }

    //     Ok(())
    // }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 12);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2(&as_input(INPUT)?), 1337);
    //     Ok(())
    // }
}
