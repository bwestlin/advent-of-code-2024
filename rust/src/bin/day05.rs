use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};

use utils::measure;

#[derive(Debug)]
struct Input {
    page_ordering_rules: Vec<(usize, usize)>,
    pages_to_produce: Vec<Vec<usize>>,
}

fn both_parts(input: &Input) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut incorrect_updates = vec![];

    let pages_after_by_page = input.page_ordering_rules.iter().fold(
        HashMap::<usize, HashSet<usize>>::new(),
        |mut m, (a, b)| {
            m.entry(*a).or_default().insert(*b);
            m
        },
    );

    let pages_before_by_page = input.page_ordering_rules.iter().fold(
        HashMap::<usize, HashSet<usize>>::new(),
        |mut m, (a, b)| {
            m.entry(*b).or_default().insert(*a);
            m
        },
    );

    'next: for update in &input.pages_to_produce {
        for idx in 0..(update.len() - 1) {
            let a = update[idx];
            let b = update[idx + 1];

            if pages_after_by_page
                .get(&a)
                .map(|m| !m.contains(&b))
                .unwrap_or(false)
                || pages_before_by_page
                    .get(&b)
                    .map(|m| !m.contains(&a))
                    .unwrap_or(false)
            {
                incorrect_updates.push(update);
                continue 'next;
            }
        }

        let middle = update[update.len() / 2];
        p1 += middle;
    }

    for &update in &incorrect_updates {
        let mut update = update.clone();

        'reorder: loop {
            for idx in 0..(update.len() - 1) {
                let a = update[idx];
                let b = update[idx + 1];

                if pages_after_by_page
                    .get(&a)
                    .map(|m| !m.contains(&b))
                    .unwrap_or(false)
                    || pages_before_by_page
                        .get(&b)
                        .map(|m| !m.contains(&a))
                        .unwrap_or(false)
                {
                    update.swap(idx, idx + 1);
                    continue 'reorder;
                }
            }
            break;
        }

        let middle = update[update.len() / 2];
        p2 += middle;
    }

    (p1, p2)
}

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        let (part1, part2) = both_parts(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut page_ordering_rules = vec![];
    let mut pages_to_produce = vec![];

    let mut lines = reader.lines().map_while(Result::ok);

    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        let (a, b) = line.split_once('|').context("")?;
        page_ordering_rules.push((a.parse()?, b.parse()?));
    }

    for line in lines {
        pages_to_produce.push(
            line.split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?,
        );
    }

    Ok(Input {
        page_ordering_rules,
        pages_to_produce,
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
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";

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
        assert_eq!(both_parts(&as_input(INPUT)?).0, 143);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(both_parts(&as_input(INPUT)?).1, 123);
        Ok(())
    }
}
