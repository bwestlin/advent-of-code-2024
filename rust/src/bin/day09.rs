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

type Input = Vec<Block>;

#[derive(Debug, Clone)]
struct Block {
    idx: Option<usize>,
    fill_len: u8,
    len: u8,
}

fn print(blocks: &Vec<Block>) {
    for b in blocks {
        // print!("[");
        for i in 0..b.len {
            if b.idx.is_some() && i < b.fill_len {
                print!("{}", b.idx.unwrap());
            } else {
                print!(".");
            }
        }
        if b.len == 0 {
            // print!("[]");
        }
        // print!("]");
    }
    println!();
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    let mut blocks = input.clone();

    let mut fill_idx = 1;
    // let mut fill_len = 0;
    // let mut fill_block = Block { idx: None, len: 0 };

    print(&blocks);

    // 'block: while fill_idx < blocks.len() {
    for _ in 0.. {
        let len = blocks.len();
        let last_idx = len - 1;

        // Last block is empty
        if blocks[last_idx].fill_len == 0 || blocks[last_idx].idx.is_none() {
            blocks.remove(last_idx);
            continue;
        }

        if fill_idx >= blocks.len() {
            break;
        }

        // Block is full
        if blocks[fill_idx].fill_len == blocks[fill_idx].len {
            fill_idx += 2;
            continue;
        }

        if let Some(idx) = blocks[last_idx].idx {
            // idx from last block is same as current
            if blocks[fill_idx].idx.is_none() || blocks[fill_idx].idx == Some(idx) {
                blocks[fill_idx].idx = Some(idx);
                blocks[fill_idx].fill_len += 1;
                blocks[last_idx].fill_len -= 1;
                blocks[last_idx].len -= 1;
                // fill_len += 1;
            } else {
                let new_block = Block {
                    idx: blocks[fill_idx].idx,
                    fill_len: blocks[fill_idx].fill_len,
                    len: blocks[fill_idx].fill_len,
                };
                // println!("new_block={new_block:?}");
                blocks.insert(fill_idx, new_block);
                fill_idx += 1;
                blocks[fill_idx].len -= blocks[fill_idx].fill_len;
                blocks[fill_idx].fill_len = 0;
                blocks[fill_idx].idx = None;
                continue;
            }
        } else {
            unreachable!()
        }

        // print(&blocks);
    }

    // print(&blocks);

    let mut sum = 0;
    let mut f_idx = 0;

    for b in blocks {
        for _ in 0..b.len {
            sum += b.idx.unwrap() * f_idx;
            f_idx += 1;
        }
    }

    sum
}

fn part2(input: &Input) -> usize {
    // dbg!(input);
    let mut blocks = input.clone();

    let mut curr_idx = blocks.len() - 1;
    let mut curr_file_idx = blocks[curr_idx].idx.unwrap();

    // print(&blocks);

    // 'block: while fill_idx < blocks.len() {
    for _ in 0.. {
        println!("curr_idx={curr_idx}, curr_file_idx={curr_file_idx}");
        if curr_file_idx == 0 {
            break;
        }
        let len = blocks.len();

        let req_len = blocks[curr_idx].fill_len;

        for i in 0..curr_idx {
            if blocks[i].idx.is_none() {
                // Found exact space
                if blocks[i].len == req_len {
                    println!("Found exact, i={i}");
                    blocks[i].idx = Some(curr_file_idx);
                    blocks[i].fill_len = req_len;
                    blocks[curr_idx].idx = None;
                    blocks[curr_idx].fill_len = 0;
                    break;
                }
                // Found more then needed space
                else if blocks[i].len > req_len {
                    println!("Found needed, i={i}");
                    let new_block = Block {
                        idx: blocks[curr_idx].idx,
                        fill_len: blocks[curr_idx].fill_len,
                        len: blocks[curr_idx].fill_len,
                    };
                    // println!("new_block={new_block:?}");
                    blocks.insert(i, new_block);
                    curr_idx += 1;
                    blocks[i + 1].len -= blocks[curr_idx].fill_len;
                    blocks[i + 1].fill_len = 0;
                    blocks[i + 1].idx = None;
                    blocks[curr_idx].idx = None;
                    blocks[curr_idx].fill_len = 0;
                    break;
                }
            }
        }

        // Find next decreasing file id
        for i in (0..curr_idx).rev() {
            if let Some(idx) = blocks[i].idx {
                if idx < curr_file_idx {
                    curr_file_idx = idx;
                    curr_idx = i;
                    break;
                }
            }
        }

        // print(&blocks);
    }

    print(&blocks);

    let mut sum = 0;
    let mut f_idx = 0;

    for b in blocks {
        for i in 0..b.len {
            if let Some(idx) = b.idx {
                if i <= b.fill_len {
                    sum += idx * f_idx;
                }
            }
            f_idx += 1;
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
    let mut blocks = vec![];
    let line = reader
        .lines()
        .map_while(Result::ok)
        .next()
        .context("No line")?;

    for (i, c) in line.chars().enumerate() {
        let len = c as u8 - b'0';
        let idx = if (i % 2) == 0 { Some(i >> 1) } else { None };
        blocks.push(Block {
            idx,
            fill_len: if idx.is_some() { len } else { 0 },
            len,
        });
    }

    Ok(blocks)
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

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
    fn test_part1() -> Result<()> {
        assert_eq!(part1(&as_input(INPUT)?), 1928);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 2858);
        Ok(())
    }
}
