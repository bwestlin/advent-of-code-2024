use std::cmp::{self, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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

type Input = Maze;

#[derive(Debug)]
struct Maze {
    start: Pos,
    end: Pos,
    walls: HashSet<Pos>,
}

impl Maze {
    fn print(&self) {
        let max_x = self.walls.iter().map(|p| p.x).max().unwrap_or_default();
        let max_y = self.walls.iter().map(|p| p.y).max().unwrap_or_default();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let p = Pos::new(x, y);
                let c = if p == self.start {
                    'S'
                } else if p == self.end {
                    'E'
                } else if self.walls.contains(&p) {
                    '#'
                } else {
                    '.'
                };
                print!("{c}");
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
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

    fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| Pos::new(self.x + dx, self.y + dy))
    }

    fn translate(&self, dx: i64, dy: i64) -> Pos {
        Self::new(self.x + dx, self.y + dy)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn all() -> impl Iterator<Item = Dir> + 'static + Clone {
        [Self::Right, Self::Down, Self::Left, Self::Up].into_iter()
    }

    fn idx(&self) -> usize {
        match self {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }
}

fn part1(input: &Input) -> usize {
    // dbg!(input);
    input.print();

    let mut visited = HashMap::<(Pos, Dir), usize>::new();
    let mut queue = VecDeque::new();
    queue.push_back((input.start.clone(), Dir::Right, 0usize));

    let mut scores = vec![];

    while let Some((pos, dir, score)) = queue.pop_front() {
        // println!("pos={pos:?}, dir={dir:?}, score={score}");
        if pos == input.end {
            scores.push(score);
            continue;
        }

        if let Some(v_score) = visited.get(&(pos, dir)) {
            if *v_score < score {
                continue;
            }
        }

        visited.insert((pos, dir), score);

        // Left
        if let Some(dir) = Dir::all().cycle().skip(dir.idx() + 3).next() {
            // println!("  -90={dir:?}");
            queue.push_front((pos, dir, score + 1000));
        }

        // Forward
        let (dx, dy) = dir.delta();
        let fpos = pos.translate(dx, dy);
        if !input.walls.contains(&fpos) {
            // println!("  forward={pos:?}");
            queue.push_front((fpos, dir, score + 1));
        }

        // Right
        if let Some(dir) = Dir::all().cycle().skip(dir.idx() + 1).next() {
            // println!("  +90={dir:?}");
            queue.push_front((pos, dir, score + 1000));
        }

        // println!("hoho");
        // break;
    }

    scores.sort();
    dbg!(&scores);

    scores[0]
}

fn part2(input: &Input) -> usize {
    // dbg!(input);
    input.print();

    let mut visited = HashMap::<(Pos, Dir), usize>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize, input.start.clone(), Dir::Right, vec![])));

    let mut scores: Vec<(usize, Vec<Pos>)> = vec![];

    while let Some(Reverse((score, pos, dir, mut trail))) = queue.pop() {
        // println!("pos={pos:?}, dir={dir:?}, score={score}");
        if pos == input.end {
            if !scores.is_empty() && scores[0].0 != score {
                break;
            }
            trail.push(pos);
            scores.push((score, trail));
            continue;
        }

        if let Some(v_score) = visited.get(&(pos, dir)) {
            if *v_score < score {
                continue;
            }
        }

        visited.insert((pos, dir), score);

        // Left
        if let Some(dir) = Dir::all().cycle().skip(dir.idx() + 3).next() {
            // println!("  -90={dir:?}");

            queue.push(Reverse((score + 1000, pos, dir, trail.clone())));
        }

        // Right
        if let Some(dir) = Dir::all().cycle().skip(dir.idx() + 1).next() {
            // println!("  +90={dir:?}");
            queue.push(Reverse((score + 1000, pos, dir, trail.clone())));
        }

        // Forward
        let (dx, dy) = dir.delta();
        let fpos = pos.translate(dx, dy);
        if !input.walls.contains(&fpos) {
            // println!("  forward={pos:?}");
            trail.push(pos);
            queue.push(Reverse((score + 1, fpos, dir, trail)));
        }

        // println!("hoho");
        // break;
    }

    // scores.sort();
    // dbg!(&scores);

    let mut on_best_path = HashSet::new();

    for (_, trail) in scores {
        for p in trail {
            on_best_path.insert(p);
        }
    }

    on_best_path.len()
}

// fn both_parts(input: &Input) -> (i32, i32) {
//     dbg!(input);
//     (0, 0)
// }

fn main() -> Result<()> {
    measure(|| {
        let input = input()?;
        // let (part1, part2) = both_parts(&input);
        // println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
        Ok(())
    })
}

fn read_input<R: Read>(reader: BufReader<R>) -> Result<Input> {
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new(x as i64, y as i64);
            if c == 'S' {
                start = Some(p);
            } else if c == 'E' {
                end = Some(p);
            } else if c == '#' {
                walls.insert(p);
            }
        }
    }

    let start = start.context("No start")?;
    let end = end.context("No end")?;

    Ok(Maze { start, end, walls })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############";

    const INPUT2: &str = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################";

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
    //     assert_eq!(part1(&as_input(INPUT1)?), 7036);
    //     assert_eq!(part1(&as_input(INPUT2)?), 11048);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT1)?), 45);
        assert_eq!(part2(&as_input(INPUT2)?), 64);
        Ok(())
    }
}
