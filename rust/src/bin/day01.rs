use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

type Input = Vec<(i32, i32)>;

fn both_parts(input: Input) -> (i32, i32) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input.into_iter().unzip();

    left.sort();
    right.sort();

    let p1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let right_freq = right
        .into_iter()
        .fold(HashMap::<i32, i32>::new(), |mut m, n| {
            *m.entry(n).or_default() += 1;
            m
        });

    let p2 = left
        .into_iter()
        .map(|l| right_freq.get(&l).unwrap_or(&0) * l)
        .sum();

    (p1, p2)
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = both_parts(input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            Ok((
                parts.next().context("Missing part 1")?.parse()?,
                parts.next().context("Missing part 2")?.parse()?,
            ))
        })
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
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

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
        assert_eq!(both_parts(as_input(INPUT)?).0, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(as_input(INPUT)?).1, 31);
        Ok(())
    }
}
