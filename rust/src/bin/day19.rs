use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

#[derive(Debug)]
struct Input {
    towel_patterns: Vec<String>,
    desired_designs: Vec<String>,
}

fn possible_design(design: &str, towel_patterns: &Vec<String>) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(design);

    while let Some(design_part) = queue.pop_front() {
        if design_part.len() == 0 {
            return true;
        }

        for pt in towel_patterns {
            if design_part.starts_with(pt) {
                let sub_part = &design_part[pt.len()..];
                if visited.contains(sub_part) {
                    continue;
                }
                queue.push_back(sub_part);
                visited.insert(sub_part);
            }
        }
    }

    false
}

fn count_possible_design<'a>(
    design: &'a str,
    towel_patterns: &Vec<String>,
    visited: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    let mut possible = 0;

    for pt in towel_patterns {
        if design.starts_with(pt) {
            let sub_part = &design[pt.len()..];
            if let Some(n) = visited.get(sub_part) {
                possible += n;
            } else {
                let n = count_possible_design(sub_part, towel_patterns, visited);
                visited.insert(sub_part, n);
                possible += n;
            }
        }
    }

    possible
}

fn part1(input: &Input) -> usize {
    let mut cnt = 0;
    for design in &input.desired_designs {
        if possible_design(&design, &input.towel_patterns) {
            cnt += 1;
            // println!("{design} possible!");
        } else {
            // println!("{design} NOT possible!");
        }
    }

    cnt
}

fn part2(input: &Input) -> usize {
    let mut cnt = 0;
    for design in &input.desired_designs {
        let mut visited = HashMap::<&str, usize>::new();
        let n = count_possible_design(&design, &input.towel_patterns, &mut visited);
        // println!("{design} {n}");
        cnt += n;
    }

    cnt
}

// fn both_parts(input: &Input) -> (usize, usize) {
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
    let mut lines = reader.lines().map_while(Result::ok);

    let towel_patterns = lines
        .next()
        .context("No patterns")?
        .split(',')
        .map(|s| s.trim().into())
        .collect();

    let desired_designs = lines.skip(1).map(|s| s.trim().into()).collect();

    Ok(Input {
        towel_patterns,
        desired_designs,
    })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb";

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
        assert_eq!(part1(&as_input(INPUT)?), 6);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 16);
        Ok(())
    }
}
