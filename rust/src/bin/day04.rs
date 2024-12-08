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

type Input = Vec<Vec<char>>;

fn part1(input: &Input) -> i32 {
    let mut x_positions = vec![];
    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'X' {
                x_positions.push((x, y));
            }
        }
    }

    // let x_positions = x_positions
    //     .into_iter()
    //     .filter(|(_, y)| y == &1)
    //     .collect::<Vec<_>>();

    // dbg!(&x_positions);

    for (x, y) in x_positions {
        'next: for (dx, dy) in [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
            let mut x = x;
            let mut y = y;
            for c in "MAS".chars() {
                // println!("1) x={x}, y={y}, dx={dx}, dy={dy}, c={c}");
                let Some(nx) = x.checked_add_signed(dx) else {
                    continue 'next;
                };
                let Some(ny) = y.checked_add_signed(dy) else {
                    continue 'next;
                };

                x = nx;
                y = ny;

                // println!("2) x={x}, y={y}, dx={dx}, dy={dy}, c={c}");

                let Some(nc) = input.get(y).and_then(|r| r.get(x)) else {
                    continue 'next;
                };
                if *nc != c {
                    continue 'next;
                }
            }
            println!("Hit!");
            count += 1;
        }
    }

    count
}

fn part2(input: &Input) -> i32 {
    /*
    Variants

    M.S
    .A.
    M.S

    S.M
    .A.
    S.M

    M.M
    .A.
    S.S

    S.S
    .A.
    M.M

    */

    let mut a_positions = vec![];
    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'A' {
                a_positions.push((x, y));
            }
        }
    }

    dbg!(a_positions.len());

    for (x, y) in a_positions {
        'next: for pat in ["MSSM", "SMMS", "MMSS", "SSMM"] {
            for ((dx, dy), c) in [(-1, -1), (1, -1), (1, 1), (-1, 1)]
                .into_iter()
                .zip(pat.chars())
            {
                let Some(nx) = x.checked_add_signed(dx) else {
                    continue 'next;
                };
                let Some(ny) = y.checked_add_signed(dy) else {
                    continue 'next;
                };

                let Some(nc) = input.get(ny).and_then(|r| r.get(nx)) else {
                    continue 'next;
                };
                if *nc != c {
                    continue 'next;
                }
            }
            println!("Hit! x={x}, y={y}");
            count += 1;
        }
    }

    count
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
        .map(|line| line.chars().collect())
        .collect())
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 18);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 9);
        Ok(())
    }
}
