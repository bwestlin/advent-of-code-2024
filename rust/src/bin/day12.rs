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

type Input = Map;

#[derive(Debug)]
struct Map {
    plots: HashMap<Pos, char>,
}

impl Map {
    fn print(&self) {
        let max_x = self.plots.keys().map(|p| p.x).max().unwrap_or_default();
        let max_y = self.plots.keys().map(|p| p.y).max().unwrap_or_default();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let p = Pos::new(x, y);
                let c = self.plots.get(&p).unwrap_or(&'?');
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn all() -> impl Iterator<Item = Dir> + 'static {
        [Self::Right, Self::Down, Self::Left, Self::Up].into_iter()
    }
}

fn part1(input: &Input) -> usize {
    dbg!(input);

    input.print();

    let plant_types = input.plots.values().collect::<BTreeSet<_>>();
    dbg!(&plant_types);

    let mut total_price = 0;

    for plant_type in plant_types {
        let area = input.plots.values().filter(|c| *c == plant_type).count();

        println!("----------------------------------------");
        println!("plan_type={plant_type}, area={area}");

        // let mut distinct_areas_n_edges = BTreeSet::<(BTreeSet<Pos>, BTreeSet<(Pos, usize)>)>::new();
        let mut not_yet_accounted = input
            .plots
            .iter()
            .filter(|(_, pt)| *pt == plant_type)
            .map(|(p, _)| p)
            .collect::<HashSet<_>>();

        while !not_yet_accounted.is_empty() {
            let mut distinct_area = BTreeSet::new();
            let mut edges = BTreeSet::new();

            let mut queue = VecDeque::new();
            queue.push_back((*not_yet_accounted.iter().next().unwrap()).clone());

            while let Some(p) = queue.pop_front() {
                if distinct_area.contains(&p) {
                    continue;
                }

                distinct_area.insert(p);
                not_yet_accounted.remove(&p);

                for (i, adj) in p.adjacent().enumerate() {
                    if distinct_area.contains(&adj) {
                        continue;
                    }
                    let pt = input.plots.get(&adj);
                    if pt.is_none() || pt != Some(plant_type) {
                        edges.insert((adj, i));
                        continue;
                    }
                    queue.push_back(adj);
                }
            }

            println!(
                "  area of: {}, with edges: {}",
                distinct_area.len(),
                edges.len()
            );

            total_price += distinct_area.len() * edges.len();
            // distinct_areas_n_edges.insert((distinct_area, edges));
        }

        // println!("distinct_areas_n_edges:\n{distinct_areas_n_edges:#?}");
    }

    total_price
}

fn part2(input: &Input) -> usize {
    // dbg!(input);

    input.print();

    let plant_types = input.plots.values().collect::<BTreeSet<_>>();
    // dbg!(&plant_types);

    let mut total_price = 0;

    for plant_type in plant_types {
        // if *plant_type != 'C' {
        //     continue;
        // }
        let area = input.plots.values().filter(|c| *c == plant_type).count();

        println!("----------------------------------------");
        println!("plan_type={plant_type}, area={area}");

        // let mut distinct_areas_n_edges = BTreeSet::<(BTreeSet<Pos>, BTreeSet<(Pos, usize)>)>::new();
        let mut not_yet_accounted = input
            .plots
            .iter()
            .filter(|(_, pt)| *pt == plant_type)
            .map(|(p, _)| p)
            .collect::<HashSet<_>>();

        while !not_yet_accounted.is_empty() {
            let mut distinct_area = BTreeSet::new();
            let mut edges = BTreeMap::<Pos, BTreeSet<Dir>>::new();

            let mut queue = VecDeque::new();
            queue.push_back((*not_yet_accounted.iter().next().unwrap()).clone());

            while let Some(p) = queue.pop_front() {
                if distinct_area.contains(&p) {
                    continue;
                }

                distinct_area.insert(p);
                not_yet_accounted.remove(&p);

                for (adj, dir) in p.adjacent().zip(Dir::all()) {
                    // if distinct_area.contains(&adj) {
                    //     continue;
                    // }
                    let pt = input.plots.get(&adj);
                    if pt.is_none() || pt != Some(plant_type) {
                        edges.entry(adj).or_default().insert(dir);
                        // edges.insert(adj, dir);
                        continue;
                    }
                    queue.push_back(adj);
                }
            }

            println!(
                "  area of: {}, with edges: {}",
                distinct_area.len(),
                edges.len()
            );

            // println!("edges={edges:#?}");
            let mut sides = 0;

            for dir in Dir::all() {
                let mut this_dir = edges
                    .iter()
                    .filter(|(_, d)| d.contains(&dir))
                    .map(|(p, _)| p)
                    .collect::<Vec<_>>();

                // let distinct_by_axis = this_dir
                //     .iter()
                //     .map(|p| {
                //         if dir == Dir::Left || dir == Dir::Right {
                //             p.x
                //         } else {
                //             p.y
                //         }
                //     })
                //     .collect::<BTreeSet<_>>();

                println!("  dir={dir:?}");

                if dir == Dir::Left || dir == Dir::Right {
                    let mut by_x = BTreeMap::<i64, Vec<&Pos>>::new();
                    for p in this_dir {
                        by_x.entry(p.x).or_default().push(p);
                    }

                    for (_, mut edges) in by_x {
                        edges.sort_by_key(|p| p.y);

                        sides += 1;
                        let mut last_y = edges[0].y;
                        for p in edges.into_iter().skip(1) {
                            if p.y != last_y + 1 {
                                sides += 1;
                            }
                            last_y = p.y;
                        }
                    }

                    // this_dir.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
                } else {
                    let mut by_y = BTreeMap::<i64, Vec<&Pos>>::new();
                    for p in this_dir {
                        by_y.entry(p.y).or_default().push(p);
                    }

                    for (_, mut edges) in by_y {
                        edges.sort_by_key(|p| p.x);

                        sides += 1;
                        let mut last_x = edges[0].x;
                        for p in edges.into_iter().skip(1) {
                            if p.x != last_x + 1 {
                                sides += 1;
                            }
                            last_x = p.x;
                        }
                    }
                }

                // println!("  dir={dir:?} this_dir={this_dir:?}");

                // println!("  for {dir:?}: distinct: {} pos: {this_dir:?}", 0);
                // sides += 0;
            }

            println!("  sides={sides}");

            total_price += distinct_area.len() * sides;
            // distinct_areas_n_edges.insert((distinct_area, edges));
        }

        // println!("distinct_areas_n_edges:\n{distinct_areas_n_edges:#?}");
    }

    total_price
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
    let mut plots = HashMap::new();

    for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
        for (x, c) in line.chars().enumerate() {
            plots.insert(Pos::new(x as i64, y as i64), c);
        }
    }

    Ok(Map { plots })
}

fn input() -> Result<Input> {
    let path = env::args().nth(1).context("No input file given")?;
    read_input(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "
        AAAA
        BBCD
        BBCC
        EEEC";

    const INPUT2: &str = "
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO";

    const INPUT3: &str = "
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE";

    const INPUT4: &str = "
        EEEEE
        EXXXX
        EEEEE
        EXXXX
        EEEEE";

    const INPUT5: &str = "
        AAAAAA
        AAABBA
        AAABBA
        ABBAAA
        ABBAAA
        AAAAAA";

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
    //     assert_eq!(part1(&as_input(INPUT1)?), 140);
    //     assert_eq!(part1(&as_input(INPUT2)?), 772);
    //     assert_eq!(part1(&as_input(INPUT3)?), 1930);
    //     Ok(())
    // }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(&as_input(INPUT1)?), 80);
        assert_eq!(part2(&as_input(INPUT2)?), 436);
        assert_eq!(part2(&as_input(INPUT4)?), 236);
        assert_eq!(part2(&as_input(INPUT5)?), 368);
        assert_eq!(part2(&as_input(INPUT3)?), 1206);
        Ok(())
    }
}
