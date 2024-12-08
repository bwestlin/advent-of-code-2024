use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<Report>;

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> (bool, usize) {
        let mut signum = None;
        for (idx, n) in self.levels.windows(2).enumerate() {
            let d = n[0] - n[1];
            if let Some(signum) = signum {
                if signum != d.signum() {
                    return (false, idx);
                }
            } else {
                signum = Some(d.signum());
            };
            if d.abs() < 1 || d.abs() > 3 {
                return (false, idx);
            }
        }

        (true, 0)
    }

    fn is_safe_damped(&self) -> bool {
        let (safe, idx) = self.is_safe();
        if safe {
            return true;
        }
        let levels1 = self
            .levels
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if idx == i { None } else { Some(*n) })
            .collect::<Vec<_>>();

        let levels2 = self
            .levels
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if idx + 1 == i { None } else { Some(*n) })
            .collect::<Vec<_>>();

        let levels3 = self
            .levels
            .iter()
            .enumerate()
            .filter_map(|(i, n)| if idx == i + 1 { None } else { Some(*n) })
            .collect::<Vec<_>>();

        println!(
            "self.levels={:?}, idx={}, levels1={:?}, levels2={:?}, levels3={:?}",
            self.levels, idx, levels1, levels2, levels3
        );
        Self { levels: levels1 }.is_safe().0
            || Self { levels: levels2 }.is_safe().0
            || Self { levels: levels3 }.is_safe().0
    }
}

fn part1(input: &Input) -> usize {
    input.iter().filter(|r| r.is_safe().0).count()
}

fn part2(input: &Input) -> usize {
    input.iter().filter(|r| r.is_safe_damped()).count()
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

impl FromStr for Report {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<_, _>>()?;
        Ok(Report { levels })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<Report>().context("Unable to parse input line"))
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
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

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

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 4);
        Ok(())
    }
}
