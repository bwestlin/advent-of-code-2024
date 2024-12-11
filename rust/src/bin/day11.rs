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

type Input = Arrangement;

#[derive(Debug, Clone)]
struct Arrangement {
    stones: Vec<i64>,
}

impl Arrangement {
    fn blink(&mut self) {
        let mut i = 0;

        while i < self.stones.len() {
            if self.stones[i] == 0 {
                self.stones[i] = 1;
                i += 1;
                continue;
            }

            let digits = (self.stones[i].ilog10() + 1) as i64;
            if digits % 2 == 0 {
                // println!("Splitting {} digits={digits}", self.stones[i]);
                let half_digits = digits / 2;
                let pow = 10_i64.pow(half_digits as u32);
                let a = self.stones[i] / pow;
                let b = self.stones[i] % pow;
                // println!("a={a} b={b}");
                self.stones[i] = a;
                self.stones.insert(i + 1, b);
                i += 2;
                continue;
            }

            self.stones[i] *= 2024;
            i += 1;
        }
    }

    fn print(&self) {
        println!("{:?}", self.stones);
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    let mut arrangement = input.clone();

    // println!("Initial:");
    // arrangement.print();
    // println!();

    for i in 0..25 {
        arrangement.blink();

        // println!("After {} blinks:", i + 1);
        // arrangement.print();
        // println!();
    }

    arrangement.stones.len()
}

fn part2(input: &Input) -> usize {
    // dbg!(input);

    let mut stone_freq = HashMap::<i64, usize>::new();
    for n in &input.stones {
        *stone_freq.entry(*n).or_default() += 1;
    }

    println!("Initial:");
    println!("stone_freq={stone_freq:?}");
    println!();

    for i in 0..75 {
        for (stone, freq) in std::mem::take(&mut stone_freq) {
            if stone == 0 {
                *stone_freq.entry(1).or_default() += freq;
                continue;
            }

            let digits = (stone.ilog10() + 1) as i64;
            if digits % 2 == 0 {
                // println!("Splitting {} digits={digits}", self.stones[i]);
                let half_digits = digits / 2;
                let pow = 10_i64.pow(half_digits as u32);
                let a = stone / pow;
                let b = stone % pow;
                // println!("a={a} b={b}");
                *stone_freq.entry(a).or_default() += freq;
                *stone_freq.entry(b).or_default() += freq;
                continue;
            }

            *stone_freq.entry((stone * 2024)).or_default() += freq;
        }

        println!("After {} blinks:", i + 1);
        println!("stone_freq={stone_freq:?}");
        println!();
    }

    stone_freq.iter().map(|(stone, freq)| freq).sum()
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
    let line = reader
        .lines()
        .map_while(Result::ok)
        .next()
        .context("No line")?;

    let stones = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>())
        .collect::<Result<_, _>>()?;

    Ok(Arrangement { stones })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    fn as_input(s: &str) -> Result<Input> {
        read_input(BufReader::new(
            s.split('\n')
                //.skip(1)
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        ))
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT)?), 55312);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 55312);
        Ok(())
    }
}
