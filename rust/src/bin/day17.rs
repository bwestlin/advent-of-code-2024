use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Computer;

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    pc: usize,
    program: Vec<u8>,
}

impl Computer {
    fn run(&mut self) -> Vec<u8> {
        let mut output = vec![];
        while self.pc < self.program.len() - 1 {
            if let Some(out) = self.step() {
                output.push(out);
            }
        }
        output
    }

    fn combo(&mut self, operand: u8) -> u64 {
        match operand {
            0 | 1 | 2 | 3 => operand as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }

    fn step(&mut self) -> Option<u8> {
        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        let mut jmp_to = None;
        let mut output = None;

        match opcode {
            // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
            // The denominator is found by raising 2 to the power of the instruction's combo operand.
            // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
            // The result of the division operation is truncated to an integer and then written to the A register.
            0 => {
                let denom = 2_u64.pow(self.combo(operand) as u32);
                self.register_a = self.register_a / denom;
            }

            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
            // then stores the result in register B.
            1 => {
                self.register_b ^= operand as u64;
            }

            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits),
            // then writes that value to the B register.
            2 => {
                self.register_b = self.combo(operand) % 8;
            }

            // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero,
            // it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps,
            // the instruction pointer is not increased by 2 after this instruction.
            3 => {
                if self.register_a != 0 {
                    jmp_to = Some(operand);
                }
            }

            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
            // (For legacy reasons, this instruction reads an operand but ignores it.)
            4 => {
                self.register_b ^= self.register_c;
            }

            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            // (If a program outputs multiple values, they are separated by commas.)
            5 => {
                output = Some((self.combo(operand) % 8) as u8);
            }

            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            // (The numerator is still read from the A register.)
            6 => {
                let denom = 2_u64.pow(self.combo(operand) as u32);
                self.register_b = self.register_a / denom;
            }

            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            // (The numerator is still read from the A register.)
            7 => {
                let denom = 2_u64.pow(self.combo(operand) as u32);
                self.register_c = self.register_a / denom;
            }

            _ => unreachable!(),
        }

        if let Some(pc) = jmp_to {
            self.pc = pc as usize;
        } else {
            self.pc += 2;
        }

        output
    }
}

fn part1(input: &Input) -> Vec<u8> {
    dbg!(input);

    let mut c = input.clone();
    let output = c.run();
    output
}

fn part2(input: &Input) -> u64 {
    // dbg!(input);

    // In test: 0,3, 5,4,3,0
    // Dissassembled:
    //
    //   adv 3 ; reg a / 2 ^ 3 -> reg a
    //   out 4 ; out reg a mod 8
    //   jnz 0 ; jmp 0 if a not 0

    //117440

    // In Input: 2,4,1,1,7,5,0,3,4,7,1,6,5,5,3,0
    // Dissassembled:
    //
    //   bst 4 ; combo register a mod 8 -> reg b
    //   bxl 1 ; bitwise xor reg b - 1 -> reg b
    //   cdv 5 ; reg a / 2 ^ reg b -> reg c
    //   adv 3 ; reg a / 2 ^ 3 -> reg a
    //   bxc 7 ; bitwise reg b - reg c -> reg b
    //   bxl 6 ; bitwise xor reg b - 6 -> reg b
    //   out 5 ; out reg b mod 8
    //   jnz 0 ; jmp 0 if a not 0

    // dbg!(input);

    // let mut c = input.clone();
    // c.register_a = 117440;

    // // dbg!(c.register_a / 2024);

    // let s = format!("{:b}", 117440);
    // println!("s={s}");
    // let chs = s.chars().collect::<Vec<_>>();

    // let mut v = 117440;

    // while v > 0 {
    //     v = v / 2_u64.pow(3);

    //     println!("v={}", v % 8);
    // }

    // // println!("{:b}", 117440);
    // // println!("{:b}", (117440 / (2_u64.pow(3))));

    // let output = c.run();

    dbg!(input);

    // let mut c = input.clone();
    // // Remove the final jnz
    // c.program.truncate(c.program.len() - 2);
    // dbg!(&c);

    // let mut res = 0;
    // // let mut shift = 3;

    // for target in input.program.iter().rev() {
    //     // println!("target={target:03b}");
    //     let mut matches = vec![];
    //     for t in 0..=0b111 {
    //         let mut c = c.clone();
    //         c.register_a = t * 2_u64.pow(3);
    //         // println!("  c.register_a={:06b}", c.register_a);
    //         let out = c.run();
    //         // println!(
    //         //     "  after: c.register_a={:06b} out[0]={:03b} out.len()={}",
    //         //     c.register_a,
    //         //     out[0],
    //         //     out.len()
    //         // );
    //         if out.len() == 1 && out[0] == *target {
    //             println!("  for target {target} found {t:03b}!");
    //             matches.push(t);
    //         }
    //     }

    //     let min = matches.iter().min().unwrap_or(&0);
    //     res <<= 3;
    //     // shift += 1;
    //     res |= min;
    // }

    // res <<= 3;

    // println!("    {:b}", 117440);
    // println!("res={:b}", res);

    let mut res = 0;
    let mut rest_program = input.program.iter().collect::<VecDeque<_>>();
    let mut curr_program = VecDeque::new();
    for _ in 0..9 {
        curr_program.push_back(rest_program.pop_front().unwrap());
    }

    println!("rest_program={:?}", rest_program);
    println!("curr_program={:?}", curr_program);

    for a in 0..=0b111_111_111_111_111_111_111_111_111 {
        let mut c = input.clone();
        c.register_a = a as u64;
        let out = c.run();
        // println!("a={} a={:b} out={:?}", a, a, out);
        if out == curr_program.iter().map(|b| **b).collect::<Vec<_>>() {
            println!("a={} a={:b} out={:?}", a, a, out);
            res = a;
            break;
        }
    }

    println!("res={:b} {}", res, res);
    if res == 0 {
        panic!("hmm");
    }

    println!("-------");
    println!("Part2");
    println!("-------");

    // 'outer: while let Some(t) = rest_program.pop_front() {
    let shift = curr_program.len() * 3;
    // curr_program.push_back(t);
    for _ in 0..7 {
        curr_program.push_back(rest_program.pop_front().unwrap());
    }

    println!("rest_program={:?}", rest_program);
    println!("curr_program={:?}", curr_program);

    println!(
        "shift={}, rest_program={:?}, curr_program={:?}",
        shift, rest_program, curr_program
    );

    for a in 0..=0b111_111_111_111_111_111_111 {
        let mut c = input.clone();
        c.register_a = res | ((a as u64) << shift);
        // println!("c.register_a={:b}", c.register_a);
        let out = c.run();
        // println!("a={} a={:b} out={:?}", a, a, out);
        if out == curr_program.iter().map(|b| **b).collect::<Vec<_>>() {
            println!("a={} a={:b} out={:?}", a, a, out);
            res = res | ((a as u64) << shift);
            break;
        }
    }
    //     unreachable!()
    // }

    res
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        // println!("Part1: {:?}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut lines = reader.lines().map_while(Result::ok);

    let register_a = lines
        .next()
        .context("No register a")?
        .split(':')
        .skip(1)
        .next()
        .context("No value for register a")?
        .trim()
        .parse()
        .context("Invalid register a")?;
    let register_b = lines
        .next()
        .context("No register b")?
        .split(':')
        .skip(1)
        .next()
        .context("No value for register b")?
        .trim()
        .parse()
        .context("Invalid register b")?;
    let register_c = lines
        .next()
        .context("No register c")?
        .split(':')
        .skip(1)
        .next()
        .context("No value for register c")?
        .trim()
        .parse()
        .context("Invalid register c")?;

    let program = lines
        .skip(1)
        .next()
        .context("No program")?
        .split(':')
        .skip(1)
        .next()
        .context("No value for program")?
        .trim()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .context("Invalid program")?;

    Ok(Computer {
        register_a,
        register_b,
        register_c,
        pc: 0,
        program,
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
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0";

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
    // fn test_computer() -> Result<()> {
    //     // If register C contains 9, the program 2,6 would set register B to 1.
    //     let mut c = Computer {
    //         register_a: 0,
    //         register_b: 0,
    //         register_c: 9,
    //         pc: 0,
    //         program: vec![2, 6],
    //     };
    //     c.run();
    //     assert_eq!(c.register_b, 1);

    //     // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    //     let mut c = Computer {
    //         register_a: 10,
    //         register_b: 0,
    //         register_c: 0,
    //         pc: 0,
    //         program: vec![5, 0, 5, 1, 5, 4],
    //     };
    //     let out = c.run();
    //     assert_eq!(out, vec![0, 1, 2]);

    //     // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    //     let mut c = Computer {
    //         register_a: 2024,
    //         register_b: 0,
    //         register_c: 0,
    //         pc: 0,
    //         program: vec![0, 1, 5, 4, 3, 0],
    //     };
    //     let out = c.run();
    //     assert_eq!(out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    //     assert_eq!(c.register_a, 0);

    //     // If register B contains 29, the program 1,7 would set register B to 26.
    //     let mut c = Computer {
    //         register_a: 0,
    //         register_b: 29,
    //         register_c: 0,
    //         pc: 0,
    //         program: vec![1, 7],
    //     };
    //     let out = c.run();
    //     assert_eq!(c.register_b, 26);

    //     // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    //     let mut c = Computer {
    //         register_a: 0,
    //         register_b: 2024,
    //         register_c: 43690,
    //         pc: 0,
    //         program: vec![4, 0],
    //     };
    //     let out = c.run();
    //     assert_eq!(c.register_b, 44354);

    //     Ok(())
    // }

    // #[test]
    // fn test_part1() -> Result<()> {
    //     assert_eq!(part1(&as_input(INPUT)?), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    //     Ok(())
    // }

    const INPUT2: &str = "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0";

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT2)?), 117440);
        Ok(())
    }
}
