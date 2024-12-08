use core::num;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::Display;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Vec<Equation>;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Operators {
    Add,
    Multiply,
    Concatenation,
}

fn part1(input: &Input) -> i64 {
    // dbg!(input);
    let max_operators = input.iter().map(|e| e.numbers.len()).max().unwrap() - 1;
    // let min_operators = input.iter().map(|e| e.numbers.len()).min().unwrap() - 1;
    // dbg!(max_operators);
    // dbg!(min_operators);

    // let mut op_combs_by_len = BTreeMap::new();
    // for len in 1..=max_operators {
    //     // println!("len={len}");
    //     let mut op_combs = BTreeSet::new();

    //     for a in 0..=len {
    //         for b in 0..=len {
    //             for o in 0..=len {
    //                 // println!("a={a} b={b} o={o}");
    //                 let comb = (0..a)
    //                     .into_iter()
    //                     .map(|_| Operators::Add)
    //                     .chain((0..b).into_iter().map(|_| Operators::Multiply))
    //                     .cycle()
    //                     .skip(o)
    //                     .take(len)
    //                     .collect::<Vec<_>>();
    //                 if comb.is_empty() {
    //                     continue;
    //                 }
    //                 // println!("comb={comb:?}");
    //                 op_combs.insert(comb);
    //             }
    //         }
    //     }
    //     op_combs_by_len.insert(len, op_combs);
    // }

    let mut op_combs_by_len: BTreeMap<usize, BTreeSet<Vec<Operators>>> = BTreeMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(vec![Operators::Add]);
    queue.push_back(vec![Operators::Multiply]);
    while let Some(next) = queue.pop_front() {
        op_combs_by_len
            .entry(next.len())
            .or_default()
            .insert(next.clone());

        if next.len() >= max_operators {
            continue;
        }

        queue.push_back(
            next.clone()
                .into_iter()
                .chain(Some(Operators::Add).into_iter())
                .collect(),
        );
        queue.push_back(
            next.clone()
                .into_iter()
                .chain(Some(Operators::Multiply).into_iter())
                .collect(),
        );
    }

    // dbg!(&op_combs_by_len);

    let mut sum = 0;

    'next: for Equation {
        test_value,
        numbers,
    } in input
    {
        let mut my = false;
        // println!("-------------------------------------------------------");
        // println!("Equation: test_value={test_value}, numbers={numbers:?}");
        let num_ops = numbers.len() - 1;
        // dbg!(num_ops);
        let op_combs = op_combs_by_len.get(&num_ops).unwrap();
        // println!("op_combs={op_combs:?}");
        // dbg!(op_combs);

        // dbg!(correct.len());
        // dbg!(op_combs.contains(&correct));

        for test_ops in op_combs {
            // println!("test_ops={test_ops:?}");
            // println!("numbers={numbers:?}");
            let mut curr = numbers[0];

            for i in 1..numbers.len() {
                curr = match test_ops[i - 1] {
                    Operators::Add => curr + numbers[i],
                    Operators::Multiply => curr * numbers[i],
                    Operators::Concatenation => unreachable!(),
                }
            }

            // println!("tested {test_value} numbers={numbers:?} test_ops={test_ops:?} curr={curr}");

            if &curr == test_value {
                // println!("Found {test_value} numbers={numbers:?} test_ops={test_ops:?}");
                // println!("{test_value}");
                sum += *test_value;
                my = true;
                // continue 'next;
                break;
            }
        }
    }

    sum
    // 723591083093 too low
    // 723591083093
}

fn part2(input: &Input) -> i64 {
    let max_operators = input.iter().map(|e| e.numbers.len()).max().unwrap() - 1;
    let mut op_combs_by_len: BTreeMap<usize, BTreeSet<Vec<Operators>>> = BTreeMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(vec![Operators::Add]);
    queue.push_back(vec![Operators::Multiply]);
    queue.push_back(vec![Operators::Concatenation]);
    while let Some(next) = queue.pop_front() {
        op_combs_by_len
            .entry(next.len())
            .or_default()
            .insert(next.clone());

        if next.len() >= max_operators {
            continue;
        }

        queue.push_back(
            next.clone()
                .into_iter()
                .chain(Some(Operators::Add).into_iter())
                .collect(),
        );
        queue.push_back(
            next.clone()
                .into_iter()
                .chain(Some(Operators::Multiply).into_iter())
                .collect(),
        );
        queue.push_back(
            next.clone()
                .into_iter()
                .chain(Some(Operators::Concatenation).into_iter())
                .collect(),
        );
    }

    let mut sum = 0;

    'next: for Equation {
        test_value,
        numbers,
    } in input
    {
        let mut my = false;
        let num_ops = numbers.len() - 1;
        let op_combs = op_combs_by_len.get(&num_ops).unwrap();
        for test_ops in op_combs {
            let mut curr = numbers[0];

            for i in 1..numbers.len() {
                curr = match test_ops[i - 1] {
                    Operators::Add => curr + numbers[i],
                    Operators::Multiply => curr * numbers[i],
                    // Operators::Concatenation => format!("{}{}", curr, numbers[i]).parse().unwrap(),
                    Operators::Concatenation => {
                        curr * 10u64.pow(numbers[i].ilog10() + 1) as i64 + numbers[i]
                    }
                }
            }

            if &curr == test_value {
                sum += *test_value;
                my = true;
                break;
            }
        }
    }

    sum
}

// fn both_parts(input: &Input) -> (u64, u64) {
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

impl FromStr for Equation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value, numbers) = s.split_once(':').context("No :")?;
        let test_value = test_value.parse::<i64>()?;
        let numbers = numbers
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()?;
        Ok(Equation {
            test_value,
            numbers,
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.parse::<Equation>()
                .context("Unable to parse input line")
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
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 3749);
    //     Ok(())
    // }

    // const INPUT2: &str = "
    //     25816825: 3 1 838 9 3 6 2 53 95 4 5";

    // #[test]
    // fn test_part1_differs() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT2)?), 25816825);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 11387);
        Ok(())
    }
}
