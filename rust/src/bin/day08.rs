use std::cmp;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Context, Result};
use regex::Regex;

use utils::measure;

type Input = Map;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<Pos, char>,
}

impl Map {
    fn print(&self, antinodes: &HashSet<Pos>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Pos::new(x as i64, y as i64);
                let c = self
                    .antennas
                    .get(&p)
                    .or_else(|| antinodes.contains(&p).then(|| &'#'))
                    .unwrap_or(&'.');
                print!("{c}");
            }
            println!();
        }
    }

    fn is_inside(&self, p: &Pos) -> bool {
        p.x >= 0 && p.x < self.width as i64 && p.y >= 0 && p.y < self.height as i64
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manh_dist(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part1(input: &Input) -> usize {
    dbg!(input);

    let mut antinodes = HashSet::new();

    for (ap, ac) in &input.antennas {
        for (bp, bc) in &input.antennas {
            if ap == bp || ac != bc {
                continue;
            }

            println!("----------------------------");
            println!("ap={ap:?}, bp={bp:?}, ac={ac}, bc={bc}");

            let dx = (ap.x - bp.x).abs();
            let dy = (ap.y - bp.y).abs();

            println!("dx={dx}, dy={dy}");

            let (lm, rm) = if ap.x < bp.x { (ap, bp) } else { (bp, ap) };

            let an1 = Pos::new(lm.x - dx, if lm.y < rm.y { lm.y - dy } else { lm.y + dy });
            let an2 = Pos::new(rm.x + dx, if rm.y < lm.y { rm.y - dy } else { rm.y + dy });

            antinodes.insert(an1);
            antinodes.insert(an2);
        }
    }

    input.print(&antinodes);

    antinodes
        .into_iter()
        .filter(|an| input.is_inside(an))
        .count()
}

fn part2(input: &Input) -> usize {
    dbg!(input);
    let mut antinodes = HashSet::new();

    for (ap, ac) in &input.antennas {
        for (bp, bc) in &input.antennas {
            if ap == bp || ac != bc {
                continue;
            }

            println!("----------------------------");
            println!("ap={ap:?}, bp={bp:?}, ac={ac}, bc={bc}");

            let dx = (ap.x - bp.x).abs();
            let dy = (ap.y - bp.y).abs();

            println!("dx={dx}, dy={dy}");

            let (lm, rm) = if ap.x < bp.x { (ap, bp) } else { (bp, ap) };

            antinodes.insert(*ap);
            antinodes.insert(*bp);

            // Left
            let ty = if lm.y < rm.y { -dy } else { dy };
            let mut an = Pos::new(lm.x - dx, lm.y + ty);
            antinodes.insert(an);

            while input.is_inside(&an) {
                an = Pos::new(an.x - dx, an.y + ty);
                antinodes.insert(an);
            }

            // Right
            let ty = if rm.y < lm.y { -dy } else { dy };
            let mut an = Pos::new(rm.x + dx, rm.y + ty);
            antinodes.insert(an);

            while input.is_inside(&an) {
                an = Pos::new(an.x + dx, an.y + ty);
                antinodes.insert(an);
            }
        }
    }

    input.print(&antinodes);

    antinodes
        .into_iter()
        .filter(|an| input.is_inside(an))
        .count()
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
    let mut width = 0;
    let mut height = 0;
    let mut antennas = HashMap::new();

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        if y > height {
            height = y;
        }

        for (x, c) in line.chars().enumerate() {
            if x > width {
                width = x;
            }

            if c != '.' {
                antennas.insert(Pos::new(x as i64, y as i64), c);
            }
        }
    }
    width += 1;
    height += 1;

    Ok(Map {
        width,
        height,
        antennas,
    })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    // const INPUT: &str = "
    //     ..........
    //     ..........
    //     ..........
    //     ....a.....
    //     ..........
    //     .....a....
    //     ..........
    //     ..........
    //     ..........
    //     ..........";
    // const INPUT2: &str = "
    //     ..........
    //     ..........
    //     ..........
    //     ..........
    //     ..........
    //     .....a....
    //     ..........
    //     ....a.....
    //     ..........
    //     ..........";

    const INPUT2: &str = "
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........";

    const INPUT3: &str = "
        .............
        .............
        .............
        .............
        ....T.T......
        .............
        ......T......
        .............
        .............
        .............
        .............";

    const INPUT: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............";

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
    //     assert_eq!(part1(&as_input(INPUT)?), 14);
    //     // assert_eq!(part1(&as_input(INPUT2)?), 1337);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT)?), 34);
        Ok(())
    }
}
