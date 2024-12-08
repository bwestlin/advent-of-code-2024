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

type Input = String;

fn parse_mul(s: &str) -> Option<(i32, i32, usize)> {
    let mut len = 6;

    if !s.starts_with("mul(") {
        return None;
    }

    let s = &s[4..];

    let s = s.split(')').next()?;
    let (a, b) = s.split_once(',')?;
    len += a.len() + b.len();
    let a = a.parse().ok()?;
    let b = b.parse().ok()?;

    Some((a, b, len))
}

fn part1(input: &Input) -> i32 {
    let mut sum = 0;
    let mut buf = input.as_str();

    while buf.len() > 0 {
        if let Some((a, b, len)) = parse_mul(buf) {
            buf = &buf[len..];
            sum += a * b;
        } else {
            buf = &buf[1..];
        }
    }

    sum
}

fn part2(input: &Input) -> i32 {
    let mut sum = 0;
    let mut buf = input.as_str();
    let mut enabled = true;

    while buf.len() > 0 {
        if let Some((a, b, len)) = parse_mul(buf) {
            buf = &buf[len..];
            if enabled {
                sum += a * b;
            }
        } else if buf.starts_with("do()") {
            buf = &buf[4..];
            enabled = true;
        } else if buf.starts_with("don't()") {
            buf = &buf[7..];
            enabled = false;
        } else {
            buf = &buf[1..];
        }
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
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line)
        .collect())
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

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

    #[test]
    fn test_parse_mul() -> Result<()> {
        assert_eq!(parse_mul("mul(44,46)"), Some((44, 46, 10)));
        assert_eq!(parse_mul("mul(123,4)"), Some((123, 4, 10)));
        assert_eq!(parse_mul("mul(4*"), None);
        assert_eq!(parse_mul("mul(6,9!"), None);
        assert_eq!(parse_mul("?(12,34)"), None);
        assert_eq!(parse_mul("mul ( 2 , 4 )"), None);
        Ok(())
    }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT1)?), 161);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT2)?), 48);
        Ok(())
    }
}
