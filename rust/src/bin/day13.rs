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

type Input = Vec<ButtonConfig>;

#[derive(Debug)]
struct ButtonConfig {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

fn part1(input: &Input) -> usize {
    dbg!(input);
    dbg!(input.len());
    if true {
        return 0;
    }

    let mut sum = 0;

    for ButtonConfig {
        button_a,
        button_b,
        prize,
    } in input
    {
        let mut possible_tokens = vec![];
        let a_max = std::cmp::min(prize.0 / button_a.0, prize.1 / button_a.1);
        println!("a_max={a_max}");
        for a in 0..=a_max {
            let x_left = prize.0 - button_a.0 * a;
            let y_left = prize.1 - button_a.1 * a;

            // let b_max = std::cmp::min(x_left / button_b.0, y_left / button_b.1);

            if x_left % button_b.0 != 0
                || y_left % button_b.1 != 0
                || x_left / button_b.0 != y_left / button_b.1
            {
                continue;
            }

            let b = x_left / button_b.0;

            println!("a={a}, b={b}");

            let tokens = a * 3 + b;
            possible_tokens.push(tokens);
        }

        println!("possible_tokens={possible_tokens:?}");

        if let Some(tokens) = possible_tokens.iter().min() {
            sum += tokens;
        }

        // dbg!(lcm(button_a.0 * prize.0, button_a.1 * prize.1));
        dbg!(lcm(button_a.0, button_b.0));
        // dbg!(gcd(button_a.1, button_b.1));
    }

    sum
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if a == b || b == 0 {
            break a;
        } else if a == 0 {
            break b;
        } else if b > a {
            std::mem::swap(&mut a, &mut b);
        }
        a %= b;
    }
}

fn part2(input: &Input) -> usize {
    // dbg!(input);

    let input = input
        .iter()
        .map(
            |ButtonConfig {
                 button_a,
                 button_b,
                 prize,
             }| ButtonConfig {
                button_a: button_a.clone(),
                button_b: button_b.clone(),
                prize: (prize.0 + 10000000000000, prize.1 + 10000000000000),
            },
        )
        .collect::<Vec<_>>();

    let mut sum = 0;

    for ButtonConfig {
        button_a,
        button_b,
        prize,
    } in input
    {
        dbg!(gcd(button_a.0, button_b.0));
        dbg!(gcd(button_a.1, button_b.1));

        let mut possible_tokens = vec![];
        let a_max = std::cmp::min(prize.0 / button_a.0, prize.1 / button_a.1);
        for a in 0..=a_max {
            let x_left = prize.0 - button_a.0 * a;
            let y_left = prize.1 - button_a.1 * a;

            // let b_max = std::cmp::min(x_left / button_b.0, y_left / button_b.1);

            if x_left % button_b.0 != 0
                || y_left % button_b.1 != 0
                || x_left / button_b.0 != y_left / button_b.1
            {
                continue;
            }

            let b = x_left / button_b.0;

            println!("a={a}, b={b}");

            let tokens = a * 3 + b;
            possible_tokens.push(tokens);
        }

        println!("possible_tokens={possible_tokens:?}");

        if let Some(tokens) = possible_tokens.iter().min() {
            sum += tokens;
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
    let mut button_configs = vec![];

    let mut lines = reader.lines().map_while(Result::ok);

    fn parse_button(s: &str) -> Result<(usize, usize)> {
        let (_, s) = s.split_once(':').context("")?;
        let (x, y) = s.split_once(',').context("")?;
        let (_, x) = x.split_once('+').context("")?;
        let (_, y) = y.split_once('+').context("")?;
        let x = x.trim().parse()?;
        let y = y.trim().parse()?;
        Ok((x, y))
    }

    fn parse_prize(s: &str) -> Result<(usize, usize)> {
        let (_, s) = s.split_once(':').context("")?;
        let (x, y) = s.split_once(',').context("")?;
        let (_, x) = x.split_once('=').context("")?;
        let (_, y) = y.split_once('=').context("")?;
        let x = x.trim().parse()?;
        let y = y.trim().parse()?;
        Ok((x, y))
    }

    loop {
        let button_a = lines.next().context("No button a")?;
        let button_b = lines.next().context("No button b")?;
        let prize = lines.next().context("No prize")?;

        button_configs.push(ButtonConfig {
            button_a: parse_button(&button_a)?,
            button_b: parse_button(&button_b)?,
            prize: parse_prize(&prize)?,
        });

        let next = lines.next();
        if next.is_none() {
            break;
        }
    }

    Ok(button_configs)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279";

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
        assert_eq!(part1(&as_input(INPUT)?), 480);
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    //     assert_eq!(part2(&as_input(INPUT)?), 480);
    //     Ok(())
    // }
}
